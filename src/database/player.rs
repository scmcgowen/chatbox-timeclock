use uuid::Uuid;


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Player {
    pub id: i32,
    name: String,
    uuid: Uuid,
    pub admin: bool,
}

impl<'q> Player {


    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub async fn create<E>(pool: E, name: String, uuid: Uuid) -> Result<Player, sqlx::Error>
    where
        E: sqlx::Acquire<'q, Database = sqlx::Postgres> + Send + 'q,
    {
        let mut tx = pool.begin().await?;
        let q = r#"INSERT INTO players (name, uuid) VALUES ($1, $2) RETURNING *"#;
        let player = sqlx::query_as(q)
            .bind(name)
            .bind(uuid)
            .fetch_one(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(player)
    }
    pub async fn promote<E>(self,pool: E) -> Result<Player, sqlx::Error>
    where
        E: sqlx::Acquire<'q, Database = sqlx::Postgres> + Send + 'q,
    {
        let mut tx = pool.begin().await?;
        let q = r#"UPDATE players SET admin = true WHERE id = $1 RETURNING *"#;
        let player = sqlx::query_as(q).bind(self.id).fetch_one(&mut *tx).await?;
        tx.commit().await?;
        Ok(player)
    }
    pub async fn demote<E>(self,pool: E) -> Result<Player, sqlx::Error>
    where
        E: sqlx::Acquire<'q, Database = sqlx::Postgres> + Send + 'q,
    {
        let mut tx = pool.begin().await?;
        let q = r#"UPDATE players SET admin = false WHERE id = $1 RETURNING *"#;
        let player = sqlx::query_as(q).bind(self.id).fetch_one(&mut *tx).await?;
        tx.commit().await?;
        Ok(player)
    }
    pub async fn remove<E>(self,pool: E) -> Result<(), sqlx::Error>
    where
        E: sqlx::Acquire<'q, Database = sqlx::Postgres> + Send + 'q,
    {
        let mut tx = pool.begin().await?;
        let q = r#"DELETE FROM players WHERE id = $1"#;
        sqlx::query(q).bind(self.id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_by_uuid<E>(pool: E, uuid: Uuid) -> Result<Player, sqlx::Error>
    where
        E: sqlx::Executor<'q, Database = sqlx::Postgres> + Send + 'q,
    {
        let q = r#"SELECT * FROM players WHERE uuid = $1"#;
        sqlx::query_as(q).bind(uuid).fetch_one(pool).await
    }

    pub async fn get_by_username<E>(pool: E, username: String) -> Result<Self, sqlx::Error>
    where
        E: sqlx::Executor<'q, Database = sqlx::Postgres> + Send + 'q,
    {
        let q = r#"SELECT * FROM players WHERE name = $1::citext"#;
        sqlx::query_as(q).bind(username).fetch_one(pool).await
    }
    pub async fn get_admins<E>(pool: E) -> Result<Vec<Player>, sqlx::Error>
    where
        E: sqlx::Executor<'q, Database = sqlx::Postgres> + Send + 'q,
    {
        let q = r#"SELECT * FROM players WHERE admin = true"#;
        sqlx::query_as(q).fetch_all(pool).await
    }
}
