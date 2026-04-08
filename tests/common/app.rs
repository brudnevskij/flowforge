use std::sync::Arc;

use api::{ApiState, router};
use app::CreateOrderService;
use axum::Router;
use infra::{PostgresCreateOrderStore, ReadinessChecker};
use sqlx::PgPool;

pub fn init_app_state(pool: PgPool) -> Router {
    let order_store = PostgresCreateOrderStore::new(pool.clone());
    let create_order_service = CreateOrderService::new(order_store);
    let readiness_checker = ReadinessChecker::new(pool);

    router(ApiState {
        create_order_service: Arc::new(create_order_service),
        readiness_service: Arc::new(readiness_checker),
    })
}
