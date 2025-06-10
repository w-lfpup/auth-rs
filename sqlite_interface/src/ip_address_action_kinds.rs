// This table doesn't really scale, very shallow
// Has a unique property so a general query should consider map->reduce form multiple servers

use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use type_flyweight::ip_address_rate_limits::IpAddressActionKind;

fn get_ip_address_action_kind_from_row(row: &Row) -> Result<IpAddressActionKind, RusqliteError> {
    Ok(IpAddressActionKind {
        id: row.get(0)?,
        kind: row.get(1)?,
        deleted_at: row.get(2)?,
    })
}

pub fn create_table(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS ip_address_action_kinds (
            id INTEGER PRIMARY KEY,
            kind TEXT NOT NULL UNIQUE,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("ip_address_action_kinds table error: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create(
    conn: &mut Connection,
    id: u64,
    kind: &str,
) -> Result<Option<IpAddressActionKind>, String> {
    let mut stmt = match conn.prepare(
        "
        INSERT INTO ip_address_action_kinds
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

    let mut ip_address_action_kind_iter =
        match stmt.query_map((id, kind), get_ip_address_action_kind_from_row) {
            Ok(kind_iter) => kind_iter,
            Err(e) => return Err(e.to_string()),
        };

    if let Some(ip_address_action_kind_maybe) = ip_address_action_kind_iter.next() {
        if let Ok(ip_address_action_kind) = ip_address_action_kind_maybe {
            return Ok(Some(ip_address_action_kind));
        }
    }

    Ok(None)
}

pub fn read(conn: &mut Connection, id: u64) -> Result<Option<IpAddressActionKind>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            ip_address_action_kinds
        WHERE
            deleted_at IS NULL
            AND
            id = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut ip_address_action_kind_iter =
        match stmt.query_map([id], get_ip_address_action_kind_from_row) {
            Ok(ip_address_action_kind) => ip_address_action_kind,
            Err(e) => return Err(e.to_string()),
        };

    let mut ip_address_action_kinds: Vec<IpAddressActionKind> = Vec::new();
    if let Some(ip_address_action_kind_maybe) = ip_address_action_kind_iter.next() {
        if let Ok(ip_address_action_kind) = ip_address_action_kind_maybe {
            return Ok(Some(ip_address_action_kind));
        }
    }

    Ok(None)
}

pub fn read_by_kind(
    conn: &mut Connection,
    kind: &str,
) -> Result<Option<IpAddressActionKind>, String> {
    let mut stmt = match conn.prepare(
        "
        SELECT
            *
        FROM
            ip_address_action_kinds
        WHERE
            deleted_at IS NULL
            AND
            kind = ?1
        ",
    ) {
        Ok(stmt) => stmt,
        _ => return Err("cound not prepare statement".to_string()),
    };

    let mut ip_address_action_kind_iter =
        match stmt.query_map([kind], get_ip_address_action_kind_from_row) {
            Ok(kind_iter) => kind_iter,
            Err(e) => return Err(e.to_string()),
        };

    if let Some(ip_address_action_kind_maybe) = ip_address_action_kind_iter.next() {
        if let Ok(ip_address_action_kind) = ip_address_action_kind_maybe {
            return Ok(Some(ip_address_action_kind));
        }
    }

    Ok(None)
}
