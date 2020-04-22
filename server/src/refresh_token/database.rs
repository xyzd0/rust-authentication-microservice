use super::model::{RefreshToken, RefreshTokenCreate, RefreshTokenRepository};

use crate::account::model::AccountId;
use crate::error::AuthError;

use async_trait::async_trait;
use sqlx::PgConnection;

#[async_trait]
impl RefreshTokenRepository for PgConnection {
    async fn issue_refresh_token(
        &mut self,
        account_id: AccountId,
    ) -> Result<RefreshToken, AuthError> {
        let token_create = RefreshTokenCreate::new(account_id);

        let refresh_token = sqlx::query_as!(
            RefreshToken,
            r#"
            INSERT INTO refresh_tokens (account_id, issued_at, expires, token)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            account_id,
            token_create.issued_at,
            token_create.expires,
            token_create.token,
        )
        .fetch_one(self)
        .await?;

        Ok(refresh_token)
    }

    async fn revoke_all_tokens_for_account(
        &mut self,
        account_id: AccountId,
    ) -> Result<(), AuthError> {
        let revocation_time = chrono::Utc::now().naive_utc();
        sqlx::query!(
            r#"
            UPDATE refresh_tokens SET revoked = true, revocation_time = $1
            WHERE account_id = $2
            "#,
            revocation_time,
            account_id,
        )
        .execute(self)
        .await?;

        Ok(())
    }
}
