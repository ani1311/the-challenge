use serde::{Deserialize, Serialize};

use crate::{challenges::ChallengeDto, friendships::{FriendRequestDto, FriendDto}, tracking::TrackingEntryDto, users::UserDto};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetMeResponse {
    pub user: UserDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMeRequest {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMeResponse {
    pub user: UserDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMyFriendsResponse {
    pub friends: Vec<FriendDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMyFriendRequestsResponse {
    pub requests: Vec<FriendRequestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMyChallengesResponse {
    pub challenges: Vec<ChallengeDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMyTrackingResponse {
    pub entries: Vec<TrackingEntryDto>,
}
