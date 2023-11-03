pub mod api;
pub mod db;
pub mod message;
pub mod room;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use dotenvy::dotenv;

use crate::api::{get_room, get_rooms};

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
    let app = Router::new()
        .route("/rooms", get(get_rooms))
        .route("/room/:room_id", get(get_room));

    // run it
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
