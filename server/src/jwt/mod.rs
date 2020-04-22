/// Json Web Tokens (JWT) are for short term authentication use.
///
/// They are used to provide users access to resources without having to check against the database.
///
/// Clients should ask for a re-issue of a JWT using the refresh token when it is close to expiry.
///
pub mod generate;
pub mod model;
