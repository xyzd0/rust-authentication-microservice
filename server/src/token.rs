/// Module for generating tokens used for authentication.
use crate::error::AuthError;
use crate::repository::EntityId;

use chrono::{Duration, Utc};
use dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

const JWT_ISSUER: &str = "authentication";
const JWT_EXPIRY_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user email
    pub email: String,
}

impl Claims {
    pub fn new(account_id: &EntityId, email: &String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(JWT_EXPIRY_HOURS);

        Claims {
            iss: JWT_ISSUER.to_string(),
            sub: account_id.to_string(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            email: email.clone(),
        }
    }
}

#[derive(Debug)]
pub struct JsonWebToken;

impl JsonWebToken {
    /// Creates a new JWT for the provided username.
    ///
    /// This will expire in 1 day from now.
    ///
    /// NOTE: This currently usees HS256 which needs a shared secret. It would be better to use
    /// RS256, publishing the public key in like Google does https://www.googleapis.com/oauth2/v3/certs.
    /// This will require a certificate authority for the microserivces, and will be done at a later
    /// stage.
    ///
    pub fn create_token(account_id: &EntityId, email: &String) -> Result<String, AuthError> {
        let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let claims: Claims = Claims::new(&account_id, &email);

        encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| AuthError::InvalidToken(e))
    }

    /// Validates a given JWT, ensuring it is valid and stll signed.
    ///
    /// Returns the JWT's claims.
    pub fn validate_token(token: &str) -> Result<Claims, AuthError> {
        let secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::RS256),
        )
        .map(|data| data.claims)
        .map_err(|e| AuthError::InvalidToken(e))
    }
}

#[derive(Debug)]
pub struct RefreshToken;

impl RefreshToken {
    /// Generates a 256 length alphanumeric string to be used as a refresh token.
    pub fn generate() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(256).collect()
    }
}
