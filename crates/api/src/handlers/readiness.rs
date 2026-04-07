use axum::{extract::State, http::StatusCode};

use crate::state::ApiState;

pub async fn readiness_handler(State(state): State<ApiState>) -> StatusCode {
    if state.readiness_service.is_ready().await {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
