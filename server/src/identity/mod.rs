/// Identites are methods of authenticating an account.
///
/// Identities have a many-to-one relationship with accounts, meaning that one account can be
/// identitied with multiple methods.
///
/// This allows support for third-party sign-in providers such as Google and Facebook.
///
/// Currently only password identities are supported.
///
pub mod database;
pub mod model;
