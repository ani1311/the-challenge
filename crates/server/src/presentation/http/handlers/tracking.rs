use axum::http::StatusCode;

use crate::presentation::http::extractors::AuthContext;

pub async fn create_tracking_entry(_auth: AuthContext) -> StatusCode {
    todo!()
}

pub async fn list_tracking_entries(_auth: AuthContext) -> StatusCode {
    todo!()
}
