use libp2p::{
    gossipsub::{Gossipsub, GossipsubEvent, MessageId, TopicHash},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};

use crate::{config::IpdmConfig, types::message::IpdmMessage};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(NetworkBehaviour)]
pub struct IpdmBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
}

impl IpdmBehaviour {
    pub async fn new(config: &IpdmConfig) -> libp2p::swarm::Result<Self> {
        let gossipsub_config = config.to_gossipsub_config();
        let gossipsub = Gossipsub::new(
            libp2p::gossipsub::MessageAuthenticity::Signed(libp2p::identity::Keypair::generate_ed25519()),
            gossipsub_config,
        )?;

        let mdns = Mdns::new(MdnsConfig::default()).await?;

        Ok(Self { gossipsub, mdns })
    }

    pub fn publish(&mut self, topic: &TopicHash, message: IpdmMessage) -> Result<MessageId, libp2p::gossipsub::error::PublishError> {
        let data = bincode::serialize(&message).expect("Serialization never fails");
        self.gossipsub.publish(topic.clone(), data)
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for IpdmBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message {
            propagation_source: peer_id,
            message_id,
            message,
        } = event {
            if let Ok(msg) = bincode::deserialize::<IpdmMessage>(&message.data) {
                // Handle the message based on its type
                match msg {
                    IpdmMessage::Account(account) => {
                        log::info!("Received account update for {}", account.pubkey);
                    }
                    IpdmMessage::Slot(slot) => {
                        log::info!("Received slot update for slot {}", slot.slot);
                    }
                    IpdmMessage::Transaction(tx) => {
                        log::info!("Received transaction update {}", tx.signature);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for IpdmBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(peers) => {
                for (peer_id, _multiaddr) in peers {
                    log::info!("Discovered peer through mDNS: {}", peer_id);
                }
            }
            MdnsEvent::Expired(peers) => {
                for (peer_id, _multiaddr) in peers {
                    log::info!("mDNS peer expired: {}", peer_id);
                }
            }
        }
    }
}