use axum::{Router, routing::post};

use crate::{infrastructure::jwt_service::JwtService, persistence::sqlx_user_repository::SqlxUserRepository, presentation::http::{handlers, state::AppState}, use_cases::ports::UserRepository};

pub fn router(user_repo: SqlxUserRepository, auth_service: JwtService) -> Router {
    let state = AppState::new(user_repo, auth_service);

    Router::new().nest("/api", api_router()).with_state(state)
}

pub fn api_router() -> Router<AppState> {
    Router::new().nest("/users", users_router()).nest("/auth", auth_router())
}

pub fn users_router() -> Router<AppState> {
    Router::new().route("/register", post(handlers::users::register_user))
        .route("/me", method_router)
}

pub fn auth_router() -> Router<AppState> {
    Router::new().route("/login", post(handlers::auth::login))
}
