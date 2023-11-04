use std::net::SocketAddr;

use axum::{http::StatusCode, routing::get, Router};
use dotenvy::dotenv;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::api::{get_room, get_rooms};

mod api;
mod db;
mod message;
mod room;

#[tokio::main]
async fn main() {
    // env vars are loaded from .env file
    dotenv().ok();
    let port = std::env::var("PORT")
        .unwrap_or(String::from("3000")) // default port is 3000
        .parse::<u16>()
        .expect("PORT env var is not a valid port");

    // enable logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting server on port {}", port);

    // cors
    let cors = CorsLayer::permissive();

    // build our application with a route
    let app = Router::new()
        .route("/healthcheck", get(|| async { StatusCode::OK }))
        .route("/rooms", get(get_rooms))
        .route("/room/:room_id", get(get_room))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // run it
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
