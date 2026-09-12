use crate::persistence::sqlx_user_repository::SqlxUserRepository;

mod domain;
mod use_cases;
mod presentation;
mod persistence;

#[tokio::main]
async fn main() {
    let user_repo = SqlxUserRepository::new();
    let app = presentation::http::router::router(user_repo);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

}
