use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::message::Message;

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub id: ObjectId,
    pub name: String,
    pub is_active: bool,
    pub messages: Vec<Message>,
}