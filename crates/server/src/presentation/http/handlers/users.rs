use axum::{Json, extract::State, http::StatusCode};
use common::users::{RegisterUserRequest, RegisterUserResponse};

use crate::{presentation::http::state::AppState, use_cases::users::register_user::{RegisterUser, RegisterUserInput}};


pub async fn register_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserRequest>
    ) -> Result<Json<RegisterUserResponse>, StatusCode> {
        let user_case = RegisterUser::new(state.user_repo());
        let input = RegisterUserInput {
            name: req.name
        };

        let output = user_case.execute(input).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(RegisterUserResponse{
            user_id: output.user_id
        }))
}
