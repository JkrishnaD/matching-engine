use axum::{
    Router,
    routing::{get, post},
};
use redis::aio::ConnectionManager;
use tokio::sync::broadcast;

use crate::{
    handlers::{
        orders::{get_orderbook, submit_order},
        ws::ws_handler,
    },
    states::Fill,
};
mod orders;
mod ws;

#[derive(Clone)]
pub struct OrderState {
    pub sender: broadcast::Sender<Fill>,
    pub redis: ConnectionManager,
}

pub fn order_routers(state: OrderState) -> Router {
    Router::new()
        .route("/orderbook", get(get_orderbook))
        .route("/orders", post(submit_order))
        .route("/ws", axum::routing::get(ws_handler))
        .with_state(state)
}
