use tracing::debug;

use crate::use_cases::ports::{AuthTokenError, AuthTokenService, AuthUser, UserRepository, UserRepositoryError};



pub struct Login<UserRepo, AuthService> {
    user_repo: UserRepo,
    auth_service: AuthService
}

pub struct LoginInput {
    pub username: String,
}

pub struct LoginOutput {
    pub access_token: String,
}

pub enum LoginError {
    UserNotFound,
    TokenFailed,
}

impl From<UserRepositoryError> for LoginError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::UserNotFound => LoginError::UserNotFound,
        }
    }
}

impl From<AuthTokenError> for LoginError {
    fn from(_: AuthTokenError) -> Self {
        LoginError::TokenFailed
    }
}

impl <UserRepo, AuthService> Login<UserRepo,AuthService > where UserRepo: UserRepository, AuthService: AuthTokenService {
    pub fn new(user_repo: UserRepo, auth_service: AuthService) -> Self {
        Self {
            user_repo,
            auth_service
        }
    }

    pub async fn execute(&self, input: LoginInput) -> Result<LoginOutput, LoginError> {
        let user = self.user_repo.lookup_user(input.username).await?;

        debug!(username = %&&user.id(), "usecase for token");

        let access_token = self.auth_service.issue_access_token(AuthUser { user_id: user.id() })?;

        Ok(LoginOutput { access_token })
    }
}
