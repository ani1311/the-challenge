use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChallengeDto {
    pub challenge_id: String,
    pub title: String,
    pub description: Option<String>,
    pub owner_user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateChallengeRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateChallengeResponse {
    pub challenge: ChallengeDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListChallengesResponse {
    pub challenges: Vec<ChallengeDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChallengeResponse {
    pub challenge: ChallengeDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinChallengeResponse {
    pub challenge_id: String,
    pub joined: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompleteChallengeResponse {
    pub challenge_id: String,
    pub completed: bool,
}
