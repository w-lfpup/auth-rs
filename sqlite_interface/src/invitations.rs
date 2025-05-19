// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand::Rng;
use rusqlite::{Connection, Result};
use snowprints::{decompose, Settings as SnowprintSettings, Snowprints};
use std::path::PathBuf;

const INVITATION_LENGTH_MS: usize = 2629800000;

#[derive(Clone, Serialize, Deserialize, Debug)]
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

pub fn create(
    conn: Connection,
    invitation_id: u64,
    contact_type: u16,
    contact_content: &str,
    session_length_ms: u32,
) -> Result<Option<Invitation>, String> {
    let mut rng = rand::thread_rng();
    let session: u64 = rng.gen();

    let mut stmt = match conn.prepare(
        "
        INSERT INTO invitations
            (id, contact_type, contact_content, session, session_length_ms)
        VALUES
            (?1, ?2, ?3, ?4, ?5)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut invitations = match stmt.query_map(
        (
            invitation_id,
            contact_type,
            contact_content,
            session,
            session_length_ms,
        ),
        |row| {
            Ok(Invitation {
                id: row.get(0)?,
                session: row.get(1)?,
                session_length_ms: row.get(2)?,
                contact_type: row.get(3)?,
                contact_content: row.get(4)?,
                completed_at: row.get(5)?,
                deleted_at: row.get(6)?,
            })
        },
    ) {
        Ok(invitations) => invitations,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(invitation_maybe) = invitations.next() {
        if let Ok(invitation) = invitation_maybe {
            return Ok(Some(invitation));
        }
    }

    Ok(None)
}

pub fn read(conn: Connection, invitation_id: u64) -> Result<Option<Invitation>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            invitations
        WHERE
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut invitations = match stmt.query_map([invitation_id], |row| {
        Ok(Invitation {
            id: row.get(0)?,
            session: row.get(1)?,
            session_length_ms: row.get(2)?,
            contact_type: row.get(3)?,
            contact_content: row.get(4)?,
            completed_at: row.get(5)?,
            deleted_at: row.get(6)?,
        })
    }) {
        Ok(invitations) => invitations,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(invitation_maybe) = invitations.next() {
        if let Ok(invitation) = invitation_maybe {
            return Ok(Some(invitation));
        }
    }

    Ok(None)
}

pub fn dangerously_delete(
    conn: Connection,
    invitation_id: u64,
) -> Result<Option<Invitation>, String> {
    let mut stmt = match conn.prepare(
        "DELETE
            invitations
        WHERE
            id = ?1
        RETURNING
            *",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut invitations = match stmt.query_map([invitation_id], |row| {
        Ok(Invitation {
            id: row.get(0)?,
            session: row.get(1)?,
            session_length_ms: row.get(2)?,
            contact_type: row.get(3)?,
            contact_content: row.get(4)?,
            completed_at: row.get(5)?,
            deleted_at: row.get(6)?,
        })
    }) {
        Ok(invitations) => invitations,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(invitation_maybe) = invitations.next() {
        if let Ok(invitation) = invitation_maybe {
            return Ok(Some(invitation));
        }
    }

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
