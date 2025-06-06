use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::sessions::Session;

fn get_session_from_row(row: &Row) -> Result<Session, RusqliteError> {
    Ok(Session {
        id: row.get(0)?,
        people_id: row.get(1)?,
        deleted_at: row.get(2)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            people_id INTEGER,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("sessions table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    people_id: Option<u64>,
) -> Result<Option<Session>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO sessions
            (id, people_id)
        VALUES
            (?1, ?2)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut sessions_iter = match stmt.query_map((id, people_id), get_session_from_row) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(session_maybe) = sessions_iter.next() {
        if let Ok(session) = session_maybe {
            return Ok(Some(session));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, session_id: u64) -> Result<Option<Session>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            sessions
        WHERE
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut sessions_iter = match stmt.query_map([session_id], get_session_from_row) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(session_maybe) = sessions_iter.next() {
        if let Ok(session) = session_maybe {
            return Ok(Some(session));
        }
    }

    Ok(None)
}

pub fn read_all_by_people_id(
    conn: &mut Connection,
    people_id: u64,
    offset: usize,
    limit: usize,
) -> Result<Vec<Session>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            sessions
        WHERE
            deleted_at IS NULL
            AND
            people_id = ?1
        ORDER BY
            id DESC
        LIMIT
            ?2,?3
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let session_iter = match stmt.query_map((people_id, offset, limit), get_session_from_row) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    let mut sessions: Vec<Session> = Vec::new();
    for session_maybe in session_iter {
        if let Ok(session) = session_maybe {
            sessions.push(session);
        }
    }

    Ok(sessions)
}
