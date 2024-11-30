use anyhow::{Context, Ok, Result};
use horizon_data_types::Player;
use axum::{Router, routing::get};
use socketioxide::{
    extract::{AckSender, Data, SocketRef}, SocketIo
};
use std::net::SocketAddr;
use horizon_logger::{log_critical, log_error, log_info, log_warn, log_debug};
use crate::LOGGER;

pub(crate) mod config;

struct HorizonServer {
    config: config::ServerConfig,
    threads: Vec<HorizonThread>,
    players: Vec<Player>,
}

impl HorizonServer {
    async fn start() {}
}

struct HorizonThread {
    /// Starting index for this pool's player range
    start_index: usize,
    /// Ending index for this pool's player range
    end_index: usize,
}

fn on_connect(socket: SocketRef, Data(data): Data<serde_json::Value>) {
    //log_info!(LOGGER, "SOCKET NET", "New connection from {}", socket.id);
    socket.emit("auth", &data).ok();
    
    // socket.on("message", move |socket: SocketRef, Data(data): Data<serde_json::Value>| {
    //     //log_debug!(LOGGER,"SOCKET EVENT","Received event");
    //     socket.emit("message-back", &data).ok();
    // });
    
    // socket.on("message-with-ack", move |Data(data): Data<serde_json::Value>, ack: AckSender| {
    //     //log_debug!(LOGGER, "SOCKET EVENT", "Received event with ack");
    //     ack.send(&data).ok();
    // });
}

pub async fn start() -> Result<()> {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    io.ns("/custom", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let address = "0.0.0.0:3000";
    //log_info!(LOGGER, "SOCKET NET", "Starting server on {}", address);
    
    let listener = tokio::net::TcpListener::bind(&address).await.context(format!("Failed to bind to port {}", address))?;
    axum::serve(listener, app).await.context("Failed to serve server")?;

    Ok(())
}