use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub id: ObjectId,
    pub name: String,
    pub is_active: bool,
    pub messages: Vec<Message>,
}
