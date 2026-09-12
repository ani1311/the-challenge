use axum::{Router, routing::post};

use crate::{persistence::sqlx_user_repository::SqlxUserRepository, presentation::http::{handlers, state::AppState}, use_cases::ports::UserRepository};

pub fn router(user_repo: SqlxUserRepository) -> Router {
    let state = AppState::new(user_repo);

    Router::new().nest("/api", api_router()).with_state(state)
}

pub fn api_router() -> Router<AppState> {
    Router::new().nest("/users", users_router())
}

pub fn users_router() -> Router<AppState> {
    Router::new().route("/register", post(handlers::register_user))
}
