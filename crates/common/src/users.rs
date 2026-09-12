use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub name: String,
}


#[derive(Serialize, Deserialize)]
pub struct RegisterUserResponse{
    pub user_id: String
}
