use axum::{
    Router, handler,
    http::StatusCode,
    routing::{get, post},
};

use crate::{
    handlers::{create_order_handler, readiness_handler},
    state::ApiState,
};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/orders", post(create_order_handler))
        .route("/health", get(|| async { StatusCode::OK }))
        .route("/ready", get(readiness_handler))
        .with_state(state)
}
