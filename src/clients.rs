use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use futures::SinkExt;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Clients {
    clients: Arc<Mutex<Vec<Arc<Mutex<SplitSink<WebSocket, Message>>>>>>,
}

impl Clients {
    pub fn new() -> Self {
        Clients {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_client(&self, sender: SplitSink<WebSocket, Message>) {
        let sender = Arc::new(Mutex::new(sender));

        let mut clients_lock = self.clients.lock().await;
        clients_lock.push(sender);
    }

    pub async fn broadcast_message(&self, message: Message) {
        let mut clients_lock = self.clients.lock().await;
        let mut closed_clients = vec![];

        for (index, client) in clients_lock.iter_mut().enumerate() {
            let mut client_lock = client.lock().await;
            if client_lock.send(message.clone()).await.is_err() {
                closed_clients.push(index);
            }
        }

        for &index in closed_clients.iter().rev() {
            clients_lock.remove(index);
        }
    }
}