use async_trait::async_trait;
use domain::{AuditLog, Order};
use thiserror::Error;

pub struct CreateOrderCommand {
    pub customer_reference: String,
    pub partner_name: String,
    pub amount_cents: i64,
    pub currency: String,
}

pub enum CreateOrderValidationError {
    EmptyCustomerName,
    EmptyPartnerName,
    AmountOfCentsLeZero,
    BadCurrencyFormat,
}

impl CreateOrderCommand {
    pub fn validate(&self) -> Result<(), CreateOrderValidationError> {
        if self.customer_reference.is_empty() {
            return Err(CreateOrderValidationError::EmptyCustomerName);
        }

        if self.partner_name.is_empty() {
            return Err(CreateOrderValidationError::EmptyPartnerName);
        }

        if self.amount_cents <= 0 {
            return Err(CreateOrderValidationError::AmountOfCentsLeZero);
        }

        if !self.currency.is_ascii()
            || self.currency.len() != 3
            || self.currency.to_uppercase() != self.currency
        {
            return Err(CreateOrderValidationError::BadCurrencyFormat);
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

pub struct CreateOrderService<S>
where
    S: CreateOrderStore,
{
    order_store: S,
}
