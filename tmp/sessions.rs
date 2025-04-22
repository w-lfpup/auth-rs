use rand::Rng;
use rusqlite::{Connection, Result};
use std::path::PathBuf;

const GUEST_SESSION_LENGTH_MS: usize = 2629800000;
const USER_SESSION_LENGTH_MS: usize = 7889400000;

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY UNIQUE,
            people_id INTEGER KEY,
            session INTEGER NOT NULL,
            session_length_ms INTEGER NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("sessions table: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    path: &PathBuf,
    session_id: u64,
    session_length_ms: u64,
    people_id: Option<u64>,
) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (sessions)".to_string()),
    };

    let mut rng = rand::thread_rng();
    let session: u64 = rng.gen();

    let results = conn.execute(
        "INSERT INTO sessions
        	(id, people_id, session, session_length_ms)
        VALUES
        	(?1, ?2, ?3, ?4)",
        (session_id, people_id, session, session_length_ms),
    );

    if let Err(e) = results {
        return Err("create sessions: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read(path: &PathBuf, session_id: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (sessions)".to_string()),
    };

    let results = conn.execute(
        "SELECT sessions
        WHERE id = ?1",
        [session_id],
    );

    // iterate return

    if let Err(e) = results {
        return Err("read sessions: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn delete(path: &PathBuf, session_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (sessions)".to_string()),
    };

    let results = conn.execute(
        "UPDATE sessions
        SET deleted_at = ?1
        WHERE id = ?2",
        (timestamp_ms, session_id),
    );

    if let Err(e) = results {
        return Err("delete sessions: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn dangerously_delete(path: &PathBuf, people_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (sessions)".to_string()),
    };

    let results = conn.execute(
        "DELETE sessions
        WHERE id = ?1",
        [people_id],
    );

    if let Err(e) = results {
        return Err("dangerously delete sessions: \n".to_string() + &e.to_string());
    }

    Ok(())
}
