use app::{CreateOrderCommand, CreateOrderServiceError};
use axum::{Json, extract::State, http::StatusCode};

use crate::{
    dto::{CreateOrderRequest, CreateOrderResponse, ErrorResponse},
    state::ApiState,
};

pub async fn create_order_handler(
    State(state): State<ApiState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<CreateOrderResponse>), (StatusCode, Json<ErrorResponse>)> {
    let command = CreateOrderCommand {
        customer_reference: request.customer_reference,
        partner_name: request.partner_name,
        amount_cents: request.amount_cents,
        currency: request.currency,
    };

    match state.create_order_service.execute(command).await {
        Ok(order) => Ok((StatusCode::CREATED, Json(CreateOrderResponse::from(order)))),

        Err(CreateOrderServiceError::Validation(err)) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: err.to_string(),
            }),
        )),

        Err(CreateOrderServiceError::Store(_err)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "internal server error".to_string(),
            }),
        )),
    }
}
