// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

// 1 DAY as ms
const INVITATION_LENGTH_MS: usize = 2629800000;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Invitation {
    id: u64,
    session: u64,
    session_length_ms: u64,
    contact_type: u64,
    contact_content: String,
    deleted_at: Option<u64>,
}
