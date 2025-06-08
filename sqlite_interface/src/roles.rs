// This table doesn't really scale, very shallow
// Has a unique property so a general query should consider map->reduce form multiple servers

use rusqlite::{Connection, Error as RusqliteError, MappedRows, Result, Row, Rows};
use type_flyweight::roles::Role;

fn get_role_from_row(row: &Row) -> Result<Role, RusqliteError> {
    Ok(Role {
        id: row.get(0)?,
        kind: row.get(1)?,
        deleted_at: row.get(2)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL UNIQUE,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("roles table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(conn: &mut Connection, id: u64, kind: &str) -> Result<Option<Role>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO roles
            (id, kind)
        VALUES
            (?1, ?2)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare create statement".to_string()),
    };

    let mut role_iter = match stmt.query_map((id, kind), get_role_from_row) {
        Ok(kind_iter) => kind_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(role_maybe) = role_iter.next() {
        if let Ok(role) = role_maybe {
            return Ok(Some(role));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<Role>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            roles
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare read statement".to_string()),
    };

    let mut role_iter = match stmt.query_map([id], get_role_from_row) {
        Ok(role) => role,
        Err(e) => return Err(e.to_string()),
    };

    let mut roles: Vec<Role> = Vec::new();
    if let Some(role_maybe) = role_iter.next() {
        if let Ok(role) = role_maybe {
            return Ok(Some(role));
        }
    }

    Ok(None)
}

pub fn read_by_kind(conn: &mut Connection, kind: &str) -> Result<Option<Role>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            roles
        WHERE
            deleted_at IS NULL
            AND
            kind = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare read_by_kind statement".to_string()),
    };

    let mut role_iter = match stmt.query_map([kind], get_role_from_row) {
        Ok(kind_iter) => kind_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(role_maybe) = role_iter.next() {
        if let Ok(role) = role_maybe {
            return Ok(Some(role));
        }
    }

    Ok(None)
}
