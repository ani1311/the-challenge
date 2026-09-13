use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::use_cases::ports::{AuthTokenError, AuthTokenService, AuthUser};

#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

impl AuthTokenService for JwtService {
    fn issue_access_token(&self, user: AuthUser) -> Result<String, AuthTokenError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        debug!(user_id = %user.user_id, "issuing access token");

        let claims = Claims {
            sub: user.user_id,
            iat: now,
            exp: now + 60 * 60 * 24,
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| AuthTokenError::Failed)
    }

    fn verify_access_token(&self, token: &str) -> Result<AuthUser, AuthTokenError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthTokenError::Failed)?;

        debug!(user_id = %token_data.claims.sub, "verified access token");

        Ok(AuthUser {
            user_id: token_data.claims.sub,
        })
    }
}
