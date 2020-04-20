use crate::error::AuthError;
use crate::hashing::Argon2idHasher;
use crate::repository::{
    AccountEntity, AuthRepository, EntityId, IdentityEntity, IdentityProvider, RefreshToken,
};
use crate::token;

use async_trait::async_trait;

use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgPool};

/// How many hours refresh tokens are valid for.
const REFRESH_EXPIRY_HOURS: i64 = 24 * 7;

#[derive(Debug)]
pub struct AuthDbRepo(PgPool);

impl AuthDbRepo {
    /// Creates a new AuthDbRepo structure.
    ///
    /// # Parameters
    /// A URL to a Postgres database. All database operations will be performed on this datadabase,
    /// after connecting.
    ///
    /// # Returns
    /// The created [`AuthDbRepo`] struct.
    ///
    /// # Errors
    /// This will panic if the database URL cannot be connected to.
    pub async fn new(database_url: &String) -> Self {
        AuthDbRepo(
            PgPool::new(database_url)
                .await
                .expect(&format!("Unable to open database at {}", database_url)),
        )
    }

    /// Attempts to obtain a connection from the connection pool.
    pub async fn conn(&self) -> Result<PoolConnection<PgConnection>, AuthError> {
        Ok(self.0.acquire().await?)
    }
}

#[async_trait]
impl AuthRepository for AuthDbRepo {
    async fn register_new_user(
        &self,
        email: &String,
        given_name: &String,
        family_name: &String,
        identity_provider: &IdentityProvider,
        token: &String,
    ) -> Result<AccountEntity, AuthError> {
        // Explicitly begin a transaction, meaning we will rollback any changes unless all queries
        // successfully complete, and are comitted.
        let conn = self.0.begin().await?;

        // First, check if an account with this email already exists.
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM accounts
            WHERE email = $1
            "#,
            email
        )
        .bind(email)
        .fetch_all(&mut conn)
        .await
        .map(|recs| recs.count);

        // Here, we return immeidately as OAuth style sign-ins currently aren't supported. If they
        // were, we would skip to adding a new identity for an existing account instead.
        if count > 0 {
            return Err(AuthError::UserAlreadyExists(email.to_string()));
        }

        // Then we want to create the account record, and return the ID of created account.
        let account = sqlx::query!(
            r#"
            INSERT INTO accounts(given_name, family_name, email)
            VALUES($1, $2, $3)
            "#,
            given_name,
            family_name,
            email,
        )
        .fetch_one(&mut conn)
        .await?;

        // Now insert the identity provider we have been given, so the user is able to authenticate
        self.add_identity_provider(&account.id, &identity_provider, &token)
            .await?;

        // If we reached here, all queries went ok, commit them!
        conn.commit().await?;

        Ok(account)
    }

    async fn add_identity_provider(
        &self,
        account_id: &EntityId,
        provider: &IdentityProvider,
        token: &String,
    ) -> Result<IdentityEntity, AuthError> {
        let conn = self.conn().await?;

        // If the identity provider is a password, we must hash it before storing it.
        let token_to_insert = match provider {
            IdentityProvider::Password => Argon2idHasher::hash_password(token)?,
            _ => token.to_string(),
        };

        let identity = sqlx::query!(
            r#"
            INSERT INTO identites(account_id, provider, token)
            VALUES($1, Password, $2)
            "#,
            account_id,
            token_to_insert
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(identity)
    }

    async fn authenticate_user(
        &self,
        email: &String,
        provider: &IdentityProvider,
        password: &String,
    ) -> Result<AccountEntity, AuthError> {
        let conn = self.conn().await?;

        let account = sqlx::query!(
            r#"
            SELECT * FROM accounts WHERE email = $1
            "#,
            email
        )
        .fetch_one(&mut conn)
        .await?;

        let hash = sqlx::query!(
            r#"
            SELECT token FROM identites
            WHERE account_id = $1 AND provider = $2
            "#,
            account.id
        )
        .fetch_one(&mut conn)
        .await
        .map(|rec| rec.token)?;

        match Argon2idHasher::verify_password(&password, &hash) {
            Ok(true) => Ok(account),
            Ok(false) => Err(AuthError::InvalidUsernameOrPassword),
            Err(e) => Err(e),
        }
    }

    async fn generate_refresh_token(
        &self,
        account_id: &EntityId,
    ) -> Result<RefreshToken, AuthError> {
        let conn = self.conn().await?;

        let token = token::RefreshToken::generate();
        let issued_at = chrono::Utc::now();
        let expiry = issued_at + chrono::Duration::hours(REFRESH_EXPIRY_HOURS);

        let refresh_token = sqlx::query!(
            r#"
            INSERT INTO refresh_tokens(account_id, issued_at, expiry, token)
            VALUES($1, $2, $3, $4)
            "#,
            account_id,
            issued_at,
            expiry,
            token
        )
        .fetch_one(&mut conn)
        .await?;

        Ok(RefreshToken {
            token: token,
            expiry: expiry.timestamp(),
        })
    }
}
