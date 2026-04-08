use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;

pub struct TestDatabase {
    pub pool: PgPool,
    pub container: ContainerAsync<Postgres>,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let container = Postgres::default()
            .start()
            .await
            .expect("failed to start postgres container");

        let host = container.get_host().await.expect("failed to get host");

        let port = container
            .get_host_port_ipv4(5432)
            .await
            .expect("failed to get postgres port");

        let database_url = format!("postgres://postgres:postgres@{}:{}/postgres", host, port);

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("failed to connect to postgres");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run migrations");

        Self { pool, container }
    }
}
