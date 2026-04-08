use app::CreateOrderUseCase;
use infra::ReadinessCheckUseCase;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub create_order_service: Arc<dyn CreateOrderUseCase>,
    pub readiness_service: Arc<dyn ReadinessCheckUseCase>,
}
