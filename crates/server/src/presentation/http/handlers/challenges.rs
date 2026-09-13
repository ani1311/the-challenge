use axum::http::StatusCode;

use crate::presentation::http::extractors::AuthContext;

pub async fn create_challenge(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn list_challenges(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn get_challenge(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn join_challenge(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn complete_challenge(_auth: AuthContext) -> StatusCode {
    todo!()
}
