pub mod behaviour;
pub mod event;
pub mod topic;

pub use behaviour::IpdmBehaviour;
pub use event::IpdmEvent;
pub use topic::TopicManager;

use crate::{config::IpdmConfig, error::Result, types::message::IpdmMessage};
use libp2p::{
    core::muxing::StreamMuxerBox,
    core::transport::Boxed,
    gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity, MessageId},
    identity::Keypair,
    mplex::MplexConfig,
    noise,
    swarm::SwarmBuilder,
    tcp::TokioTcpConfig,
    NetworkBehaviour, PeerId, Transport,
};
use std::collections::HashSet;
use std::time::Duration;
use tokio::sync::mpsc;

const MAX_CONNECTIONS: u32 = 1000;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(60);

pub fn build_transport(
    keypair: &Keypair,
) -> Result<Boxed<(PeerId, StreamMuxerBox)>> {
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(keypair)
        .expect("Signing libp2p-noise static DH keypair failed.");

    Ok(TokioTcpConfig::new()
        .nodelay(true)
        .connection_timeout(CONNECT_TIMEOUT)
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(MplexConfig::new())
        .boxed())
}

pub fn build_gossipsub_config(config: &IpdmConfig) -> libp2p::gossipsub::GossipsubConfig {
    libp2p::gossipsub::GossipsubConfigBuilder::default()
        .heartbeat_interval(Duration::from_millis(config.heartbeat_interval))
        .validation_mode(libp2p::gossipsub::ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .max_transmit_size(config.max_transmit_size)
        .mesh_n_low(config.mesh_n_low)
        .mesh_n_high(config.mesh_n_high)
        .mesh_outbound_min(config.mesh_outbound_min)
        .mesh_n_lazy(config.mesh_n_lazy)
        .build()
        .expect("Valid gossipsub configuration")
}

fn message_id_fn(message: &libp2p::gossipsub::Message) -> MessageId {
    use std::hash::{Hash, Hasher};
    let mut s = std::collections::hash_map::DefaultHasher::new();
    message.data.hash(&mut s);
    MessageId::from(s.finish().to_string())
}

pub struct NetworkEventHandler {
    event_sender: mpsc::UnboundedSender<IpdmEvent>,
    known_peers: HashSet<PeerId>,
}

impl NetworkEventHandler {
    pub fn new(event_sender: mpsc::UnboundedSender<IpdmEvent>) -> Self {
        Self {
            event_sender,
            known_peers: HashSet::new(),
        }
    }

    pub fn handle_gossipsub_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source,
                message_id,
                message,
            } => {
                if let Ok(ipdm_message) = bincode::deserialize::<IpdmMessage>(&message.data) {
                    let _ = self.event_sender.send(IpdmEvent::MessageReceived {
                        peer_id: propagation_source,
                        message: ipdm_message,
                    });
                }
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                log::debug!("Peer {:?} subscribed to {:?}", peer_id, topic);
            }
            GossipsubEvent::Unsubscribed { peer_id, topic } => {
                log::debug!("Peer {:?} unsubscribed from {:?}", peer_id, topic);
            }
            _ => {}
        }
    }

    pub fn handle_peer_discovery(&mut self, peer_id: PeerId) {
        if self.known_peers.insert(peer_id) {
            let _ = self.event_sender.send(IpdmEvent::PeerDiscovered(peer_id));
        }
    }

    pub fn handle_peer_expiration(&mut self, peer_id: PeerId) {
        if self.known_peers.remove(&peer_id) {
            let _ = self.event_sender.send(IpdmEvent::PeerExpired(peer_id));
        }
    }
}