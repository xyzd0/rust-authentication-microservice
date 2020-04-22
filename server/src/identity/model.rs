/// Defines identity models.
use crate::account::model::AccountId;
use crate::error::AuthError;

use async_trait::async_trait;
use num_derive::{FromPrimitive, ToPrimitive};

/// Define a custom type for Identity IDs.
pub type IdentityId = i32;

/// Defines the supported identity sources.
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive, sqlx::Type)]
#[sqlx(rename = "identitysource")]
#[sqlx(rename_all = "lowercase")]
pub enum IdentitySource {
    Password,
    Google,
}
#[derive(Debug, PartialEq)]
pub struct Identity {
    pub id: IdentityId,
    pub account_id: AccountId,
    pub source: IdentitySource,
}

#[derive(Debug, PartialEq)]
pub struct IdentityCreate {
    pub account_id: AccountId,
    pub source: IdentitySource,
}

/// Defines repository based data options for the Identity data type.
#[async_trait]
pub(super) trait IdentityRepository {
    /// Adds a new identity for an existing account.
    ///
    /// # Parameters
    /// The fields required to add a new identity.
    ///
    /// # Return Values
    /// ## Success
    /// The newly created Identity struct.
    ///
    /// ## Errors
    /// If the account was not found, or a database failure occured.
    async fn add_identity(
        &mut self,
        identity_create: &IdentityCreate,
    ) -> Result<Identity, AuthError>;

    /// Gets a list of all identities for an exisitng account.
    ///
    /// # Parameters
    /// The account ID to get identities for.
    ///
    /// # Return Values
    /// ## Success
    /// A vector containing all account identities.
    ///
    /// ## Errors
    /// If the account was not found, or a database failure occured.
    async fn get_identities_for_account(
        &mut self,
        account_id: AccountId,
    ) -> Result<Vec<Identity>, AuthError>;
}
