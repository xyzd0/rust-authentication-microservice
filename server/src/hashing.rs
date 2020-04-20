/// Defines functions to hash passwords, and verify those hashes.
use crate::error::AuthError;

use argonautica::{Hasher, Verifier};

pub struct Argon2idHasher;

impl Argon2idHasher {
    /// Hashes a password using the Argon2id hashing algorithm.AuthError
    ///
    /// # Parameters
    /// Takes a plain-text password as a String. This will be hashed.
    ///
    /// # Return Values
    /// Upon success, the hashed result will be returned, this is safe to store in a database and
    /// use for password verification at a later stage.
    pub fn hash_password(password: &String) -> Result<String, AuthError> {
        let result = Hasher::default()
            .opt_out_of_secret_key(true)
            .with_password(password)
            .hash();

        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(AuthError::HashingError),
        }
    }

    /// Verifies a plain-text password against an Argon2id hash to see if they match.
    ///
    /// # Parameters
    /// A plain-text password and an Argon2id hash.
    ///
    /// # Return Values
    /// `true` if the password matched the hash, `false` otherwise.
    pub fn verify_password(password: &String, hash: &String) -> Result<bool, AuthError> {
        let result = Verifier::default()
            .with_password(password)
            .with_hash(hash)
            .verify();

        match result {
            Ok(result) => Ok(result),
            Err(_) => Err(AuthError::HashingError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashing_and_verification_works_as_expected() {
        let password = "P@ssw0rd".to_string();

        let hash =
            Argon2idHasher::hash_password(&password).expect("hash_password returned an error");

        let is_match = Argon2idHasher::verify_password(&password, &hash)
            .expect("verify_password returned an error");

        assert!(is_match);
    }
}
