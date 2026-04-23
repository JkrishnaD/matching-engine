use tokio::signal;

pub const ORDERS_QUEUE: &str = "orders:queue";
pub const ORDERS_NEXT_ID: &str = "orders:next_id";
pub const ORDERBOOK_SNAPSHOT: &str = "orderbook:snapshot";
pub const FILLS_CHANNEL: &str = "fills";

pub async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let mut terminate =
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("Unable to terminate");

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Ctrl-C received"),
        _ = terminate.recv() => tracing::info!("Recieved SIGTERM")
    }

    tracing::info!("shutdown signal received, draining connections...");
}
