// EMAIL INVITATIONS

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::Rng;
use rusqlite::{Connection, Result};
use snowprints::{decompose, Settings as SnowprintSettings, Snowprints};
use std::path::PathBuf;

const INVITATION_LENGTH_MS: usize = 2629800000;

pub struct Invitation {
    id: u64,
    session: u64,
    session_length_ms: usize,
    contact_type: u16,
    contact_content: String,
    completed_at: u64,
    deleted_at: u64,
}

// let snowprints = match create_snowprints(origin_time_ms, None) {
//     Ok(sp) => sp,
//     Err(e) => return Err("failed to create snowprints".to_string()),
// };

// id INTEGER PRIMARY KEY UNIQUE,
// contact_type INTEGER NOT NULL,
// contact_content TEXT KEY UNIQUE NOT NULL,
// session INTEGER NOT NULL,
// session_length_ms INTEGER NOT NULL,
// completed_at INTEGER,
// deleted_at INTEGER

pub struct InvitationsCrud {
    snowprints: Snowprints,
}

impl InvitationsCrud {
    fn new(&self, snowprint_settings: SnowprintSettings) -> Result<InvitationsCrud, String> {
        let snowprints = match Snowprints::new(snowprint_settings) {
            Ok(sp) => sp,
            Err(e) => return Err("failed to create snowprints".to_string()),
        };

        Ok(InvitationsCrud {
            snowprints: snowprints,
        })
    }

    fn create(&mut self, contact_type: u16, contact_content: String) -> Result<String, String> {
        let snowprint = match self.snowprints.compose() {
            Ok(sp) => sp,
            Err(e) => return Err("snowprint error has no default formatter".to_string()),
        };

        let (timestamp_ms, _, _) = decompose(snowprint);

        // get snowprint
        // get random 128
        // convert to hex
        // return as string
        Ok("".to_string())
    }
}

pub fn create_table(conn: Connection, path: &PathBuf) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS invitations (
            id INTEGER PRIMARY KEY UNIQUE,
            contact_type INTEGER NOT NULL,
            contact_content TEXT KEY UNIQUE NOT NULL,
            session INTEGER NOT NULL,
            session_length_ms INTEGER NOT NULL,
            completed_at INTEGER,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("invitations table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

fn create_invitation_and_session_as_base64(invitation_id: u64, session: u64) -> String {
    let mut invitation: String = "".to_string();

    invitation.push_str(&URL_SAFE.encode(invitation_id.to_ne_bytes()));
    invitation.push(':');
    invitation.push_str(&URL_SAFE.encode(session.to_ne_bytes()));

    invitation
}

fn get_arry_u8(data_vec: Vec<u8>) -> Result<[u8; 8], String> {
    if 8 != data_vec.len() {
        return Err("required length not found".to_string());
    }

    let mut data: [u8; 8] = [0; 8];
    let mut index = 0;
    for pip in data {
        data[index] = pip;
        index += 1;
    }

    Ok(data)
}

fn get_invitation_and_session_from_base64(invitation_base64: &str) -> Result<(u64, u64), String> {
    let mut splitted = invitation_base64.split(":");

    let mut invitation_u64: Option<u64> = None;
    if let Some(invitation_base64) = splitted.next() {
        if let Ok(invitation_vec_bytes) = URL_SAFE.decode(invitation_base64.as_bytes()) {
            if let Ok(invitation_arr) = get_arry_u8(invitation_vec_bytes) {
                invitation_u64 = Some(u64::from_ne_bytes(invitation_arr));
            }
        }
    }

    let mut session_u64: Option<u64> = None;
    if let Some(session_base64) = splitted.next() {
        if let Ok(session_vec_bytes) = URL_SAFE.decode(session_base64.as_bytes()) {
            if let Ok(session_arr) = get_arry_u8(session_vec_bytes) {
                session_u64 = Some(u64::from_ne_bytes(session_arr));
            }
        };
    }

    if let (Some(invitation), Some(session)) = (invitation_u64, session_u64) {
        return Ok((invitation, session));
    }

    Err("didnt' make it!".to_string())
}

pub fn create(
    conn: Connection,
    invitation_id: u64,
    contact_type: u16,
    contact_content: &str,
    session_length_ms: u32,
) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let session: u64 = rng.gen();

    let results = conn.execute(
        "INSERT INTO invitations
            (id, contact_type, contact_content, session, session_length_ms)
        VALUES
            (?1, ?2, ?3, ?4, ?5)",
        (
            invitation_id,
            contact_type,
            contact_content,
            session,
            session_length_ms,
        ),
    );

    if let Err(e) = results {
        return Err("create invitations: \n".to_string() + &e.to_string());
    }

    let invitation = create_invitation_and_session_as_base64(invitation_id, session);

    Ok(invitation)
}

pub fn read(conn: Connection, invitation_session: &str) -> Result<Option<()>, String> {
    let (invitation_id, session) = match get_invitation_and_session_from_base64(invitation_session)
    {
        Ok((inv, sess)) => (inv, sess),
        _ => return Err("could not get invitation and id from base64".to_string()),
    };

    let mut stmt = match conn.prepare(
        "SELECT invitations
        WHERE id = ?1",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    // this can be a separate function
    let rows = stmt.query_map([invitation_id], |row| {
        // get values
        // add it to struct
        // return
        Ok(())
    });

    // iterate return

    Ok(None)
}

// pub fn delete(path: &PathBuf, session_id: u64, timestamp_ms: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (invitations table)".to_string()),
//     };

//     let results = conn.execute(
//         "UPDATE invitations
//         SET deleted_at = ?1
//         WHERE id = ?2",
//         (timestamp_ms, session_id),
//     );

//     if let Err(e) = results {
//         return Err("delete invitations: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// Invitations Maintenance

// pub fn dangerously_delete(path: &PathBuf, people_id: u64, timestamp_ms: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (invitations table)".to_string()),
//     };

//     let results = conn.execute(
//         "DELETE invitations
//         WHERE id = ?1",
//         [people_id],
//     );

//     if let Err(e) = results {
//         return Err("dangerously delete invitations: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }
