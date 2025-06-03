use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::contacts::ContactKind;

// This table doesn't really scale, very shallow

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
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut contact_kind = match stmt.query_map([id], get_contact_kind_from_row) {
        Ok(contact_kind) => contact_kind,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(contact_kind_maybe) = contact_kind.next() {
        if let Ok(contact_kind) = contact_kind_maybe {
            return Ok(Some(contact_kind));
        }
    }

    Ok(None)
}

pub fn read_by_kind(conn: &mut Connection, kind: u64) -> Result<Option<ContactKind>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            contact_kinds
        WHERE
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

// pub fn delete(
//     conn: &mut Connection,
//     contact_kind_id: u64,
//     deleted_at: u64,
// ) -> Result<Option<ContactKind>, String> {
//     let mut stmt = match conn.prepare(
//         "
//         UPDATE
//             contact_kind
//         SET
//             deleted_at = ?1
//         WHERE
//             id = ?2
//         RETURNING
//             *
//     ",
//     ) {
//         Ok(stmt) => stmt,
//         _ => return Err("cound not prepare statement".to_string()),
//     };

//     let mut contact_kind = match stmt.query_map((deleted_at, contact_kind_id), get_contact_kind_from_row)
//     {
//         Ok(contact_kind) => contact_kind,
//         Err(e) => return Err(e.to_string()),
//     };

//     if let Some(contact_kind_maybe) = contact_kind.next() {
//         if let Ok(invitation) = contact_kind_maybe {
//             return Ok(Some(invitation));
//         }
//     }

//     Ok(None)
// }

// pub fn dangerously_delete(
//     conn: &mut Connection,
//     contact_kind_id: u64,
// ) -> Result<Option<ContactKind>, String> {
//     let mut stmt = match conn.prepare(
//         "
//         DELETE
//             contact_kind
//         WHERE
//             id = ?1
//         RETURNING
//             *
//     ",
//     ) {
//         Ok(stmt) => stmt,
//         _ => return Err("cound not prepare statement".to_string()),
//     };

//     let mut contact_kind = match stmt.query_map([contact_kind_id], get_contact_kind_from_row) {
//         Ok(contact_kind) => contact_kind,
//         Err(e) => return Err(e.to_string()),
//     };

//     if let Some(contact_kind_maybe) = contact_kind.next() {
//         if let Ok(invitation) = contact_kind_maybe {
//             return Ok(Some(invitation));
//         }
//     }

//     Ok(None)
// }
