pub mod auth {
    tonic::include_proto!("com.service.auth");
}

use std::net::SocketAddr;
use std::sync::Arc;

use auth::auth_server::{Auth, AuthServer};
use auth::{
    authenticated_user_response, AuthenticatedUserResponse, AuthenticationRequest,
    RegisterUserRequest, ValidateJwtRequest, ValidateJwtResponse,
};

use crate::repository::{AuthRepository, IdentityProvider};
use crate::token;

use num_traits::FromPrimitive;

use tonic::{transport::Server, Request, Response, Status};

/// The AuthService struct is used for handling incoming gRPC requests to this microservice.
pub struct AuthService<T> {
    repository: Arc<T>,
}

impl<T> AuthService<T>
where
    T: AuthRepository + Send + Sync + 'static,
{
    /// Creates a new AuthService instance.
    pub fn new(repository: T) -> AuthService<T> {
        AuthService {
            repository: Arc::new(repository),
        }
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
impl<T> Auth for AuthService<T>
where
    T: AuthRepository + Send + Sync + 'static,
{
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<AuthenticatedUserResponse>, Status> {
        println!("Got register_user request from {:?}", request.remote_addr());

        let inner_request = request.into_inner();
        let identity_provider = match IdentityProvider::from_i32(inner_request.identity_provider) {
            Some(provider) => provider,
            None => {
                return Err(Status::invalid_argument(format!(
                    "IdentityProvider {:?} is not a valid value",
                    inner_request.identity_provider
                )))
            }
        };

        let account = self
            .repository
            .register_new_user(
                &inner_request.email,
                &inner_request.given_name,
                &inner_request.family_name,
                &identity_provider,
                &inner_request.password,
            )
            .await?;

        let refresh_token = self.repository.generate_refresh_token(&account.id).await?;
        let jwt = token::JsonWebToken::create_token(&account.id, &account.email)?;

        Ok(Response::new(AuthenticatedUserResponse {
            jwt,
            refresh_token: Some(authenticated_user_response::RefreshToken {
                token: refresh_token.token,
                expiry: refresh_token.expiry,
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

        let inner_request = request.into_inner();
        let identity_provider = match IdentityProvider::from_i32(inner_request.identity_provider) {
            Some(provider) => provider,
            None => {
                return Err(Status::invalid_argument(format!(
                    "IdentityProvider {:?} is not a valid value",
                    inner_request.identity_provider
                )))
            }
        };

        let account = self
            .repository
            .authenticate_user(
                &inner_request.email,
                &identity_provider,
                &inner_request.password,
            )
            .await?;

        let refresh_token = self.repository.generate_refresh_token(&account.id).await?;
        let jwt = token::JsonWebToken::create_token(&account.id, &account.email)?;

        Ok(Response::new(AuthenticatedUserResponse {
            jwt,
            refresh_token: Some(authenticated_user_response::RefreshToken {
                token: refresh_token.token,
                expiry: refresh_token.expiry,
            }),
        }))
    }

    async fn validate_jwt(
        &self,
        request: Request<ValidateJwtRequest>,
    ) -> Result<Response<ValidateJwtResponse>, Status> {
        match token::JsonWebToken::validate_token(&request.into_inner().jwt) {
            Ok(_) => Ok(Response::new(ValidateJwtResponse { is_valid: true })),
            Err(e) => Err(e.into()),
        }
    }
}
