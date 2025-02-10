use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct Person {}

// keep table creation out of regular api?
pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (people)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
			id INTEGER PRIMARY KEY,
			password_hash_params TEXT NOT NULL,
			deleted_at INTEGER
		)",
        (),
    );

    if let Err(e) = results {
        return Err("people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(path: &PathBuf, people_id: u64, password_hash_params: &str) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (people)".to_string()),
    };

    let results = conn.execute(
        "INSERT OR IGNORE INTO people
            (id, password_hash_params)
        VALUES
            (?1, ?2)",
        (people_id, password_hash_params),
    );

    if let Err(e) = results {
        return Err("people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read(path: &PathBuf, people_id: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (people)".to_string()),
    };

    let results = conn.execute(
        "SELECT people
        WHERE id = ?1",
        [people_id],
    );

    if let Err(e) = results {
        return Err("read people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn update(path: &PathBuf, people_id: u64, password_hash_params: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (people)".to_string()),
    };

    let results = conn.execute(
        "UPDATE people
        SET password_hash_params = ?1
        WHERE id = ?2",
        (password_hash_params, people_id),
    );

    if let Err(e) = results {
        return Err("update people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn delete(path: &PathBuf, people_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (people)".to_string()),
    };

    let results = conn.execute(
        "UPDATE people
        SET deleted_at = ?1
        WHERE id = ?2",
        (timestamp_ms, people_id),
    );

    if let Err(e) = results {
        return Err("delete people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn dangerously_delete(path: &PathBuf, people_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (people)".to_string()),
    };

    let results = conn.execute(
        "DELETE people
        WHERE id = ?1",
        [people_id],
    );

    if let Err(e) = results {
        return Err("dangerously delete people: \n".to_string() + &e.to_string());
    }

    Ok(())
}
