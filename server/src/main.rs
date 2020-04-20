mod database;
mod error;
mod hashing;
mod repository;
mod server;
mod token;

use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_addr = dotenv::var("SERVER_ADDR").expect("SERVER_URL must be set");

    let auth_repo = database::AuthDbRepo::new(&database_url).await;
    let auth_service = server::AuthService::new(auth_repo);

    auth_service
        .run_server(
            server_addr
                .parse()
                .expect("SEVER_ADDR is not a valid socket address"),
        )
        .await?;

    Ok(())
}
