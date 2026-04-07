use app::{CreateOrderCommand, CreateOrderService, CreateOrderServiceError, CreateOrderStore};
use domain::Order;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub create_order_service: Arc<dyn CreateOrderUseCase>,
    pub readiness_service: Arc<dyn ReadinessCheckUseCase>,
}

#[async_trait::async_trait]
pub trait CreateOrderUseCase: Send + Sync {
    async fn execute(&self, cmd: CreateOrderCommand) -> Result<Order, CreateOrderServiceError>;
}

#[async_trait::async_trait]
pub trait ReadinessCheckUseCase: Send + Sync {
    async fn is_ready(&self) -> bool;
}
