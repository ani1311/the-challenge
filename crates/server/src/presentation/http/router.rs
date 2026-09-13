use axum::{middleware, routing::{delete, get, post}, Router};
use tower_http::trace::TraceLayer;

use crate::{
    infrastructure::jwt_service::JwtService,
    persistence::sqlx_user_repository::SqlxUserRepository,
    presentation::http::{handlers, middleware::auth::require_auth, state::AppState},
};

pub fn router(user_repo: SqlxUserRepository, auth_service: JwtService) -> Router {
    let state = AppState::new(user_repo, auth_service);

    let api_routes = public_api_router().merge(protected_api_router(state.clone()));

    Router::new()
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn public_api_router() -> Router<AppState> {
    Router::new().nest("/auth", public_auth_router())
}

fn protected_api_router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/me", me_router())
        .nest("/users", users_router())
        .nest("/friendships", friendships_router())
        .nest("/challenges", challenges_router())
        .nest("/tracking", tracking_router())
        .route_layer(middleware::from_fn_with_state(state, require_auth))
}

fn public_auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
}

fn me_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::me::get_me).patch(handlers::me::update_me))
        .route("/friends", get(handlers::me::list_my_friends))
        .route("/friend-requests", get(handlers::me::list_my_friend_requests))
        .route("/challenges", get(handlers::me::list_my_challenges))
        .route("/tracking", get(handlers::me::list_my_tracking))
}

fn users_router() -> Router<AppState> {
    Router::new()
        .route("/search", get(handlers::users::search_users))
        .route("/{user_id}", get(handlers::users::get_user))
}

fn friendships_router() -> Router<AppState> {
    Router::new()
        .route("/requests", post(handlers::friendship::send_friend_request))
        .route("/requests/{request_id}/accept", post(handlers::friendship::accept_friend_request))
        .route("/requests/{request_id}/reject", post(handlers::friendship::reject_friend_request))
        .route("/{friendship_id}", delete(handlers::friendship::delete_friendship))
}

fn challenges_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::challenges::list_challenges).post(handlers::challenges::create_challenge))
        .route("/{challenge_id}", get(handlers::challenges::get_challenge))
        .route("/{challenge_id}/join", post(handlers::challenges::join_challenge))
        .route("/{challenge_id}/complete", post(handlers::challenges::complete_challenge))
}

fn tracking_router() -> Router<AppState> {
    Router::new()
        .route("/entries", get(handlers::tracking::list_tracking_entries).post(handlers::tracking::create_tracking_entry))
}
