use rusqlite::{Connection, Error as RusqliteError, Result, Row};

use type_flyweight::ip_address_rate_limits::IpAddressRateLimit;

fn get_ip_address_rate_limit_from_row(row: &Row) -> Result<IpAddressRateLimit, RusqliteError> {
    Ok(IpAddressRateLimit {
        ip_address: row.get(0)?,
        kind_id: row.get(1)?,
        window_count: row.get(2)?,
        prev_window_count: row.get(3)?,
        updated_at: row.get(4)?,
        deleted_at: row.get(5)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS ip_address_rate_limits (
            ip_address TEXT NOT NULL,
            kind_id INTEGER NOT NULL,
            window_count INTEGER NOT NULL,
            prev_window_count INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            deleted_at INTEGER,
            PRIMARY KEY (ip_address, kind_id)
        )",
        (),
    );

    if let Err(e) = results {
        return Err("sessions table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn rate_limit_ip_address(
    conn: &mut Connection,
    ip_address: &str,
    kind_id: u64,
    current_timestamp: u64,
    max_window_count: u64,
    window_length_ms: u64,
) -> Result<Option<IpAddressRateLimit>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO ip_address_rate_limits
            (ip_address, kind_id, window_count, prev_window_count, updated_at)
        VALUES
            (?1, ?2, 1, 0, 0)
        ON CONFLICT(ip_address, kind_id) DO UPDATE
            SET
                prev_window_count =
                    CASE
                        WHEN ?3 > (?4 - updated_at) THEN prev_window_count
                        ELSE MIN(window_count, ?5)
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

    let mut ip_address_rate_limit_iter = match stmt.query_map(
        (
            ip_address,
            kind_id,
            window_length_ms,
            current_timestamp,
            max_window_count,
        ),
        get_ip_address_rate_limit_from_row,
    ) {
        Ok(sessions) => sessions,
        Err(e) => return Err(e.to_string()),
    };

    if let Some(ip_address_rate_limit_maybe) = ip_address_rate_limit_iter.next() {
        if let Ok(ip_address_rate_limit) = ip_address_rate_limit_maybe {
            return Ok(Some(ip_address_rate_limit));
        }
    }

    Ok(None)
}
