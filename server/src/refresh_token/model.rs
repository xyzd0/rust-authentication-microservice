/// Data models for Refresh Tokens.
use crate::account::model::AccountId;
use crate::error::AuthError;

use async_trait::async_trait;
use chrono::naive::NaiveDateTime;
use chrono::{Duration, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Define a custom type for Refresh Token IDs.
pub type RefreshTokenId = i32;

/// Defne the number of characters a refresh token should contain.
const TOKEN_LENGTH: usize = 256;

/// Define the default expiry length of a refresh token, in hours.
const TOKEN_EXPIRY_HOURS: i64 = 7 * 24; // 168 hours, or 7 days.

#[derive(Debug)]
pub struct RefreshToken {
    pub id: RefreshTokenId,
    pub account_id: AccountId,
    pub issued_at: NaiveDateTime,
    pub expires: NaiveDateTime,
    pub revoked: bool,
    pub revocation_time: Option<NaiveDateTime>,
    pub token: String,
}

#[derive(Debug)]
pub struct RefreshTokenCreate {
    pub account_id: AccountId,
    pub issued_at: NaiveDateTime,
    pub expires: NaiveDateTime,
    pub token: String,
}

impl RefreshTokenCreate {
    /// Generates a new refresh token, with the default expiry time.
    pub fn new(account_id: AccountId) -> Self {
        let issued_at = Utc::now();
        let expires = issued_at + Duration::hours(TOKEN_EXPIRY_HOURS);

        RefreshTokenCreate {
            account_id,
            issued_at: issued_at.naive_utc(),
            expires: expires.naive_utc(),
            token: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(TOKEN_LENGTH)
                .collect(),
        }
    }
}

#[async_trait]
pub(crate) trait RefreshTokenRepository {
    /// Issues a new refresh token for the given account.
    ///
    /// # Parameters
    /// The ID of the account to issue a token for.
    ///
    /// # Returns
    /// ## Success
    /// The newly created RefreshToken structure.
    ///
    /// ## Errors
    /// If the account was not found, or a database failure occured.
    async fn issue_refresh_token(
        &mut self,
        account_id: AccountId,
    ) -> Result<RefreshToken, AuthError>;

    /// Revokes all refresh tokens issued for an account.
    ///
    /// # Parameters
    /// The account ID to revoke all tokens for.
    ///
    /// # Returns
    /// ## Success
    /// Ok, but empty.
    ///
    /// ## Errors
    /// If the account was not found, or a database failure occured.
    async fn revoke_all_tokens_for_account(
        &mut self,
        account_id: AccountId,
    ) -> Result<(), AuthError>;
}
