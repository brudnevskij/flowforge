use sqlx::PgPool;

pub struct ReadinessChecker {
    pool: PgPool,
}

impl ReadinessChecker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn is_ready(&self) -> bool {
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }
}
