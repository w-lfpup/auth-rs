use std::env;
use std::path::PathBuf;

use rusqlite::{Connection, Error as RusqliteError, Result, Row};
use sqlite_interface::{contact_kinds, contacts, people, public_sessions, sessions, signups};

fn main() -> Result<(), String> {
    let cwd = match env::current_dir() {
        Ok(directory_path) => directory_path,
        _ => return Err("current directory does not exist!".to_string()),
    };
    println!("{:?}", cwd);

    // get args
    let args: Vec<String> = env::args().collect();
    let action = match args.get(1) {
        Some(arg) => arg,
        _ => return Err("no arguments provided".to_string()),
    };
    println!("{}", action);

    let sqlite_path = match args.get(3) {
        Some(arg) => PathBuf::from(cwd).join(arg),
        _ => return Err("no arguments provided".to_string()),
    };
    let mut conn = match Connection::open(sqlite_path) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    println!("{}", action);

    // if arg create_tables ./config
    Ok(())
}

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
