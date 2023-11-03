use axum::Json;

use crate::{message::Message, room::Room};

pub async fn get_rooms_mock() -> Json<Vec<Room>> {
    let rooms = vec![Room {
        id: bson::oid::ObjectId::new(),
        name: "Mock Room".to_string(),
        is_active: true,
        messages: vec![
            Message {
                timestamp: bson::DateTime::now(),
                text: "Hello, world!".to_string(),
                author: bson::oid::ObjectId::new(),
            },
            Message {
                timestamp: bson::DateTime::now(),
                text: "Goodbye, world!".to_string(),
                author: bson::oid::ObjectId::new(),
            },
        ],
    }];

    Json(rooms)
}
