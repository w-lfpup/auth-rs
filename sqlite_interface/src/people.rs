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
        _ => return Err("cound not create person".to_string()),
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
        _ => return Err("cound not read person".to_string()),
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
