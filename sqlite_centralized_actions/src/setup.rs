
pub fn set_journal_mode_to_WAL2(conn: &mut Connection) -> Result<(), String> {
    let results = conn.execute("PRAGMA journal_mode = wal2", ());

    if let Err(e) = results {
        return Err("error setting journal mode to WAL2: \n".to_string() + &e.to_string());
    }

    Ok(())
}

pub fn create_tables(conn: &mut Connection) -> Result<(), String> {
    // create connection

    if let Err(e) = contact_kinds::create_table(conn) {
        return Err(e);
    }

    if let Err(e) = signups::create_table(conn) {
        return Err(e);
    }

    if let Err(e) = contacts::create_table(conn) {
        return Err(e);
    }

    if let Err(e) = people::create_table(conn) {
        return Err(e);
    }

    if let Err(e) = sessions::create_table(conn) {
        return Err(e);
    }

    if let Err(e) = public_sessions::create_table(conn) {
        return Err(e);
    }

    // then pass connection
    Ok(())
}
