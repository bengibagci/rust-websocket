use crate::clients::Clients;
use axum::extract::ws::Utf8Bytes;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::Response,
};
use futures::stream::{SplitSink, SplitStream};
use futures::StreamExt;

pub async fn websocket_handler(ws: WebSocketUpgrade, State(clients): State<Clients>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, clients))
}

async fn handle_socket(socket: WebSocket, clients: Clients) {
    let (sender, receiver) = socket.split();

    tokio::spawn(write(sender, clients.clone()));
    tokio::spawn(read(receiver, clients));
}

async fn read(mut receiver: SplitStream<WebSocket>, clients: Clients) {
    let message = Message::Text(Utf8Bytes::from("New user connected!"));
    clients.broadcast_message(message).await;

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            let broadcast_msg = Message::Text(Utf8Bytes::from(format!("User: {}", text)));
            clients.broadcast_message(broadcast_msg).await;
        }
    }
}

async fn write(sender: SplitSink<WebSocket, Message>, clients: Clients) {
    clients.add_client(sender).await;
}