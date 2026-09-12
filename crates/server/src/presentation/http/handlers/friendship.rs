use axum::http::StatusCode;

use crate::presentation::http::extractors::AuthContext;

pub async fn send_friend_request(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn accept_friend_request(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn reject_friend_request(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn delete_friendship(_auth: AuthContext) -> StatusCode {
    todo!()
}
