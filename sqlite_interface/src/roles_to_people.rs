use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::roles::RoleToPerson;

// Table has unique constraint on role_id and CONTENT
// This requires queries at scale to "get" all possible from all shards
// if searching by RoleToPerson

fn get_role_to_people_from_row(row: &Row) -> Result<RoleToPerson, RusqliteError> {
    Ok(RoleToPerson {
        id: row.get(0)?,
        role_id: row.get(2)?,
        people_id: row.get(1)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS roles_to_people (
            id INTEGER PRIMARY KEY,
            role_id INTEGER NOT NULL,
            people_id INTEGER NOT NULL,
            deleted_at INTEGER,
            UNIQUE (role_id, people_id)
        )",
        (),
    );

    if let Err(e) = results {
        return Err("roles_to_people table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    role_id: u64,
    people_id: u64,
) -> Result<Option<RoleToPerson>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO roles_to_people
            (id, role_id, people_id)
        VALUES
            (?1, ?2, ?3)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to create a RoleToPerson".to_string()),
    };

    let mut role_to_people_iter =
        match stmt.query_map((id, role_id, people_id), get_role_to_people_from_row) {
            Ok(role_to_people_iter) => role_to_people_iter,
            Err(e) => return Err(e.to_string()),
        };

    if let Some(role_to_people_maybe) = role_to_people_iter.next() {
        if let Ok(role_to_people) = role_to_people_maybe {
            return Ok(Some(role_to_people));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<RoleToPerson>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            roles_to_people
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read a RoleToPerson entry".to_string()),
    };

    let mut role_to_people_iter = match stmt.query_map([id], get_role_to_people_from_row) {
        Ok(role_to_people_iter) => role_to_people_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(role_to_people_maybe) = role_to_people_iter.next() {
        if let Ok(role_to_people) = role_to_people_maybe {
            return Ok(Some(role_to_people));
        }
    }

    Ok(None)
}

pub fn read_by_role_id_and_people_id(
    conn: &mut Connection,
    role_id: u64,
    people_id: &u64,
) -> Result<Option<RoleToPerson>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            roles_to_people
        WHERE
            deleted_at IS NULL
            AND
            role_id = ?1
            AND
            people_id = ?2
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read a RoleToPerson by id".to_string()),
    };

    let mut role_to_people_iter =
        match stmt.query_map((role_id, people_id), get_role_to_people_from_row) {
            Ok(role_to_people_iter) => role_to_people_iter,
            Err(e) => return Err(e.to_string()),
        };

    if let Some(role_to_people_maybe) = role_to_people_iter.next() {
        if let Ok(role_to_people) = role_to_people_maybe {
            return Ok(Some(role_to_people));
        }
    }

    Ok(None)
}
