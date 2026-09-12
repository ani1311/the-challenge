use crate::use_cases::ports::{AuthTokenService, AuthUser};

#[derive(Clone)]
pub struct JwtService;

impl JwtService {
    pub fn new() -> Self {
        Self
    }
}

impl AuthTokenService for JwtService {
    fn issue_access_token(&self, user: crate::use_cases::ports::AuthUser) -> Result<String, crate::use_cases::ports::AuthTokenError> {
        Ok(format!("token User: {}", user.user_id).to_string())
    }

    fn verify_access_token(&self, token: &str) -> Result<crate::use_cases::ports::AuthUser, crate::use_cases::ports::AuthTokenError> {
        Ok(AuthUser{
            user_id: "SomeUser".to_string()
        })
    }
}
