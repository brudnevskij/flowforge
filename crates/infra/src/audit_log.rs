use chrono::{DateTime, Utc};
use domain::AuditLog;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct AuditLogRow {
    id: Uuid,
    order_id: Uuid,
    event_type: String,
    message: String,
    metadata: Option<serde_json::Value>,
    created_at: DateTime<Utc>,
}

impl From<AuditLogRow> for AuditLog {
    fn from(value: AuditLogRow) -> Self {
        AuditLog {
            id: value.id,
            order_id: value.order_id,
            event_type: value.event_type,
            message: value.message,
            metadata: value.metadata,
            created_at: value.created_at,
        }
    }
}

pub struct PostgresAuditLogRepository {
    pool: PgPool,
}

impl PostgresAuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_audit_log(&self, audit_log: &AuditLog) -> anyhow::Result<AuditLog> {
        let row: AuditLogRow = sqlx::query_as(
            r#"
            INSERT INTO audit_logs (
                id,
                order_id,
                event_type,
                message,
                metadata,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(audit_log.id)
        .bind(audit_log.order_id)
        .bind(&audit_log.event_type)
        .bind(&audit_log.message)
        .bind(audit_log.metadata.as_ref())
        .bind(audit_log.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    pub async fn insert_tx(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        audit_log: &AuditLog,
    ) -> anyhow::Result<AuditLog> {
        let row: AuditLogRow = sqlx::query_as(
            r#"
        INSERT INTO audit_logs (
            id,
            order_id,
            event_type,
            message,
            metadata,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        )
        .bind(audit_log.id)
        .bind(audit_log.order_id)
        .bind(&audit_log.event_type)
        .bind(&audit_log.message)
        .bind(audit_log.metadata.as_ref())
        .bind(audit_log.created_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(row.into())
    }
}
