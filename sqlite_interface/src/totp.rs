use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::totp::Totp;

// This table doesn't really scale, very shallow

fn get_totp_from_row(row: &Row) -> Result<Totp, RusqliteError> {
    Ok(Totp {
        id: row.get(0)?,
        people_id: row.get(1)?,
        secret_key: row.get(2)?,
        algorithm: row.get(3)?,
        period: row.get(4)?,
        digits: row.get(5)?,
        deleted_at: row.get(6)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS totp (
            id INTEGER PRIMARY KEY,
            people_id INTEGER NOT NULL,
            secret_key TEXT NOT NULL,
            alrgorithm INTEGER,
            period INTEGER,
            digits INTEGER,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("totp table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    people_id: u64,
    secret_key: &str,
) -> Result<Option<Totp>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO totp
            (id, people_id, secret_key)
        VALUES
            (?1, ?2, ?3)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not create totp".to_string()),
    };

    let mut totp_iter = match stmt.query_map((id, people_id, secret_key), get_totp_from_row) {
        Ok(totp) => totp,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(totp_maybe) = totp_iter.next() {
        if let Ok(totp) = totp_maybe {
            return Ok(Some(totp));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<Totp>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            totp
        WHERE
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not read totp".to_string()),
    };

    let mut totp_iter = match stmt.query_map([id], get_totp_from_row) {
        Ok(totp) => totp,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(totp_maybe) = totp_iter.next() {
        if let Ok(totp) = totp_maybe {
            return Ok(Some(totp));
        }
    }

    Ok(None)
}
