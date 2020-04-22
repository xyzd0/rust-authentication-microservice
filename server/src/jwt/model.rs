/// Defines JWT models.
use crate::account::model::AccountId;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

const JWT_ISSUER: &str = "authentication";
const JWT_EXPIRY_HOURS: i64 = 1;

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
    pub fn new(account_id: AccountId, email: &str) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(JWT_EXPIRY_HOURS);

        Claims {
            iss: JWT_ISSUER.to_string(),
            sub: account_id.to_string(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            email: email.to_string(),
        }
    }
}
