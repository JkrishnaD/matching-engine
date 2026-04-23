use std::sync::{Arc, Mutex, atomic::AtomicU64};

use axum::{Router, routing::get};

use crate::{book::OrderBook, handlers::orders::get_orderbook};
mod orders;

#[derive(Debug, Clone)]
pub struct OrderState {
    pub book: Arc<Mutex<OrderBook>>,
    pub next_id: Arc<AtomicU64>,
}

pub fn order_routers(state: OrderState) -> Router {
    Router::new()
        .route("/orderbook", get(get_orderbook))
        .with_state(state)
}
