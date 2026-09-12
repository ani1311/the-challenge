use axum::{Router, routing::get};

mod domain;
mod use_cases;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hi));

    let listener = tokio::net::TcpListener::bind("127.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

}

async fn say_hi() -> &'static str{
    "Hi"
}
