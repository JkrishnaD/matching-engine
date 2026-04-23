use axum::{Json, extract::State};
use redis::AsyncTypedCommands;
use serde::Serialize;

use crate::{
    handlers::OrderState,
    states::{Fill, Order, OrderRequest, Snapshot},
    utils::{ORDERBOOK_SNAPSHOT, ORDERS_NEXT_ID, ORDERS_QUEUE},
};

pub async fn get_orderbook(State(state): State<OrderState>) -> Json<Snapshot> {
    let mut conn = state.redis.clone();

    let raw: Option<String> = conn.get(ORDERBOOK_SNAPSHOT).await.unwrap_or(None);
    let snapshot = raw
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(Snapshot {
            bids: vec![],
            asks: vec![],
        });

    Json(snapshot)
}

#[derive(Serialize)]
pub struct PostOrderResponse {
    pub id: u64,
    pub fills: Vec<Fill>,
}

pub async fn submit_order(
    State(state): State<OrderState>,
    Json(req): Json<OrderRequest>,
) -> Json<PostOrderResponse> {
    let mut conn = state.redis.clone();

    // assign a unique id to the order
    let id = conn.incr(ORDERS_NEXT_ID, 1).await.unwrap() as u64;

    // building the order
    let order = Order {
        id,
        side: req.side,
        price: req.price,
        qty: req.qty,
    };

    let payload = serde_json::to_string(&order).unwrap();
    conn.lpush(ORDERS_QUEUE, payload).await.unwrap();

    tracing::info!("order {} queued", id);
    // returning the response
    Json(PostOrderResponse {
        id,
        fills: Vec::new(),
    })
}
