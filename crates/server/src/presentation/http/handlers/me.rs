use axum::{extract::State, http::StatusCode, Json};
use common::{me::GetMeResponse, users::UserDto};

use crate::{
    presentation::http::{extractors::AuthContext, state::AppState},
    use_cases::users::get_current_user::{GetCurrentUserInput, GetMe},
};

pub async fn get_me(
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<Json<GetMeResponse>, StatusCode> {
    let use_case = GetMe::new(state.user_repo());
    let input = GetCurrentUserInput {
        user_id: auth.user_id,
    };

    let output = use_case
        .execute(input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(GetMeResponse {
        user: UserDto {
            user_id: output.user.id(),
            username: output.user.username(),
        },
    }))
}

pub async fn update_me(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn list_my_friends(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn list_my_friend_requests(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn list_my_challenges(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn list_my_tracking(_auth: AuthContext) -> StatusCode {
    todo!()
}
