// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ContactKind {
    pub id: u64,
    pub kind: String,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Contact {
    pub id: u64,
    pub kind_id: u64,
    pub content: String,
    pub verified_at: Option<u64>,
    pub deleted_at: Option<u64>,
}
