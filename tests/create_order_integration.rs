use crate::common::{TestDatabase, init_app_state};
use api::{CreateOrderRequest, CreateOrderResponse};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sqlx::FromRow;
use tower::ServiceExt;

mod common;

#[derive(Debug, FromRow)]
struct OrderRow {
    id: uuid::Uuid,
    customer_reference: String,
    partner_name: String,
    status: String,
    amount_cents: i64,
    currency: String,
}

#[derive(Debug, FromRow)]
struct AuditLogRow {
    order_id: uuid::Uuid,
    event_type: String,
    message: String,
}

#[tokio::test]
async fn create_order() {
    let db = TestDatabase::new().await;
    let app = init_app_state(db.pool.clone());

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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let created_order: CreateOrderResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(created_order.status, "draft");

    let order_row: OrderRow = sqlx::query_as(
        r#"
        SELECT id, customer_reference, partner_name, status, amount_cents, currency
        FROM orders
        WHERE id = $1
        "#,
    )
    .bind(created_order.id)
    .fetch_one(&db.pool)
    .await
    .unwrap();

    assert_eq!(order_row.id, created_order.id);
    assert_eq!(order_row.customer_reference, "cust-123");
    assert_eq!(order_row.partner_name, "dhl");
    assert_eq!(order_row.status, "draft");
    assert_eq!(order_row.amount_cents, 1500);
    assert_eq!(order_row.currency, "EUR");

    let audit_log_row: AuditLogRow = sqlx::query_as(
        r#"
        SELECT order_id, event_type, message
        FROM audit_logs
        WHERE order_id = $1
        "#,
    )
    .bind(created_order.id)
    .fetch_one(&db.pool)
    .await
    .unwrap();

    assert_eq!(audit_log_row.order_id, created_order.id);
    assert_eq!(audit_log_row.event_type, "order_created");
    assert_eq!(audit_log_row.message, "Order created");
}
