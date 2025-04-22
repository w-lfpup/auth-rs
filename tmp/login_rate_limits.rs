// Named IP rate limits
// but essentially means
// "session creation rate limits"
//
// This prevents ip addresses from spamming resources
// specifically for sessions
//

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
        "CREATE TABLE IF NOT EXISTS login_rate_limits (
			id INTEGER PRIMARY KEY,
            people_id INTEGER KEY UNIQUE,
			bucket_alpha INTEGER,
			bucket_omega INTEGER,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("login_rate_limits table: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    path: &PathBuf,
    ip_address: &str,
) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (login_rate_limits)".to_string()),
    };

    let mut rng = rand::thread_rng();
    let session: u64 = rng.gen();

    let results = conn.execute(
        "INSERT INTO login_rate_limits
        	(id, bucket_alpha, bucket_omega)
        VALUES
        	(?1, 0, 0)",
        [ip_address],
    );

    if let Err(e) = results {
        return Err("create login_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read(path: &PathBuf, ip_address: &str) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (login_rate_limits)".to_string()),
    };

    let results = conn.execute(
        "SELECT login_rate_limits
        WHERE ip_address = ?1",
        [ip_address],
    );

    // iterate return

    if let Err(e) = results {
        return Err("read login_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn update(path: &PathBuf, ip_address: &str, bucket_alpha: u64, bucket_omega: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (sessions)".to_string()),
    };

    let results = conn.execute(
        "UPDATE login_rate_limits
        SET
            bucket_alpha = ?1
            AND
            bucket_omega = ?2
        WHERE id = ?3",
        (bucket_alpha, bucket_omega, ip_address),
    );

    if let Err(e) = results {
        return Err("update sessions: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn delete(path: &PathBuf, ip_address: &str, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (login_rate_limits)".to_string()),
    };

    let results = conn.execute(
        "UPDATE login_rate_limits
        SET deleted_at = ?1
        WHERE ip_address = ?2",
        (timestamp_ms, ip_address),
    );

    if let Err(e) = results {
        return Err("delete login_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn dangerously_delete(path: &PathBuf, ip_address: &str, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (login_rate_limits)".to_string()),
    };

    let results = conn.execute(
        "DELETE login_rate_limits
        WHERE ip_address = ?1",
        [ip_address],
    );

    if let Err(e) = results {
        return Err("dangerously delete login_rate_limits: \n".to_string() + &e.to_string());
    }

    Ok(())
}
