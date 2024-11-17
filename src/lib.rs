pub mod config;
pub mod error;
pub mod network;
pub mod types;

use config::IpdmConfig;
use error::Result;
use libp2p::{
    core::transport::MemoryTransport,
    gossipsub::TopicHash,
    identity::Keypair,
    noise,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};
use network::{behaviour::IpdmBehaviour, topic::TopicManager};
use types::message::IpdmMessage;

pub struct IpdmNode {
    swarm: Swarm<IpdmBehaviour>,
    topic_manager: TopicManager,
}

impl IpdmNode {
    pub async fn new(config: IpdmConfig) -> Result<Self> {
        let id_keys = Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        
        // Create transport
        let transport = tcp::TokioTcpTransport::new(tcp::Config::default().nodelay(true))
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(&id_keys).unwrap())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();

        let behaviour = IpdmBehaviour::new(&config).await?;
        let mut swarm = Swarm::with_tokio_executor(transport, behaviour, peer_id);

        // Listen on the configured address
        swarm.listen_on(config.listen_address.parse()?)?;

        Ok(Self {
            swarm,
            topic_manager: TopicManager::new(),
        })
    }

    pub fn subscribe(&mut self, topic: String) -> Result<()> {
        let topic_hash = self.topic_manager.subscribe(topic.clone());
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic_hash)?;
        Ok(())
    }

    pub fn publish(&mut self, topic: &str, message: IpdmMessage) -> Result<()> {
        if let Some(topic_hash) = self.topic_manager.topics.get(topic) {
            self.swarm.behaviour_mut().publish(topic_hash, message)?;
        }
        Ok(())
    }

    pub async fn run(&mut self) {
        loop {
            if let SwarmEvent::Behaviour(event) = self.swarm.next().await {
                // Handle events
                match event {
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::message::{AccountMessage, IpdmMessage};

    #[tokio::test]
    async fn test_node_creation() -> Result<()> {
        let config = IpdmConfig::default();
        let mut node = IpdmNode::new(config).await?;
        
        // Subscribe to an account topic
        let topic = TopicManager::get_account_topic("program123");
        node.subscribe(topic.clone())?;

        // Create and publish a test message
        let message = IpdmMessage::Account(AccountMessage {
            pubkey: "test".to_string(),
            owner: "owner".to_string(),
            lamports: 100,
            slot: 1,
            data: vec![],
            executable: false,
            rent_epoch: 0,
            write_version: 0,
        });

        node.publish(&topic, message)?;
        Ok(())
    }
}