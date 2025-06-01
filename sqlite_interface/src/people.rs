use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::people::Person;

// This table doesn't really scale, very shallow

fn get_person_from_row(row: &Row) -> Result<Person, RusqliteError> {
    Ok(Person {
        id: row.get(0)?,
        password_hash_results: row.get(1)?,
        deleted_at: row.get(2)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
            id INTEGER PRIMARY KEY,
            password_hash_results TEXT NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("people table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    password_hash_results: &str,
) -> Result<Option<Person>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO people
            (id, password_hash_results)
        VALUES
            (?1, ?2)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut people_iter = match stmt.query_map((id, password_hash_results), get_person_from_row) {
        Ok(people) => people,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(person_maybe) = people_iter.next() {
        if let Ok(person) = person_maybe {
            return Ok(Some(person));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<Person>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            people
        WHERE
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut people_iter = match stmt.query_map([id], get_person_from_row) {
        Ok(people) => people,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(person_maybe) = people_iter.next() {
        if let Ok(person) = person_maybe {
            return Ok(Some(person));
        }
    }

    Ok(None)
}

pub fn read_by_kind(conn: &mut Connection, kind: u64) -> Result<Option<Person>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            people
        WHERE
            kind = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut people_iter = match stmt.query_map([kind], get_person_from_row) {
        Ok(people) => people,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(person_maybe) = people_iter.next() {
        if let Ok(person) = person_maybe {
            return Ok(Some(person));
        }
    }

    Ok(None)
}

// pub fn delete(
//     conn: &mut Connection,
//     contact_kind_id: u64,
//     deleted_at: u64,
// ) -> Result<Option<Person>, String> {
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

//     let mut contact_kind = match stmt.query_map((deleted_at, contact_kind_id), get_person_from_row)
//     {
//         Ok(contact_kind) => contact_kind,
//         Err(e) => return Err(e.to_string()),
//     };

//     if let Some(person_maybe) = contact_kind.next() {
//         if let Ok(invitation) = person_maybe {
//             return Ok(Some(invitation));
//         }
//     }

//     Ok(None)
// }

// pub fn dangerously_delete(
//     conn: &mut Connection,
//     contact_kind_id: u64,
// ) -> Result<Option<Person>, String> {
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

//     let mut contact_kind = match stmt.query_map([contact_kind_id], get_person_from_row) {
//         Ok(contact_kind) => contact_kind,
//         Err(e) => return Err(e.to_string()),
//     };

//     if let Some(person_maybe) = contact_kind.next() {
//         if let Ok(invitation) = person_maybe {
//             return Ok(Some(invitation));
//         }
//     }

//     Ok(None)
// }
