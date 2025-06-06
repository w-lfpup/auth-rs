use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::signups::Signup;

fn get_signup_from_row(row: &Row) -> Result<Signup, RusqliteError> {
    Ok(Signup {
        id: row.get(0)?,
        token: row.get(1)?,
        contact_kind_id: row.get(2)?,
        contact_content: row.get(3)?,
        deleted_at: row.get(4)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS signups (
            id INTEGER PRIMARY KEY,
            token INTEGER NOT NULL,
            contact_kind_id INTEGER NOT NULL,
            contact_content TEXT KEY NOT NULL,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("signups table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    token: u64,
    contact_kind_id: u64,
    contact_content: &str,
) -> Result<Option<Signup>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO signups
            (id, token, contact_kind, contact_content)
        VALUES
            (?1, ?2, ?3, ?4)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut signups = match stmt.query_map(
        (id, token, contact_kind_id, contact_content),
        get_signup_from_row,
    ) {
        Ok(signups) => signups,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(signup_maybe) = signups.next() {
        if let Ok(singup) = signup_maybe {
            return Ok(Some(singup));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, signup_id: u64) -> Result<Option<Signup>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            signups
        WHERE
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut signups = match stmt.query_map([signup_id], get_signup_from_row) {
        Ok(signups) => signups,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(signup_maybe) = signups.next() {
        if let Ok(singup) = signup_maybe {
            return Ok(Some(singup));
        }
    }

    Ok(None)
}

pub fn read_by_contact(
    conn: &mut Connection,
    contact_kind_id: u64,
    contact_content: &str,
) -> Result<Vec<Signup>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            signups
        WHERE
            contact_kind_id = ?1
            AND contact_content = ?2
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let signup_iter = match stmt.query_map((contact_kind_id, contact_content), get_signup_from_row)
    {
        Ok(signups) => signups,
        Err(e) => return Err(e.to_string()),
    };

    let mut signups: Vec<Signup> = Vec::new();
    for signup_maybe in signup_iter {
        if let Ok(signup) = signup_maybe {
            signups.push(signup);
        }
    }

    Ok(signups)
}
