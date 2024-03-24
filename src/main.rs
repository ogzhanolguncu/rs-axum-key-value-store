pub mod handlers;
pub mod state;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use tracing_subscriber;

use crate::{
    handlers::{kv_capacity, kv_del, kv_flush, kv_get, kv_get_keys, kv_set},
    state::SharedState,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .init();

    let shared_state = SharedState::default();

    let app = Router::new()
        .route("/:key", get(kv_get).post(kv_set).delete(kv_del))
        .route("/flush", post(kv_flush))
        .route("/capacity", get(kv_capacity))
        .route("/keys", get(kv_get_keys))
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
