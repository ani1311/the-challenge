#[derive(Clone)]
pub struct AuthUser {
    pub user_id: String,
}

pub enum AuthTokenError {
    Failed,
}

pub trait AuthTokenService {
    fn issue_access_token(&self, user: AuthUser) -> Result<String, AuthTokenError>;
    fn verify_access_token(&self, token: &str) -> Result<AuthUser, AuthTokenError>;
}
