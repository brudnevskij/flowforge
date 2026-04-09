use std::sync::Arc;

use api::{ApiState, router};
use app::CreateOrderService;
use axum::Router;
use infra::{PostgresCreateOrderStore, ReadinessChecker};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_subscriber::EnvFilter;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}

async fn connect_to_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

fn build_app(pool: PgPool) -> Router {
    let create_order_store = PostgresCreateOrderStore::new(pool.clone());
    let create_order_service = CreateOrderService::new(create_order_store);

    let readiness_service = ReadinessChecker::new(pool.clone());

    let state = ApiState {
        create_order_service: Arc::new(create_order_service),
        readiness_service: Arc::new(readiness_service),
    };

    router(state)
}

#[tokio::main]
async fn main() {
    init_tracing();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = connect_to_db(&database_url)
        .await
        .expect("failed to initialize database");

    let app = build_app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app).await.expect("server error");
}
