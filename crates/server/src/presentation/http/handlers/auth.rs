use axum::{extract::State, http::StatusCode, Json};
use common::{
    auth::{LoginRequest, LoginResponse},
    users::{RegisterUserRequest, RegisterUserResponse},
};
use tracing::debug;

use crate::{
    presentation::http::state::AppState,
    use_cases::{
        auth::login::{Login, LoginInput},
        users::register_user::{RegisterUser, RegisterUserInput},
    },
};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, StatusCode> {
    let use_case = RegisterUser::new(state.user_repo());
    let input = RegisterUserInput { username: req.username };

    let output = use_case
        .execute(input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RegisterUserResponse {
        user_id: output.user_id,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {

    debug!(username = %req.username, "login request");

    let use_case = Login::new(state.user_repo(), state.auth());
    let input = LoginInput { username: req.username };

    let output = use_case
        .execute(input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        access_token: output.access_token,
    }))
}
