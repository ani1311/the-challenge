use axum::{extract::State, http::StatusCode, Json};
use common::users::{RegisterUserRequest, RegisterUserResponse};

use crate::{
    presentation::http::{extractors::AuthContext, state::AppState},
    use_cases::users::register_user::{RegisterUser, RegisterUserInput},
};

pub async fn register_user(
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

pub async fn search_users(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn get_user(_auth: AuthContext) -> StatusCode {
    todo!()
}
