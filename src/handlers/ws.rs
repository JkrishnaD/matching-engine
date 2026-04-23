use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, Utf8Bytes, WebSocket},
    },
    response::Response,
};

use crate::handlers::OrderState;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<OrderState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: OrderState) {
    let mut rx = state.sender.subscribe();

    while let Ok(fill) = rx.recv().await {
        let json = match serde_json::to_string(&fill) {
            Ok(json) => json,
            Err(_) => continue,
        };

        if socket
            .send(Message::Text(Utf8Bytes::from(json)))
            .await
            .is_err()
        {
            break;
        }
    }
}
