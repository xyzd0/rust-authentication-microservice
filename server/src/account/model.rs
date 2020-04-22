/// Defines identity models.
use std::convert::TryFrom;

use crate::error::AuthError;
use crate::hashing::Argon2id;

use async_trait::async_trait;
use chrono::naive::NaiveDateTime;
use uuid::Uuid;

/// Define a custom type for Identity IDs.
pub type AccountId = i32;

/// Defines the full account details structure.
///
/// This should never be returned in full over the server.
#[derive(Debug, PartialEq)]
pub struct Account {
    pub id: AccountId,
    pub uuid: Uuid,
    pub given_name: String,
    pub email: String,
    pub hash: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
}

/// Defines an account structure that can be inserted into the database.
#[derive(Debug, PartialEq)]
pub(super) struct AccountInsert {
    pub uuid: Uuid,
    pub given_name: String,
    pub email: String,
    pub hash: Option<String>,
    pub created_at: NaiveDateTime,
}

/// Defines the data required to create a new account.
#[derive(Debug, PartialEq)]
pub struct AccountRegister {
    pub given_name: String,
    pub email: String,
    pub password: Option<String>,
}

/// Provide the conversion of an AccountRegister structure to AccountInsert.
///
/// This will hash the provided password.
impl TryFrom<&AccountRegister> for AccountInsert {
    type Error = AuthError;

    fn try_from(account_register: &AccountRegister) -> Result<Self, Self::Error> {
        let AccountRegister {
            given_name,
            email,
            password,
            ..
        } = account_register;

        let hash = match password {
            Some(password) => Some(Argon2id::hash_password(&password)?),
            None => None,
        };

        Ok(Self {
            uuid: Uuid::new_v4(),
            given_name: given_name.to_string(),
            email: email.to_string(),
            hash,
            created_at: chrono::Local::now().naive_utc(),
        })
    }
}

/// Defines the structure for authenticating an existing account.
#[derive(Debug, PartialEq)]
pub struct AccountAuthenticate {
    pub email: String,
    pub password: String,
}

#[async_trait]
pub(crate) trait AccountRepository: Send + Sync + 'static {
    /// Registers a new user in the database.
    ///
    /// # Parameters
    /// The fields required to register a new user.
    ///
    /// # Return Values
    ///
    /// ## Success
    /// A struct containing the authenticated users account details, with a valid JWT.
    ///
    /// ## Errors
    /// An error could occur if the user has already been registered, or a failure occured with the
    /// database.
    async fn register_new_account(
        &mut self,
        account_register: &AccountRegister,
    ) -> Result<Account, AuthError>;

    /// Attempts to authenticate an existing user.
    ///
    /// # Parameters
    /// An authentication attempt struct.
    ///
    /// # Return Values
    ///
    /// ## Success
    /// A struct containing the authenticated users account details.
    ///
    /// ## Errors
    /// If the attempted authentication details were incorrect, or a failure occured with the
    /// database.
    async fn authenticate_account(
        &mut self,
        account_auth: &AccountAuthenticate,
    ) -> Result<Account, AuthError>;
}
