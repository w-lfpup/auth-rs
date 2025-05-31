// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Person {
    pub id: u64,
    pub password_hash_results: String,
    pub deleted_at: Option<u64>,
}
