use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub struct RoleToPerson {}

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (roles_to_people)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS roles_to_people (
            id INTEGER PRIMARY KEY,
            role_id TEXT NOT NULL,
            people_id INTEGER NOT NULL,
            deleted_at INTEGER,
            UNIQUE(role_id, people_id)
        )",
        (),
    );

    if let Err(e) = results {
        return Err("roles_to_people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    path: &PathBuf,
    roles_to_people_id: u64,
    role_id: u64,
    people_id: u64,
) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (roles_to_people)".to_string()),
    };

    let results = conn.execute(
        "INSERT INTO roles_to_people
            (id, role_id, people_id)
        VALUES
            (?1, ?2, ?3)",
        (roles_to_people_id, role_id, people_id),
    );

    if let Err(e) = results {
        return Err("roles_to_people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn read(path: &PathBuf, people_id: u64, role_id: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (roles_to_people)".to_string()),
    };

    let results = conn.execute(
        "SELECT roles_to_people
        WHERE
            people_id = ?1
            AND
            role_id = ?2
        ",
        (people_id, role_id),
    );

    // iterate through roles_to_people

    if let Err(e) = results {
        return Err("read roles_to_people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn delete(path: &PathBuf, roles_to_people_id: u64, timestamp_ms: u64) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (roles_to_people)".to_string()),
    };

    let results = conn.execute(
        "UPDATE roles_to_people
        SET deleted_at = ?1
        WHERE id = ?2",
        (timestamp_ms, roles_to_people_id),
    );

    if let Err(e) = results {
        return Err("delete roles_to_people: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn dangerously_delete(
    path: &PathBuf,
    roles_to_people_id: u64,
    timestamp_ms: u64,
) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (roles_to_people)".to_string()),
    };

    let results = conn.execute(
        "DELETE roles_to_people
        WHERE id = ?1",
        [roles_to_people_id],
    );

    if let Err(e) = results {
        return Err("dangerously delete roles_to_people: \n".to_string() + &e.to_string());
    }

    Ok(())
}
