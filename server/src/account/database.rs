/// Implements the AccountRepository trait for a PostgreSQL database.
use std::convert::TryFrom;

use super::model::{
    Account, AccountAuthenticate, AccountInsert, AccountRegister, AccountRepository,
};

use crate::error::AuthError;
use crate::hashing::Argon2id;

use async_trait::async_trait;
use sqlx::PgConnection;

#[async_trait]
impl AccountRepository for PgConnection {
    async fn register_new_account(
        &mut self,
        account_register: &AccountRegister,
    ) -> Result<Account, AuthError> {
        let account = AccountInsert::try_from(account_register)?;
        let registered_account = sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts (uuid, given_name, email, hash)
            VALUES($1, $2, $3, $4)
            RETURNING *
            "#,
            account.uuid,
            account.given_name,
            account.email,
            account.hash,
        )
        .fetch_one(self)
        .await?;

        Ok(registered_account)
    }

    async fn authenticate_account(
        &mut self,
        account_auth: &AccountAuthenticate,
    ) -> Result<Account, AuthError> {
        // Get the account struct
        let account = sqlx::query_as!(
            Account,
            r#"
            SELECT * FROM accounts WHERE email = $1
            "#,
            account_auth.email
        )
        .fetch_one(self)
        .await?;

        let password_hash = match &account.hash {
            Some(hash) => Ok(hash),
            None => Err(AuthError::InvalidUsernameOrPassword),
        }?;

        // First check if we need to verify password hashes
        match Argon2id::verify_password(&account_auth.password, &password_hash) {
            Ok(true) => Ok(account),
            Ok(false) => Err(AuthError::InvalidUsernameOrPassword),
            Err(e) => Err(e),
        }
    }
}
