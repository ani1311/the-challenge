
use axum::{Json, extract::State, http::StatusCode};
use common::{auth::{LoginRequest, LoginResponse}, users::{RegisterUserRequest, RegisterUserResponse}};

use crate::{presentation::http::state::AppState, use_cases::{auth::login::{Login, LoginInput}, users::register_user::{RegisterUser, RegisterUserInput}}};


pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>
    ) -> Result<Json<LoginResponse>, StatusCode> {
        let login_case = Login::new(state.user_repo(), state.auth());
        let input = LoginInput{
            name: req.username
        };

        let output = login_case.execute(input).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(LoginResponse {
            access_token: "".to_string()
        }))
}
