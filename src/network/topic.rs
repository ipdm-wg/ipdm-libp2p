use libp2p::gossipsub::TopicHash;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TopicManager {
    topics: HashMap<String, TopicHash>,
}

impl TopicManager {
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
        }
    }

    pub fn get_account_topic(program_id: &str) -> String {
        format!("accounts/{}", program_id)
    }

    pub fn get_slot_topic() -> String {
        "slots/updates".to_string()
    }

    pub fn get_transaction_topic(block_range: &str) -> String {
        format!("transactions/{}", block_range)
    }

    pub fn subscribe(&mut self, topic_str: String) -> TopicHash {
        let topic = TopicHash::from_raw(topic_str.clone());
        self.topics.insert(topic_str, topic.clone());
        topic
    }

    pub fn unsubscribe(&mut self, topic_str: &str) -> Option<TopicHash> {
        self.topics.remove(topic_str)
    }
}