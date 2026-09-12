use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackingEntryDto {
    pub entry_id: String,
    pub challenge_id: String,
    pub user_id: String,
    pub value: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTrackingEntryRequest {
    pub challenge_id: String,
    pub value: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTrackingEntryResponse {
    pub entry: TrackingEntryDto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListTrackingEntriesResponse {
    pub entries: Vec<TrackingEntryDto>,
}
