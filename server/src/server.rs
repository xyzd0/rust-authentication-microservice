pub(crate) mod auth {
    tonic::include_proto!("com.service.auth");
}

use std::net::SocketAddr;

use auth::auth_server::{Auth, AuthServer};
use auth::{
    authenticated_user_response::RefreshToken as ProtoRefreshToken, AuthenticatedUserResponse,
    AuthenticationRequest, RegisterUserRequest,
};

use crate::account::model::{AccountAuthenticate, AccountRegister, AccountRepository};
use crate::database::Db;
use crate::jwt;
use crate::refresh_token::model::RefreshTokenRepository;

use sqlx::PgPool;
use tonic::{transport::Server, Request, Response, Status};

/// The AuthService struct is used for handling incoming gRPC requests to this microservice.
pub struct AuthService {
    pool: PgPool,
}

impl AuthService {
    /// Creates a new AuthService instance.
    pub fn new(pool: PgPool) -> AuthService {
        Self { pool }
    }

    pub async fn run_server(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        Server::builder()
            .add_service(AuthServer::new(self))
            .serve(addr)
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<AuthenticatedUserResponse>, Status> {
        println!("Got register_user request from {:?}", request.remote_addr());

        let mut conn = self.pool.conn().await?;
        let inner_request = request.into_inner();
        let account = conn
            .register_new_account(&AccountRegister {
                given_name: inner_request.given_name,
                email: inner_request.email,
                password: Some(inner_request.password),
            })
            .await?;

        let jwt = jwt::generate::create_token(account.id, &account.email)?;
        let refresh_token = conn.issue_refresh_token(account.id).await?;

        Ok(Response::new(AuthenticatedUserResponse {
            jwt,
            refresh_token: Some(ProtoRefreshToken {
                issued_at: refresh_token.issued_at.timestamp(),
                expires: refresh_token.expires.timestamp(),
                token: refresh_token.token,
            }),
        }))
    }

    async fn authenticate_user(
        &self,
        request: Request<AuthenticationRequest>,
    ) -> Result<Response<AuthenticatedUserResponse>, Status> {
        println!(
            "Got authenticate_user request from {:?}",
            request.remote_addr()
        );

        let mut conn = self.pool.conn().await?;
        let inner_request = request.into_inner();
        let account = conn
            .authenticate_account(&AccountAuthenticate {
                email: inner_request.email,
                password: inner_request.password,
            })
            .await?;

        let jwt = jwt::generate::create_token(account.id, &account.email)?;
        let refresh_token = conn.issue_refresh_token(account.id).await?;

        Ok(Response::new(AuthenticatedUserResponse {
            jwt,
            refresh_token: Some(ProtoRefreshToken {
                issued_at: refresh_token.issued_at.timestamp(),
                expires: refresh_token.expires.timestamp(),
                token: refresh_token.token,
            }),
        }))
    }
}
