use std::env;
use std::path::PathBuf;

use rusqlite::{Connection, Result};
// use sqlite_interface::{contact_kinds, contacts, people, public_sessions, sessions, signups};

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
    let _conn = match Connection::open(sqlite_path) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    println!("{}", action);

    // if arg create_tables ./config
    Ok(())
}
