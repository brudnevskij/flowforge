use crate::{PostgresAuditLogRepository, PostgresOrderRepository};
use app::{CreateOrderStore, CreateOrderStoreError};
use async_trait::async_trait;
use domain::{AuditLog, Order};
use sqlx::PgPool;

pub struct PostgresCreateOrderStore {
    pool: PgPool,
    order_repository: PostgresOrderRepository,
    audit_log_repository: PostgresAuditLogRepository,
}

impl PostgresCreateOrderStore {
    pub fn new(pool: PgPool) -> Self {
        Self {
            order_repository: PostgresOrderRepository::new(pool.clone()),
            audit_log_repository: PostgresAuditLogRepository::new(pool.clone()),
            pool,
        }
    }
}

#[async_trait]
impl CreateOrderStore for PostgresCreateOrderStore {
    async fn create_order_with_audit_log(
        &self,
        order: &Order,
        audit_log: &AuditLog,
    ) -> Result<Order, CreateOrderStoreError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| CreateOrderStoreError::Persistence(e.to_string()))?;

        let created_order = self
            .order_repository
            .insert_tx(&mut tx, order)
            .await
            .map_err(|e| CreateOrderStoreError::Persistence(e.to_string()))?;

        self.audit_log_repository
            .insert_tx(&mut tx, audit_log)
            .await
            .map_err(|e| CreateOrderStoreError::Persistence(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| CreateOrderStoreError::Persistence(e.to_string()))?;

        Ok(created_order)
    }
}
