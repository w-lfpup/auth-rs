// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Signups {
    id: u64,
    session: u64,
    session_length_ms: u64,
    contact_type: u64,
    contact_content: String,
    deleted_at: Option<u64>,
}
