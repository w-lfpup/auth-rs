use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::sessions::PublicSession;

fn get_public_session_from_row(row: &Row) -> Result<PublicSession, RusqliteError> {
    Ok(PublicSession {
        id: row.get(0)?,
        people_id: row.get(1)?,
        token: row.get(2)?,
        session_id: row.get(3)?,
        window_count: row.get(4)?,
        prev_window_count: row.get(5)?,
        updated_at: row.get(6)?,
        deleted_at: row.get(7)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS public_sessions (
            id INTEGER PRIMARY KEY,
            people_id INTEGER NOT NULL,
            token INTEGER NOT NULL,
            session_id INTEGER NOT NULL,
            window_count INTEGER NOT NULL,
            prev_window_count INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
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
    token: u64,
    session_id: u64,
) -> Result<Option<PublicSession>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO public_sessions
            (id, people_id, token, session_id, window_count, prev_window_count, updated_at)
        VALUES
            (?1, ?2, ?3, ?4, 1, 0, 0)
        RETURNING
            *
    ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut sessions_iter = match stmt.query_map(
        (id, people_id, token, session_id),
        get_public_session_from_row,
    ) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(session_maybe) = sessions_iter.next() {
        if let Ok(public_session) = session_maybe {
            return Ok(Some(public_session));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<PublicSession>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            public_sessions
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut sessions_iter = match stmt.query_map([id], get_public_session_from_row) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(session_maybe) = sessions_iter.next() {
        if let Ok(public_session) = session_maybe {
            return Ok(Some(public_session));
        }
    }

    Ok(None)
}

pub fn read_all_by_session_id(
    conn: &mut Connection,
    session_id: u64,
    offset: usize,
    limit: usize,
) -> Result<Vec<PublicSession>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            public_sessions
        WHERE
            deleted_at IS NULL
            AND
            session_id = ?1
        ORDER BY
            id DESC
        LIMIT
            ?2,?3
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let public_sessions_iter =
        match stmt.query_map((session_id, offset, limit), get_public_session_from_row) {
            Ok(sessions) => sessions,
            Err(e) => return Err(e.to_string()),
        };

    let mut sessions: Vec<PublicSession> = Vec::new();
    for session_maybe in public_sessions_iter {
        if let Ok(public_session) = session_maybe {
            sessions.push(public_session);
        }
    }

    Ok(sessions)
}

pub fn read_all_by_people_id(
    conn: &mut Connection,
    people_id: Option<u64>,
    offset: usize,
    limit: usize,
) -> Result<Vec<PublicSession>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            public_sessions
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

    let public_sessions_iter =
        match stmt.query_map((people_id, offset, limit), get_public_session_from_row) {
            Ok(sessions) => sessions,
            Err(e) => return Err(e.to_string()),
        };

    let mut sessions: Vec<PublicSession> = Vec::new();
    for session_maybe in public_sessions_iter {
        if let Ok(public_session) = session_maybe {
            sessions.push(public_session);
        }
    }

    Ok(sessions)
}

pub fn rate_limit_session(
    conn: &mut Connection,
    id: u64,
    current_timestamp: u64,
    window_max_count: u64,
    window_length_ms: u64,
) -> Result<Option<PublicSession>, String> {
    // update or ignore

    // provide id, window limit, window length, and current_timestamp
    let mut stmt = match conn.prepare(
        "
        UPDATE OR IGNORE
            public_sessions
        SET
            prev_window_count =
                CASE
                    WHEN ?1 > (?2 - updated_at) THEN prev_window_count
                    ELSE MIN(window_count, ?3)
                END,
            window_count =
                CASE
                    WHEN ?1 > (?2 - updated_at) THEN (window_count + 1)
                    ELSE 1
                END,
            updated_at = ?2
        WHERE
            deleted_at IS NULL
            AND
            id = ?4
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut sessions_iter = match stmt.query_map(
        (window_length_ms, current_timestamp, window_max_count, id),
        get_public_session_from_row,
    ) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(session_maybe) = sessions_iter.next() {
        if let Ok(public_session) = session_maybe {
            return Ok(Some(public_session));
        }
    }

    Ok(None)
}
