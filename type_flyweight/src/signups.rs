// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Signup {
    pub id: u64,
    pub token: u64,
    pub contact_kind_id: u64,
    pub contact_content: String,
    pub deleted_at: Option<u64>,
}
