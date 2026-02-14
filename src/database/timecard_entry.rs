use chrono::{DateTime, Utc};
use std::time::Duration;
use sqlx::Error;
use uuid::Uuid;
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct TimecardEntry {
    pub id: i32,
    pub user_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

impl<'q> TimecardEntry {
    pub async fn create<E>(pool: E, user_id: &Uuid) -> Result<TimecardEntry, Error>
    where
        E: 'q + sqlx::Acquire<'q, Database = sqlx::Postgres>,
    {
        let mut tx = pool.begin().await?;
        let q = r#"INSERT INTO timecards (user_id, start_time) VALUES ($1, NOW()) RETURNING *"#;
        let row = sqlx::query_as(q).bind(user_id).fetch_one(&mut *tx).await?;
        tx.commit().await?;
        Ok(row)
    }

    pub async fn end<E>(&self, pool: E, description: Option<String>) -> Result<TimecardEntry, sqlx::Error>
    where
        E: 'q + sqlx::Acquire<'q, Database = sqlx::Postgres>,
    {
        let mut tx = pool.begin().await?;
        let q = r#"UPDATE timecards SET end_time = NOW(), description = $2 WHERE id = $1 RETURNING *"#;
        let row = sqlx::query_as(q).bind(self.id).bind(description).fetch_one(&mut *tx).await?;
        tx.commit().await?;
        Ok(row)
    }

    pub async fn get<E>(pool: E, id: i32) -> Result<Self, sqlx::Error>
    where
        E: 'q + sqlx::Acquire<'q, Database = sqlx::Postgres>,
    {
        let mut tx = pool.begin().await?;
        let q = r#"SELECT * FROM timecards WHERE id = $1"#;
        let row = sqlx::query_as(q).bind(id).fetch_one(&mut *tx).await?;
        tx.commit().await?;
        Ok(row)
    }
    pub async fn get_by_user<E>(pool: E, user_id: &Uuid) -> Result<Vec<Self>, sqlx::Error>
    where
        E: 'q + sqlx::Acquire<'q, Database = sqlx::Postgres>,
    {
        let mut tx = pool.begin().await?;
        let q = r#"SELECT * FROM timecards WHERE user_id = $1"#;
        let rows = sqlx::query_as(q).bind(user_id).fetch_all(&mut *tx).await?;
        tx.commit().await?;
        Ok(rows)
    }
    pub fn is_clocked_in(&self) -> bool {
        self.end_time.is_none()
    }
    pub fn get_total_time(&self) -> Duration {
        (self.end_time.unwrap_or_else(|| Utc::now()) - self.start_time).abs().to_std().unwrap()
    }
}
