use common::users::{RegisterUserRequest, RegisterUserResponse};
use gloo_net::http::Request;

pub async fn register_user(name: String) -> Result<RegisterUserResponse, String> {
    let request = RegisterUserRequest { name };

    let response = Request::post("/api/users/register")
        .json(&request)
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !response.ok() {
        return Err(format!("Register failed: {}", response.status()));
    }

    response
        .json::<RegisterUserResponse>()
        .await
        .map_err(|error| error.to_string())
}
