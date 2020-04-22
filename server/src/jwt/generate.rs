/// Generates Json Web Tokens.
use super::model::Claims;

use crate::account::model::AccountId;
use crate::error::AuthError;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Creates a new JWT for the provided username.
///
/// This will expire in 1 day from now.
///
/// NOTE: This currently usees HS256 which needs a shared secret. It would be better to use
/// RS256, publishing the public key in like Google does https://www.googleapis.com/oauth2/v3/certs.
/// This will require a certificate authority for the microserivces, and will be done at a later
/// stage.
///
pub(crate) fn create_token(account_id: AccountId, email: &str) -> Result<String, AuthError> {
    let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims: Claims = Claims::new(account_id, &email);

    encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(AuthError::InvalidToken)
}

/// Validates a given JWT, ensuring it is valid and stll signed.
///
/// Returns the JWT's claims.
pub(crate) fn validate_token(token: &str) -> Result<Claims, AuthError> {
    let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
    .map_err(AuthError::InvalidToken)
}
