use super::Db;
use crate::error::AuthError;

use async_trait::async_trait;
use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, PgPool};

/// Open a connection to a postgres database
pub async fn connect(db_url: &str) -> Result<PgPool, AuthError> {
    match PgPool::new(db_url).await {
        Ok(pool) => Ok(pool),
        Err(e) => Err(AuthError::DatabaseError(e)),
    }
}

#[async_trait]
impl Db for PgPool {
    type Conn = PoolConnection<PgConnection>;

    async fn conn(&self) -> Result<Self::Conn, AuthError> {
        self.acquire().await.map_err(AuthError::DatabaseError)
    }
}
