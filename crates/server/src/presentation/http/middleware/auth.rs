use axum::{extract::{Request, State}, http::{ StatusCode, header::AUTHORIZATION}, middleware::Next, response::Response};

use crate::{presentation::http::{extractors::AuthContext, state::AppState}, use_cases::ports::AuthTokenService};

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request, next: Next) -> Result<Response, StatusCode>{
    let token = request.headers().get(AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(|value| value.strip_prefix("Bearer ")).ok_or(StatusCode::UNAUTHORIZED)?;
    let auth_user = state.auth().verify_access_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(AuthContext{
        user_id: auth_user.user_id
    });

    Ok(next.run(request).await)
}
