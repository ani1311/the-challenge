use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendDto {
    pub user_id: String,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FriendRequestDto {
    pub request_id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub status: FriendRequestStatusDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FriendRequestStatusDto {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendFriendRequestRequest {
    pub to_user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendFriendRequestResponse {
    pub request: FriendRequestDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AcceptFriendRequestResponse {
    pub request: FriendRequestDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RejectFriendRequestResponse {
    pub request: FriendRequestDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFriendshipResponse {
    pub deleted: bool,
}
