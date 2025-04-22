// use rusqlite::{Connection, Result};
// use std::path::PathBuf;

// pub struct Role {
//     id: u64,
//     kind: String,
//     deleted_at: u64,
// }

// pub fn create_table(path: &PathBuf) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (roles)".to_string()),
//     };

//     let results = conn.execute(
//         "CREATE TABLE IF NOT EXISTS roles (
//             id INTEGER PRIMARY KEY,
//             kind TEXT NOT NULL UNIQUE,
//             deleted_at INTEGER
//         )",
//         (),
//     );

//     if let Err(e) = results {
//         return Err("roles_to_roles: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// pub fn create(path: &PathBuf, role_id: u64, kind: &str) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (roles)".to_string()),
//     };

//     let results = conn.execute(
//         "INSERT OR IGNORE INTO roles
//             (id, kind)
//         VALUES
//             (?1, ?2)",
//         (role_id, kind),
//     );

//     if let Err(e) = results {
//         return Err("roles: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// pub fn read(path: &PathBuf, role_id: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (roles)".to_string()),
//     };

//     let results = conn.execute(
//         "SELECT roles
//         WHERE id = ?1",
//         [role_id],
//     );

//     // iterate through roles

//     if let Err(e) = results {
//         return Err("read roles: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// pub fn read_id_by_kind(path: &PathBuf, kind: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (roles)".to_string()),
//     };

//     let results = conn.execute(
//         "SELECT roles
//         WHERE kind = ?1",
//         [kind],
//     );

//     // iterate through roles

//     if let Err(e) = results {
//         return Err("read roles by kind: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// pub fn delete(path: &PathBuf, role_id: u64, timestamp_ms: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (roles)".to_string()),
//     };

//     let results = conn.execute(
//         "UPDATE roles
//         SET deleted_at = ?1
//         WHERE id = ?2",
//         (timestamp_ms, role_id),
//     );

//     if let Err(e) = results {
//         return Err("delete roles: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// pub fn dangerously_delete(path: &PathBuf, role_id: u64, timestamp_ms: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (roles)".to_string()),
//     };

//     let results = conn.execute(
//         "DELETE roles
//         WHERE id = ?1",
//         [role_id],
//     );

//     if let Err(e) = results {
//         return Err("dangerously delete roles: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }
