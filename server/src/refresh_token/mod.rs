/// Refresh tokens are used for session management.
///
/// Refresh tokens are longer lived than Json Web Tokens, and can be used to issue a new JWT for
/// an account, and to issue a new refresh token if the current one is close to expiry.
///
/// These are more secure for session management as they are stored in the database and allow
/// for instant recovation.
///
pub mod database;
pub mod model;
