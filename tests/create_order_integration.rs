use api::CreateOrderRequest;
use axum::{body::Body, http::Request};
use reqwest::StatusCode;
use tower::ServiceExt;

use crate::common::{TestDatabase, init_app_state};

mod common;

#[tokio::test]
async fn create_order() {
    let db = TestDatabase::new().await;
    let app = init_app_state(db.pool);

    let dto = CreateOrderRequest {
        customer_reference: "cust-123".to_string(),
        partner_name: "dhl".to_string(),
        amount_cents: 1500,
        currency: "EUR".to_string(),
    };
    let request = Request::builder()
        .method("POST")
        .uri("/orders")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&dto).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    db.container.stop().await.expect("container should stop");
}
