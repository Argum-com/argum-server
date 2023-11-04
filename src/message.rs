use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: i64,
    pub text: String,
    pub author: ObjectId,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            timestamp: DateTime::now().timestamp_millis(),
            text: Default::default(),
            author: ObjectId::new(),
        }
    }
}
