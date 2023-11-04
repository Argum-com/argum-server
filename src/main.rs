use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{ws::Message, ConnectInfo, State, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::api::{get_room, get_rooms};

mod api;
mod db;
mod message;
mod room;

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("Got a WebSocket connection from: {}", addr);
    ws.on_failed_upgrade(move |error| {
        println!("Failed to upgrade WebSocket connection: {:?}", error);
    })
    .on_upgrade(move |socket| async move {
        println!("New WebSocket connection: {}", addr);
        let (mut sender, mut receiver) = socket.split();

        let tx = state.tx.lock().unwrap().clone();

        // Subscribe before sending joined message.
        let mut rx = tx.subscribe();

        // Send joined message to all subscribers.
        let msg = format!("{} joined.", addr);
        tracing::debug!("{}", msg);
        let _ = tx.send(msg);

        // This task will receive broadcast messages and send text message to our client.
        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                // In any websocket error, break loop.
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });

        // This task will receive messages from client and send them to broadcast subscribers.
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                // Add username before message.
                let _ = tx.send(format!("{}: {}", addr, text));
            }
        });

        // If any one of the tasks exit, abort the other.
        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        };
        println!("WebSocket closed: {}", addr);
    })
}

struct AppState {
    tx: Mutex<broadcast::Sender<String>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            tx: Mutex::new(broadcast::channel(10).0),
        }
    }
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
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting server on port {}", port);

    // cors
    let cors = CorsLayer::permissive();

    // state
    let state = Arc::new(AppState::new());

    // build our application with a route
    let app = Router::new()
        .route("/healthcheck", get(|| async { StatusCode::OK }))
        .route("/rooms", get(get_rooms))
        .route("/room/:room_id", get(get_room))
        .route("/ws", get(ws_handler))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // run it
    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        // .serve(app.into_make_service())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
