// This table doesn't really scale, very shallow
// Has a unique property so a general query should consider map->reduce form multiple servers

use rusqlite::{Connection, Error as RusqliteError, MappedRows, Result, Row, Rows};
use type_flyweight::contacts::ContactKind;

fn get_contact_kind_from_row(row: &Row) -> Result<ContactKind, RusqliteError> {
    Ok(ContactKind {
        id: row.get(0)?,
        kind: row.get(1)?,
        deleted_at: row.get(2)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS contact_kinds (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL UNIQUE,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("contact_kinds table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    content: &str,
) -> Result<Option<ContactKind>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO contact_kinds
            (id, kind)
        VALUES
            (?1, ?2)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut contact_kind_iter = match stmt.query_map((id, content), get_contact_kind_from_row) {
        Ok(kind_iter) => kind_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_kind_maybe) = contact_kind_iter.next() {
        if let Ok(contact_kind) = contact_kind_maybe {
            return Ok(Some(contact_kind));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<ContactKind>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contact_kinds
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut contact_kind_iter = match stmt.query_map([id], get_contact_kind_from_row) {
        Ok(contact_kind) => contact_kind,
        Err(e) => return Err(e.to_string()),
    };

    let mut contact_kinds: Vec<ContactKind> = Vec::new();
    if let Some(contact_kind_maybe) = contact_kind_iter.next() {
        if let Ok(contact_kind) = contact_kind_maybe {
            return Ok(Some(contact_kind));
        }
    }

    Ok(None)
}

pub fn read_by_kind(conn: &mut Connection, kind: &str) -> Result<Option<ContactKind>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contact_kinds
        WHERE
            deleted_at IS NULL
            AND
            kind = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut contact_kind_iter = match stmt.query_map([kind], get_contact_kind_from_row) {
        Ok(kind_iter) => kind_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_kind_maybe) = contact_kind_iter.next() {
        if let Ok(contact_kind) = contact_kind_maybe {
            return Ok(Some(contact_kind));
        }
    }

    Ok(None)
}
