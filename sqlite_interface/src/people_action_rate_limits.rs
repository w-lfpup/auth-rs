use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::people_action_rate_limits::PeopleActionRateLimit;

// need to have a qualifier
// rate limit stuff

fn get_people_action_rate_limit_from_row(
    row: &Row,
) -> Result<PeopleActionRateLimit, RusqliteError> {
    Ok(PeopleActionRateLimit {
        people_id: row.get(0)?,
        kind_id: row.get(1)?,
        window_count: row.get(2)?,
        prev_window_count: row.get(3)?,
        updated_at: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS people_action_rate_limits (
            people_id INTEGER NOT NULL,
            kind_id INTEGER NOT NULL,
            window_count INTEGER NOT NULL,
            prev_window_count INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            deleted_at INTEGER,
            PRIMARY KEY (people_id, kind_id)
        )",
        (),
    );

    if let Err(e) = results {
        return Err("sessions table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn rate_limit_people_action(
    conn: &mut Connection,
    people_id: u64,
    kind_id: u64,
    current_timestamp: u64,
    window_length_ms: u64,
) -> Result<Option<PeopleActionRateLimit>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO people_action_rate_limits
            (people_id, kind_id, window_count, prev_window_count, updated_at)
        VALUES
            (?1, ?2, 1, 0, 0)
        ON CONFLICT (people_id, kind_id) DO UPDATE
            SET
                prev_window_count =
                    CASE
                        WHEN ?3 > (?4 - updated_at) THEN prev_window_count
                        ELSE window_count
                    END,
                window_count =
                    CASE
                        WHEN ?3 > (?4 - updated_at) THEN (window_count + 1)
                        ELSE 1
                    END,
                updated_at = ?4
        RETURNING
            *
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut people_action_rate_limit_iter = match stmt.query_map(
        (people_id, kind_id, window_length_ms, current_timestamp),
        get_people_action_rate_limit_from_row,
    ) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(people_action_rate_limit_maybe) = people_action_rate_limit_iter.next() {
        if let Ok(people_action_rate_limit) = people_action_rate_limit_maybe {
            return Ok(Some(people_action_rate_limit));
        }
    }

    Ok(None)
}
