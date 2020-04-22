mod account;
mod database;
mod error;
mod hashing;
mod identity;
mod jwt;
mod refresh_token;
mod server;

use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_addr = dotenv::var("SERVER_ADDR").expect("SERVER_URL must be set");

    let pool = database::postgres::connect(&database_url).await?;
    let auth_service = server::AuthService::new(pool);

    auth_service.run_server(server_addr.parse()?).await?;

    Ok(())
}
