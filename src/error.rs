use thiserror::Error;

#[derive(Error, Debug)]
pub enum IpdmError {
    #[error("Network error: {0}")]
    Network(#[from] libp2p::swarm::DialError),

    #[error("Gossipsub error: {0}")]
    Gossipsub(#[from] libp2p::gossipsub::error::GossipsubError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Topic error: {0}")]
    Topic(String),
}

pub type Result<T> = std::result::Result<T, IpdmError>;