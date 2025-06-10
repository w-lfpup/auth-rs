// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ActionKind {
    pub id: u64,
    pub kind: String,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PeopleActionRateLimit {
    pub people_id: u64,
    pub kind_id: u64,
    pub window_count: u64,
    pub prev_window_count: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,
}
