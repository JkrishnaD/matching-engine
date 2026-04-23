use std::sync::{Arc, Mutex, atomic::AtomicU64};

use tokio::net::TcpListener;

use crate::handlers::OrderState;

mod book;
mod handlers;
mod states;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = OrderState {
        book: Arc::new(Mutex::new(book::OrderBook::new())),
        next_id: Arc::new(AtomicU64::new(1)),
    };
    let app = handlers::order_routers(state);

    let port = "3000";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("Server is running at port: {}", port);

    axum::serve(listener, app).await.unwrap();
}
