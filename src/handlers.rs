use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use tracing::info;

use crate::state::SharedState;

// The kv_set function to insert data into the store
pub async fn kv_set(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    bytes: Bytes,
) -> StatusCode {
    info!(%key, "Trying to SET value");

    let mut app_state = state.write().await;

    app_state.store.insert(key, bytes);
    StatusCode::OK
}

pub async fn kv_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Bytes, StatusCode> {
    info!(%key, "Trying to GET key-value");

    let app_state = state.read().await;

    if let Some(value) = app_state.store.get(&key) {
        // Attempt to deserialize the Bytes assumed to be a UTF-8 encoded JSON string
        Ok(value.clone())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn kv_del(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Value>, StatusCode> {
    info!(%key, "Trying to DELETE key-value");

    let mut app_state = state.write().await;
    let ok_message = format!("{key} removed successfully");

    if let Some(_) = app_state.store.remove(&key) {
        Ok(Json(json!({"message": ok_message})))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn kv_flush(State(state): State<SharedState>) -> Json<Value> {
    info!("Flushing the store");

    let mut app_state = state.write().await;

    app_state.store.clear();

    Json(json!({"message": "Store flushed successfully"}))
}

pub async fn kv_capacity(State(state): State<SharedState>) -> String {
    info!("Returning the capacity of store");

    let app_state = state.read().await;
    app_state.store.len().to_string()
}

pub async fn kv_get_keys(State(state): State<SharedState>) -> String {
    let app_state = state.read().await;

    app_state
        .store
        .keys()
        .map(|key| key.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}
