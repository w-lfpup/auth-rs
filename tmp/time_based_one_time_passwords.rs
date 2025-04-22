use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct TimebasedOneTimePassword {}

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS timebased_one_time_passwords (
            id INTEGER PRIMARY KEY,
            people_id INTEGER KEY NOT NULL,
            secret TEXT NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("emails_to_emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(path: &PathBuf, email_id: u64, people_id: u64, email: &str) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "INSERT INTO emails
            (id, people_id, email)
        VALUES
            (?1, ?2, ?3)",
        (email_id, people_id, email),
    );

    if let Err(e) = results {
        return Err("emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read(path: &PathBuf, email_id: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "SELECT emails
        WHERE id = ?1",
        [email_id],
    );

    // iterate through emails

    if let Err(e) = results {
        return Err("read emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read_by_email(path: &PathBuf, email: &str) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "SELECT emails
        WHERE email = ?1",
        [email],
    );

    // iterate through emails

    if let Err(e) = results {
        return Err("read emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn delete(path: &PathBuf, email_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "UPDATE emails
        SET deleted_at = ?1
        WHERE id = ?2",
        (timestamp_ms, email_id),
    );

    if let Err(e) = results {
        return Err("delete emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn dangerously_delete(path: &PathBuf, email_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (emails)".to_string()),
    };

    let results = conn.execute(
        "DELETE emails
        WHERE id = ?1",
        [email_id],
    );

    if let Err(e) = results {
        return Err("dangerously delete emails: \n".to_string() + &e.to_string());
    }

    Ok(())
}
