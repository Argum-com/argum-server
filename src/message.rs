use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub timestamp: DateTime,
    pub text: String,
    pub author: ObjectId,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            timestamp: DateTime::now(),
            text: Default::default(),
            author: ObjectId::new(),
        }
    }
}
