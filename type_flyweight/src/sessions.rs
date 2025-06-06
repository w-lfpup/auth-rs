// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: u64,
    pub people_id: Option<u64>,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PublicSession {
    pub id: u64,
    pub people_id: Option<u64>,
    pub token: u64,
    pub session_id: u64,
    pub deleted_at: Option<u64>,
}
