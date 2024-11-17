use libp2p::gossipsub::GossipsubConfigBuilder;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpdmConfig {
    pub listen_address: String,
    pub bootstrap_peers: Vec<String>,
    pub heartbeat_interval: u64,
    pub max_transmit_size: usize,
    pub mesh_n_low: usize,
    pub mesh_n_high: usize,
    pub mesh_outbound_min: usize,
    pub mesh_n_lazy: usize,
}

impl Default for IpdmConfig {
    fn default() -> Self {
        Self {
            listen_address: "/ip4/0.0.0.0/tcp/0".to_string(),
            bootstrap_peers: vec![],
            heartbeat_interval: 1000,
            max_transmit_size: 65536,
            mesh_n_low: 4,
            mesh_n_high: 12,
            mesh_outbound_min: 2,
            mesh_n_lazy: 4,
        }
    }
}

impl IpdmConfig {
    pub fn to_gossipsub_config(&self) -> libp2p::gossipsub::GossipsubConfig {
        GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_millis(self.heartbeat_interval))
            .max_transmit_size(self.max_transmit_size)
            .mesh_n_low(self.mesh_n_low)
            .mesh_n_high(self.mesh_n_high)
            .mesh_outbound_min(self.mesh_outbound_min)
            .mesh_n_lazy(self.mesh_n_lazy)
            .build()
            .expect("Valid gossipsub config")
    }
}