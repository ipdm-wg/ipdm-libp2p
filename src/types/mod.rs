//! Types module exports all the message and data structures used in the IPDM network

pub mod message;

pub use message::{
    AccountMessage, 
    IpdmMessage, 
    SlotMessage, 
    TransactionMessage
};

use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub subscribed_topics: Vec<String>,
    pub last_seen: u64,
    pub metadata: PeerMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerMetadata {
    pub version: String,
    pub features: Vec<String>,
    pub node_type: NodeType,
    pub stats: NodeStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Validator,
    RpcNode,
    LightClient,
    IndexerNode,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeStats {
    pub messages_processed: u64,
    pub messages_published: u64,
    pub peer_count: u32,
    pub latency: LatencyStats,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LatencyStats {
    pub average: f64,
    pub min: u64,
    pub max: u64,
    pub p95: u64,
}

#[derive(Debug, Clone)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    NeedsVerification,
}

#[derive(Debug, Clone, Default)]
pub struct NetworkMetrics {
    pub peers: HashMap<PeerId, PeerMetrics>,
    pub topics: HashMap<String, TopicMetrics>,
    pub global: GlobalMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct PeerMetrics {
    pub messages_received: u64,
    pub messages_sent: u64,
    pub invalid_messages: u64,
    pub uptime: u64,
}

#[derive(Debug, Clone, Default)]
pub struct TopicMetrics {
    pub peer_count: u32,
    pub messages_per_second: f64,
    pub total_messages: u64,
    pub message_sizes: SizeStats,
}

#[derive(Debug, Clone, Default)]
pub struct GlobalMetrics {
    pub total_peers: u64,
    pub total_messages: u64,
    pub bandwidth: BandwidthMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct BandwidthMetrics {
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub current_bandwidth: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SizeStats {
    pub average: f64,
    pub min: u64,
    pub max: u64,
    pub p95: u64,
}

impl PeerInfo {
    pub fn new(peer_id: PeerId, node_type: NodeType) -> Self {
        Self {
            peer_id,
            subscribed_topics: Vec::new(),
            last_seen: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata: PeerMetadata {
                version: env!("CARGO_PKG_VERSION").to_string(),
                features: vec!["basic".to_string()],
                node_type,
                stats: NodeStats::default(),
            },
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}