// EMAIL INVITATIONS
use serde::{Deserialize, Serialize};

use base64::engine::general_purpose::URL_SAFE;
use rand::Rng;
use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use std::path::PathBuf;

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

fn get_invitation_from_row(row: &Row) -> Result<Invitation, RusqliteError> {
    Ok(Invitation {
        id: row.get(0)?,
        session: row.get(1)?,
        session_length_ms: row.get(2)?,
        contact_type: row.get(3)?,
        contact_content: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS invitations (
            id INTEGER PRIMARY KEY UNIQUE,
            session INTEGER NOT NULL,
            session_length_ms INTEGER NOT NULL,
            contact_type INTEGER NOT NULL,
            contact_content TEXT KEY NOT NULL,
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
    contact_type: u64,
    contact_content: &str,
    session_length_ms: u32,
) -> Result<Option<Invitation>, String> {
    let mut rng = rand::rng();
    let session: u64 = rng.random();

    let mut stmt = match conn.prepare(
        "
        INSERT INTO invitations
            (id, session, session_length_ms, contact_type, contact_content)
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
            session,
            session_length_ms,
            contact_type,
            contact_content,
        ),
        get_invitation_from_row,
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

    let mut invitations = match stmt.query_map([invitation_id], get_invitation_from_row) {
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

pub fn delete(
    conn: Connection,
    invitation_id: u64,
    deleted_at: u64,
) -> Result<Option<Invitation>, String> {
    let mut stmt = match conn.prepare(
        "
        UPDATE
            invitations
        SET
            deleted_at = ?1
        WHERE
            id = ?2
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut invitations = match stmt.query_map((deleted_at, invitation_id), get_invitation_from_row)
    {
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
        "
        DELETE
            invitations
        WHERE
            id = ?1
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut invitations = match stmt.query_map([invitation_id], get_invitation_from_row) {
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
