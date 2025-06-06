use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::contacts::Contact;

// Table has unique constraint on CONTACT_KIND_ID and CONTENT
// This requires queries at scale to "get" all possible from all shards
// if searching by contact

fn get_contact_from_row(row: &Row) -> Result<Contact, RusqliteError> {
    Ok(Contact {
        id: row.get(0)?,
        people_id: row.get(1)?,
        contact_kind_id: row.get(2)?,
        content: row.get(3)?,
        verified_at: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY,
            people_id INTEGER NOT NULL,
            contact_kind_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            verified_at INTEGER,
            deleted_at INTEGER,
            UNIQUE (contact_kind_id, content)
        )",
        (),
    );

    if let Err(e) = results {
        return Err("contacts table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    people_id: u64,
    contact_kind_id: u64,
    content: &str,
    verified_at: Option<u64>,
) -> Result<Option<Contact>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO contacts
            (id, people_id, contact_kind_id, content, verified_at)
        VALUES
            (?1, ?2, ?3, ?4, ?5)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to create a contact".to_string()),
    };

    let mut contact_iter = match stmt.query_map(
        (id, people_id, contact_kind_id, content, verified_at),
        get_contact_from_row,
    ) {
        Ok(contact_iter) => contact_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_maybe) = contact_iter.next() {
        if let Ok(contact) = contact_maybe {
            return Ok(Some(contact));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<Contact>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contacts
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read a contact".to_string()),
    };

    let mut contact_iter = match stmt.query_map([id], get_contact_from_row) {
        Ok(contact_iter) => contact_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_maybe) = contact_iter.next() {
        if let Ok(contact) = contact_maybe {
            return Ok(Some(contact));
        }
    }

    Ok(None)
}

pub fn read_by_kind_id_and_content(
    conn: &mut Connection,
    contact_kind_id: u64,
    content: &str,
) -> Result<Option<Contact>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contacts
        WHERE
            deleted_at IS NULL
            AND
            contact_kind_id = ?1
            AND
            content = ?2
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("failed to read a contact by id".to_string()),
    };

    let mut contact_iter = match stmt.query_map((contact_kind_id, content), get_contact_from_row) {
        Ok(contact_iter) => contact_iter,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_maybe) = contact_iter.next() {
        if let Ok(contact) = contact_maybe {
            return Ok(Some(contact));
        }
    }

    Ok(None)
}
