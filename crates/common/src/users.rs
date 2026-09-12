use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub user_id: String,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterUserResponse {
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchUsersRequest {
    pub q: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchUsersResponse {
    pub users: Vec<UserDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: UserDto,
}
