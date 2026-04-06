use chrono::{DateTime, Utc};
use domain::{Order, OrderStatus, OrderStatusParseError};
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct OrderRow {
    id: Uuid,
    customer_reference: String,
    partner_name: String,
    external_reference: Option<String>,
    status: String,
    amount_cents: i64,
    currency: String,
    failure_reason: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    submitted_at: Option<DateTime<Utc>>,
    completed_at: Option<DateTime<Utc>>,
    cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Error)]
pub enum OrderRowParseError {
    #[error("failed to parse order status")]
    OrderStatus(#[from] OrderStatusParseError),
}

impl TryFrom<OrderRow> for Order {
    type Error = OrderRowParseError;

    fn try_from(row: OrderRow) -> Result<Self, Self::Error> {
        Ok(Order {
            id: row.id,
            customer_reference: row.customer_reference,
            partner_name: row.partner_name,
            external_reference: row.external_reference,
            status: OrderStatus::try_from(row.status.as_str())?,
            amount_cents: row.amount_cents,
            currency: row.currency,
            failure_reason: row.failure_reason,
            created_at: row.created_at,
            updated_at: row.updated_at,
            submitted_at: row.submitted_at,
            completed_at: row.completed_at,
            cancelled_at: row.cancelled_at,
        })
    }
}

pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, order: &Order) -> anyhow::Result<Order> {
        let row: OrderRow = sqlx::query_as(
            r#"
            INSERT INTO orders (
                id,
                customer_reference,
                partner_name,
                external_reference,
                status,
                amount_cents,
                currency,
                failure_reason,
                created_at,
                updated_at,
                submitted_at,
                completed_at,
                cancelled_at
            )
            VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9, $10,
                $11, $12, $13
            )
            RETURNING *
            "#,
        )
        .bind(order.id)
        .bind(&order.customer_reference)
        .bind(&order.partner_name)
        .bind(&order.external_reference)
        .bind(order.status.as_str()) // important
        .bind(order.amount_cents)
        .bind(&order.currency)
        .bind(&order.failure_reason)
        .bind(order.created_at)
        .bind(order.updated_at)
        .bind(order.submitted_at)
        .bind(order.completed_at)
        .bind(order.cancelled_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.try_into()?)
    }
}
