// setup tables

// setup fallback users & roles

// remove outdated

// remove deleted at
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

    let first_arg = match args.get(0) {
        Some(arg) => arg,
        _ => return Err("no arguments provided".to_string()),
    };
    println!("{}", first_arg);

    // if arg create_tables ./config
    Ok(())
}

fn create_tables(sqlite_path: &PathBuf) -> Result<(), String> {
    // create connection
    let mut conn = match Connection::open(sqlite_path) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    if let Err(e) = contact_kinds::create_table(&mut conn) {
        return Err(e);
    }

    if let Err(e) = signups::create_table(&mut conn) {
        return Err(e);
    }

    if let Err(e) = contacts::create_table(&mut conn) {
        return Err(e);
    }

    if let Err(e) = people::create_table(&mut conn) {
        return Err(e);
    }

    if let Err(e) = sessions::create_table(&mut conn) {
        return Err(e);
    }

    if let Err(e) = public_sessions::create_table(&mut conn) {
        return Err(e);
    }

    // then pass connection
    Ok(())
}
