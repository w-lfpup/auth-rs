use rand::Rng;
use rusqlite::{Connection, Result};
use std::path::PathBuf;


// Session rate limits are a 1:1 table relation
// The "id" of the table is the session id.
// This way, the id can retain a timestamp for deletion.


pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS session_rate_limits (
            session_id INTEGER PRIMARY KEY UNIQUE,
			bucket_alpha INTEGER,
			bucket_omega INTEGER,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("session_rate_limits table: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    path: &PathBuf,
    session_id: u64,
) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session_rate_limits)".to_string()),
    };

    let mut rng = rand::thread_rng();
    let session: u64 = rng.gen();

    let results = conn.execute(
        "INSERT INTO session_rate_limits
        	(id, bucket_alpha, bucket_omega)
        VALUES
        	(?1, 0, 0)",
        [session_id],
    );

    if let Err(e) = results {
        return Err("create session_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read(path: &PathBuf, session_id: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session_rate_limits)".to_string()),
    };

    let results = conn.execute(
        "SELECT session_rate_limits
        WHERE session_id = ?1",
        [session_id],
    );

    // iterate return

    if let Err(e) = results {
        return Err("read session_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn update(path: &PathBuf, session_id: u64, bucket_alpha: u64, bucket_omega: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (sessions)".to_string()),
    };

    let results = conn.execute(
        "UPDATE session_rate_limits
        SET
            bucket_alpha = ?1
            AND
            bucket_omega = ?2
        WHERE id = ?3",
        (bucket_alpha, bucket_omega, session_id),
    );

    if let Err(e) = results {
        return Err("update sessions: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn delete(path: &PathBuf, session_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session_rate_limits)".to_string()),
    };

    let results = conn.execute(
        "UPDATE session_rate_limits
        SET deleted_at = ?1
        WHERE session_id = ?2",
        (timestamp_ms, session_id),
    );

    if let Err(e) = results {
        return Err("delete session_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn dangerously_delete(path: &PathBuf, session_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session_rate_limits)".to_string()),
    };

    let results = conn.execute(
        "DELETE session_rate_limits
        WHERE session_id = ?1",
        [session_id],
    );

    if let Err(e) = results {
        return Err("dangerously delete session_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}
