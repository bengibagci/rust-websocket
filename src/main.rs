mod handler;
mod clients;

use crate::clients::Clients;
use crate::handler::websocket_handler;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let clients = Clients::new();

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(clients.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("WebSocket server running on ws://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
