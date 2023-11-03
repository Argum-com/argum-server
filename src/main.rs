pub mod room;
pub mod message;

use std::net::SocketAddr;

use axum::{routing::get, Router, Json};
use dotenvy::dotenv;
use room::Room;

use crate::message::Message;

async fn get_rooms() -> Json<Vec<Room>> {
    let rooms = vec![
        Room {
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
        }
    ];

    Json(rooms)
}

#[tokio::main]
async fn main() {
    // env vars are loaded from .env file
    dotenv().ok();
    let port = std::env::var("PORT")
        .unwrap_or(String::from("3000")) // default port is 3000
        .parse::<u16>()
        .expect("PORT env var is not a valid port");

    // build our application with a route
    println!("Listening on port {}", port);
    let app = Router::new().route("/", get(get_rooms));

    // run it
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
