use std::sync::Arc;

use app::CreateOrderService;
use infra::PostgresCreateOrderStore;

#[derive(Clone)]
pub struct ApiState {
    pub create_order_service: Arc<CreateOrderService<PostgresCreateOrderStore>>,
}
