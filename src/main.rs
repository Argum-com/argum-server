use std::net::SocketAddr;

use axum::{
    extract::{ws::Message, ConnectInfo, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use futures_util::{SinkExt, StreamExt};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::api::{get_room, get_rooms};

mod api;
mod db;
mod message;
mod room;

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("Got a WebSocket connection from: {}", addr);
    ws.on_failed_upgrade(move |error| {
        println!("Failed to upgrade WebSocket connection: {:?}", error);
    })
    .on_upgrade(move |mut socket| async move {
        //send a ping (unsupported by some browsers) just to kick things off and get a response
        if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
            println!("Pinged {addr}...");
        } else {
            println!("Could not send ping {addr}!");
            // no Error here since the only thing we can do is to close the connection.
            // If we can not send messages, there is no way to salvage the statemachine anyway.
            return;
        }
        println!("New WebSocket connection: {}", addr);
        let (mut sender, mut receiver) = socket.split();
        while let Some(Ok(message)) = receiver.next().await {
            println!("Received a message from {}: {:?}", addr, message);
            sender.send(message).await.unwrap();
        }
        println!("WebSocket closed: {}", addr);
    })
}

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
        .route("/ws", get(ws_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // run it
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
