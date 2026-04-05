use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub customer_reference: String,
    pub partner_name: String,
    pub external_reference: Option<String>,
    pub status: OrderStatus,
    pub amount_cents: i64,
    pub currency: String,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Draft,
    Submitted,
    Processing,
    PartnerPending,
    Completed,
    Failed,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Draft => "draft",
            OrderStatus::Submitted => "submitted",
            OrderStatus::Processing => "processing",
            OrderStatus::PartnerPending => "partner_pending",
            OrderStatus::Completed => "completed",
            OrderStatus::Failed => "failed",
            OrderStatus::Cancelled => "cancelled",
        }
    }
}
