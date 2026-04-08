use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderRequest {
    pub customer_reference: String,
    pub partner_name: String,
    pub amount_cents: i64,
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderResponse {
    pub id: Uuid,
    pub customer_reference: String,
    pub partner_name: String,
    pub external_reference: Option<String>,
    pub status: String,
    pub amount_cents: i64,
    pub currency: String,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

impl From<domain::Order> for CreateOrderResponse {
    fn from(order: domain::Order) -> Self {
        Self {
            id: order.id,
            customer_reference: order.customer_reference,
            partner_name: order.partner_name,
            external_reference: order.external_reference,
            status: order.status.as_str().to_string(),
            amount_cents: order.amount_cents,
            currency: order.currency,
            failure_reason: order.failure_reason,
            created_at: order.created_at,
            updated_at: order.updated_at,
            submitted_at: order.submitted_at,
            completed_at: order.completed_at,
            cancelled_at: order.cancelled_at,
        }
    }
}
