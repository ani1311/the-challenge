use axum::{extract::FromRequestParts, http::StatusCode};



#[derive(Clone)]
pub struct AuthContext {
    pub user_id: String
}

impl<S> FromRequestParts<S> for AuthContext where S: Send + Sync{
    type Rejection = StatusCode;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        parts.extensions.get::<AuthContext>().cloned().ok_or(StatusCode::UNAUTHORIZED)
    }
}
