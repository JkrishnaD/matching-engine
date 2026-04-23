use std::sync::{Arc, Mutex, atomic::AtomicU64};

use tokio::{net::TcpListener, sync::broadcast};

use crate::{handlers::OrderState, states::Fill};

mod book;
mod handlers;
mod states;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (tx, _rx) = broadcast::channel::<Fill>(1024);

    let state = OrderState {
        book: Arc::new(Mutex::new(book::OrderBook::new())),
        next_id: Arc::new(AtomicU64::new(1)),
        sender: tx,
    };
    let app = handlers::order_routers(state);

    let port = "3000";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("Server is running at port: {}", port);

    axum::serve(listener, app).await.unwrap();
}
