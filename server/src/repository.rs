use crate::error::AuthError;

use async_trait::async_trait;

use num_derive::{FromPrimitive, ToPrimitive};

pub type EntityId = i32;

/// Define the Account model.
pub struct AccountEntity {
    pub id: EntityId,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub avatar_url: Option<String>,
}

/// Defines the possible identity providers.
///
/// An IdentityProvider is a type of possible authentication, e.g. a regular password or a
/// third party OAuth provider such as Google or Facebook sign-in.
#[derive(PartialEq, Debug, FromPrimitive, ToPrimitive, sqlx::Type)]
pub enum IdentityProvider {
    Password,
    Google,
}

/// Defines an Identity model.
///
/// Contains a method of authentication for an account.
pub struct IdentityEntity {
    pub id: EntityId,
    pub account_id: EntityId,
    pub provider: IdentityProvider,
    pub identity_token: String,
}

/// Defines the structure of a refresh token.
pub struct RefreshToken {
    pub token: String,
    pub expiry: i64,
}

/// Tokens issued by this auth service upon successful authentication.
pub struct AuthTokens {
    pub jwt: String,
    pub refresh_token: Option<RefreshToken>,
}

/// The AuthRepository trait defines a contract for CRUD operations on authentication related data.
#[async_trait]
pub trait AuthRepository: Send + Sync + 'static {
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
    async fn register_new_user(
        &self,
        email: &String,
        given_name: &String,
        family_name: &String,
        identity_provider: &IdentityProvider,
        token: &String,
    ) -> Result<AccountEntity, AuthError>;

    /// Adds a new identity provider for an existing account.
    ///
    /// # Parameters
    /// The fields required to add a new identity provider.
    ///
    /// # Return Values
    ///
    /// ## Success
    /// The newly created IdentityEntity struct.
    ///
    /// ## Errors
    /// An error could occur if the account could not be found, or a failure occured with the
    /// database.
    async fn add_identity_provider(
        &self,
        account_id: &EntityId,
        provider: &IdentityProvider,
        token: &String,
    ) -> Result<IdentityEntity, AuthError>;

    /// Attempts to authenticate an existing user.
    ///
    /// # Parameters
    /// An authentication attempt struct.
    ///
    /// # Return Values
    ///
    /// ## Success
    /// A struct containing the authenticated users account details, with a valid JWT and refresh
    /// token.
    ///
    /// ## Errors
    /// If the attempted authentication details were incorrect, or a failure occured with the
    /// database.
    async fn authenticate_user(
        &self,
        email: &String,
        provider: &IdentityProvider,
        password: &String,
    ) -> Result<AccountEntity, AuthError>;

    /// Adds a new refresh token to the database for the given account ID.
    ///
    /// These tokens expire after 7 days. They should be refreshed before that.
    ///
    /// # Return Values
    ///
    /// ## Success
    /// A populated RefreshToken structure.
    ///
    /// ## Errors
    /// If a failure occured with the database.
    async fn generate_refresh_token(
        &self,
        account_id: &EntityId,
    ) -> Result<RefreshToken, AuthError>;
}
