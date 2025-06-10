// create invitation from

// create invitation
// input contact kind, contact content, get base64:base64

// create people entry
// input base64:base64, password, create contact type, create people entry

// EMAIL INVITATIONS
use std::sync::{Arc, Mutex};

use base64::engine::general_purpose::URL_SAFE;
use rand::Rng;
use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use snowprints::Snowprints;

// 1 DAY as ms
const INVITATION_LENGTH_MS: usize = 2629800000;

// create()
// returns hexidecimal string
//

// RETURNS NONE when contact already exists
pub fn create_signup_session(
    snowrpints: Arc<Mutex<Snowprints>>,
    conn: &mut Connection,
    contact_kind: &str,
    contact_content: &str,
) -> Result<Option<String>, String> {
    // get contact kind

    // see if contact already exists

    // return none if contact already exists

    // create snowprint

    // create session

    // create token

    // create signup with snowprint, token, contact kind, contact content

    // create signup session based on snowprint::token

    Err("failed to create signup session".to_string())
}

// potential results
// contact is occupied (maybe reset password?)
// contact is verified ()
// invitation has been used (people_id matches invitation_id)
// success

pub fn create_person_and_contact_from_signup_session(
    conn: &mut Connection,
    session_base64: &str,
    password: &str,
    session_length_ms: u64,
) -> Result<(), String> {
    // get id, session
    // query for invitation
    // if session return

    // if some then check if session matches session
    // if session matches return Session

    // if ok return otherwise
    Err("failed to create person from signup session".to_string())
}
