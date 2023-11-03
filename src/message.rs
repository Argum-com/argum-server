use bson::{DateTime, oid::ObjectId};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub timestamp: DateTime,
    pub text: String,
    pub author: ObjectId,
}