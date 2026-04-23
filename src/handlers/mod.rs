use std::sync::{Arc, Mutex, atomic::AtomicU64};

use axum::{
    Router,
    routing::{get, post},
};
use tokio::sync::broadcast;

use crate::{
    book::OrderBook,
    handlers::{
        orders::{get_orderbook, submit_order},
        ws::ws_handler,
    },
    states::Fill,
};
mod orders;
mod ws;

#[derive(Debug, Clone)]
pub struct OrderState {
    pub book: Arc<Mutex<OrderBook>>,
    pub next_id: Arc<AtomicU64>,
    pub sender: broadcast::Sender<Fill>,
}

pub fn order_routers(state: OrderState) -> Router {
    Router::new()
        .route("/orderbook", get(get_orderbook))
        .route("/orders", post(submit_order))
        .route("/ws", axum::routing::get(ws_handler))
        .with_state(state)
}
