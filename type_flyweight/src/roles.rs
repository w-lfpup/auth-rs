// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: u64,
    pub kind: String,
    pub deleted_at: Option<u64>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct RoleToPerson {
    pub id: u64,
    pub role_id: u64,
    pub people_id: u64,
    pub deleted_at: Option<u64>,
}
