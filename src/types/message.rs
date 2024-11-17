use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpdmMessage {
    Account(AccountMessage),
    Slot(SlotMessage),
    Transaction(TransactionMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMessage {
    pub pubkey: String,
    pub owner: String,
    pub lamports: u64,
    pub slot: u64,
    pub data: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: u64,
    pub write_version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotMessage {
    pub slot: u64,
    pub parent: Option<u64>,
    pub status: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    pub signature: String,
    pub slot: u64,
    pub success: bool,
    pub timestamp: u64,
}