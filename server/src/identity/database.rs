use super::model::{Identity, IdentityCreate, IdentityRepository};

use crate::account::model::AccountId;
use crate::error::AuthError;

use async_trait::async_trait;
use sqlx::PgConnection;

#[async_trait]
impl IdentityRepository for PgConnection {
    async fn add_identity(
        &mut self,
        identity_create: &IdentityCreate,
    ) -> Result<Identity, AuthError> {
        // Unchecked for now as query macros do not appear to support custom enum types.
        let identity = sqlx::query_as_unchecked!(
            Identity,
            r#"
            INSERT INTO identities(account_id, source)
            VALUES($1, $2)
            RETURNING *
            "#,
            identity_create.account_id,
            identity_create.source,
        )
        .fetch_one(self)
        .await?;

        Ok(identity)
    }

    async fn get_identities_for_account(
        &mut self,
        account_id: AccountId,
    ) -> Result<Vec<Identity>, AuthError> {
        // Unchecked for now as query macros do not appear to support custom enum types.
        Ok(sqlx::query_as_unchecked!(
            Identity,
            r#"
            SELECT * FROM identities WHERE account_id = $1
            "#,
            account_id
        )
        .fetch_all(self)
        .await?)
    }
}
