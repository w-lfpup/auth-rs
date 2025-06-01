use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::signups::Signup;

fn get_signup_from_row(row: &Row) -> Result<Signup, RusqliteError> {
    Ok(Signup {
        id: row.get(0)?,
        session: row.get(1)?,
        session_length_ms: row.get(2)?,
        contact_kind_id: row.get(3)?,
        contact_content: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS signups (
            id INTEGER PRIMARY KEY,
            session INTEGER NOT NULL,
            session_length_ms INTEGER NOT NULL,
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
    session: u64,
    session_length_ms: u32,
    contact_kind_id: u64,
    contact_content: &str,
) -> Result<Option<Signup>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO signups
            (id, session, session_length_ms, contact_kind, contact_content)
        VALUES
            (?1, ?2, ?3, ?4, ?5)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut signups = match stmt.query_map(
        (
            id,
            session,
            session_length_ms,
            contact_kind_id,
            contact_content,
        ),
        get_signup_from_row,
    ) {
        Ok(signups) => signups,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(signup_maybe) = signups.next() {
        if let Ok(invitation) = signup_maybe {
            return Ok(Some(invitation));
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
        if let Ok(invitation) = signup_maybe {
            return Ok(Some(invitation));
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

// pub fn delete(
//     conn: &mut Connection,
//     signup_id: u64,
//     deleted_at: u64,
// ) -> Result<Option<Signup>, String> {
//     let mut stmt = match conn.prepare(
//         "
//         UPDATE
//             signups
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

//     let mut signups = match stmt.query_map((deleted_at, signup_id), get_signup_from_row)
//     {
//         Ok(signups) => signups,
//         Err(e) => return Err(e.to_string()),
//     };

//     if let Some(signup_maybe) = signups.next() {
//         if let Ok(invitation) = signup_maybe {
//             return Ok(Some(invitation));
//         }
//     }

//     Ok(None)
// }

// pub fn dangerously_delete(
//     conn: &mut Connection,
//     signup_id: u64,
// ) -> Result<Option<Signup>, String> {
//     let mut stmt = match conn.prepare(
//         "
//         DELETE
//             signups
//         WHERE
//             id = ?1
//         RETURNING
//             *
//     ",
//     ) {
//         Ok(stmt) => stmt,
//         _ => return Err("cound not prepare statement".to_string()),
//     };

//     let mut signups = match stmt.query_map([signup_id], get_signup_from_row) {
//         Ok(signups) => signups,
//         Err(e) => return Err(e.to_string()),
//     };

//     if let Some(signup_maybe) = signups.next() {
//         if let Ok(invitation) = signup_maybe {
//             return Ok(Some(invitation));
//         }
//     }

//     Ok(None)
// }

// dangerously_delete_all_stale
