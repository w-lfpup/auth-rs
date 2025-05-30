// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Signup {
    pub id: u64,
    pub session: u64,
    pub session_length_ms: u64,
    pub contact_kind_id: u64,
    pub contact_content: String,
    pub deleted_at: Option<u64>,
}
