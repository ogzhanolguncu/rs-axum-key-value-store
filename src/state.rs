use std::{collections::HashMap, sync::Arc};

use axum::body::Bytes;

pub type SharedState = Arc<tokio::sync::RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    pub store: HashMap<String, Bytes>,
}
