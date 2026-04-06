use axum::{Router, routing::post};

use crate::{handlers::create_order_handler, state::ApiState};

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/orders", post(create_order_handler))
        .with_state(state)
}
