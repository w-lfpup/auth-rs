// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

use base64::engine::general_purpose::URL_SAFE;
use rand::Rng;
use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use crate::invitations::Invitation;

// 1 DAY as ms
const INVITATION_LENGTH_MS: usize = 2629800000;

// create()
// returns hexidecimal string
pub fn create_base64(
    conn: Connection,
    id: u64,
    contact_type: u64,
    contact_content: &str,
) -> Result<Option<String>, String> {
    // get snowprint
    // get session length
    // create db entry
    // turn id and session into base64:base64

    Ok(None)
}

pub fn get_by_base64(conn: Connection, session_base64: &str) -> Result<Option<Invitation>, String> {
    // get id, session
    // query for invitation
    // if session return

    // if some then check if session matches session
    // if session matches return Session

    // if ok return otherwise
    Ok(None)
}

pub fn get_all_by_contact(
    conn: Connection,
    contact_type: u64,
    contact_content: &str,
) -> Result<Option<Vec<Invitation>>, String> {
    // query by type, content
    //
    Ok(None)
}
