use axum::{Json, extract::State};

use crate::{handlers::OrderState, states::Snapshot};

#[axum::debug_handler]
pub async fn get_orderbook(State(state): State<OrderState>) -> Json<Snapshot> {
    tracing::info!("Fetching orders...");
    let book = state.book.lock().unwrap();

    let snapshot = book.snapshot();
    tracing::info!("Orders Fetched");

    Json(snapshot)
}
