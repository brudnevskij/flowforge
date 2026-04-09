use async_trait::async_trait;
use domain::{AuditLog, Order, OrderStatus};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct CreateOrderCommand {
    pub customer_reference: String,
    pub partner_name: String,
    pub amount_cents: i64,
    pub currency: String,
}

#[derive(Debug, Error)]
pub enum CreateOrderValidationError {
    #[error("customer reference must not be empty")]
    EmptyCustomerReference,

    #[error("partner name must not be empty")]
    EmptyPartnerName,

    #[error("amount_cents must be greater than zero")]
    AmountCentsMustBePositive,

    #[error("currency must be exactly 3 uppercase ASCII letters")]
    InvalidCurrencyFormat,
}

impl CreateOrderCommand {
    pub fn validate(&self) -> Result<(), CreateOrderValidationError> {
        if self.customer_reference.trim().is_empty() {
            return Err(CreateOrderValidationError::EmptyCustomerReference);
        }

        if self.partner_name.trim().is_empty() {
            return Err(CreateOrderValidationError::EmptyPartnerName);
        }

        if self.amount_cents <= 0 {
            return Err(CreateOrderValidationError::AmountCentsMustBePositive);
        }

        if self.currency.len() != 3 || !self.currency.chars().all(|c| c.is_ascii_uppercase()) {
            return Err(CreateOrderValidationError::InvalidCurrencyFormat);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CreateOrderStoreError {
    #[error("persistence error: {0}")]
    Persistence(String),
}

#[async_trait]
pub trait CreateOrderStore: Send + Sync {
    async fn create_order_with_audit_log(
        &self,
        order: &Order,
        audit_log: &AuditLog,
    ) -> Result<Order, CreateOrderStoreError>;
}

#[derive(Debug, Error)]
pub enum CreateOrderServiceError {
    #[error(transparent)]
    Validation(#[from] CreateOrderValidationError),

    #[error(transparent)]
    Store(#[from] CreateOrderStoreError),
}

#[async_trait::async_trait]
pub trait CreateOrderUseCase: Send + Sync {
    async fn execute(&self, cmd: CreateOrderCommand) -> Result<Order, CreateOrderServiceError>;
}

pub struct CreateOrderService<S>
where
    S: CreateOrderStore,
{
    order_store: S,
}

impl<S> CreateOrderService<S>
where
    S: CreateOrderStore,
{
    pub fn new(order_store: S) -> Self {
        Self { order_store }
    }
}

#[async_trait::async_trait]
impl<S> CreateOrderUseCase for CreateOrderService<S>
where
    S: CreateOrderStore,
{
    async fn execute(&self, cmd: CreateOrderCommand) -> Result<Order, CreateOrderServiceError> {
        info!(customer_reference = %cmd.customer_reference, partner_name = %cmd.partner_name, amount_cents = %cmd.amount_cents, currency = %cmd.currency, "create order attempt");

        if let Err(err) = cmd.validate() {
            warn!(
            customer_reference = %cmd.customer_reference,
            partner_name = %cmd.partner_name,
            amount_cents = %cmd.amount_cents,
            currency = %cmd.currency,
            "create order validation error"
                );
            return Err(err.into());
        };

        let now = chrono::Utc::now();
        let order_id = uuid::Uuid::new_v4();

        let order = Order {
            id: order_id,
            customer_reference: cmd.customer_reference,
            partner_name: cmd.partner_name,
            external_reference: None,
            status: OrderStatus::Draft,
            amount_cents: cmd.amount_cents,
            currency: cmd.currency,
            failure_reason: None,
            created_at: now,
            updated_at: now,
            submitted_at: None,
            completed_at: None,
            cancelled_at: None,
        };

        let audit_log = AuditLog {
            id: uuid::Uuid::new_v4(),
            order_id,
            event_type: "order_created".to_string(),
            message: "Order created".to_string(),
            metadata: None,
            created_at: now,
        };

        let created_order = self
            .order_store
            .create_order_with_audit_log(&order, &audit_log)
            .await
            .map_err(|err| {
                warn!(
                    order_id = %order_id,
                    error = %err,
                    "create order persistence failure"
                );
                err
            })?;

        info!(
            order_id = %created_order.id,
            customer_reference = %created_order.customer_reference,
            partner_name = %created_order.partner_name,
            status = ?created_order.status,
            "create order succeeded"
        );

        Ok(created_order)
    }
}
