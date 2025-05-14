// EMAIL INVITATIONS

use rand::Rng;
use rusqlite::{Connection, Result};
use snowprints::{decompose, Settings as SnowprintSettings, Snowprints};
use std::path::PathBuf;

const INVITATION_LENGTH_MS: usize = 2629800000;

pub struct Invitation {
    id: u64,
    session: u64,
    session_length_ms: usize,
    contact_type: u16,
    contact_data: String,
    completed_at: u64,
    deleted_at: u64,
}

// let snowprints = match create_snowprints(origin_time_ms, None) {
//     Ok(sp) => sp,
//     Err(e) => return Err("failed to create snowprints".to_string()),
// };

// id INTEGER PRIMARY KEY UNIQUE,
// contact_type INTEGER NOT NULL,
// contact_data TEXT KEY UNIQUE NOT NULL,
// session INTEGER NOT NULL,
// session_length_ms INTEGER NOT NULL,
// completed_at INTEGER,
// deleted_at INTEGER

pub struct InvitationsCrud {
    snowprints: Snowprints,
}

impl InvitationsCrud {
    fn new(&self, snowprint_settings: SnowprintSettings) -> Result<InvitationsCrud, String> {
        let snowprints = match Snowprints::new(snowprint_settings) {
            Ok(sp) => sp,
            Err(e) => return Err("failed to create snowprints".to_string()),
        };

        Ok(InvitationsCrud {
            snowprints: snowprints,
        })
    }

    fn create(&mut self, contact_type: u16, contact_data: String) -> Result<String, String> {
        let snowprint = match self.snowprints.compose() {
            Ok(sp) => sp,
            Err(e) => return Err("snowprint error has no default formatter".to_string()),
        };

        let (timestamp_ms, _, _) = decompose(snowprint);

        // get snowprint
        // get random 128
        // convert to hex
        // return as string
        Ok("".to_string())
    }
}

pub fn create_table(path: &PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(cn) => cn,
        Err(e) => return Err("falled to connect to sqlite db (session)".to_string()),
    };

    let results = conn.execute(
        "CREATE TABLE IF NOT EXISTS invitations (
            id INTEGER PRIMARY KEY UNIQUE,
            contact_type INTEGER NOT NULL,
            contact_data TEXT KEY UNIQUE NOT NULL,
            session INTEGER NOT NULL,
            session_length_ms INTEGER NOT NULL,
            completed_at INTEGER,
            deleted_at INTEGER
        )",
        (),
    );

    if let Err(e) = results {
        return Err("invitations table: \n".to_string() + &e.to_string());
    }

    Ok(())
}

// pub fn create(
//     &mut self,
//     path: &PathBuf,
//     contact_type: u16,
//     contact_content: &str,
// ) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (invitations table)".to_string()),
//     };

//     // create session id
//     let session_id = match self.snowprints.compose();

//     let mut rng = rand::thread_rng();
//     let session: u64 = rng.gen();

//     let results = conn.execute(
//         "INSERT INTO invitations
//             (id, contact_type, contact_data, session, session_length_ms)
//         VALUES
//             (?1, ?2, ?3, ?4, ?5)",
//         (session_id, contact_type, contact_data, session, self.session_length_ms),
//     );

//     if let Err(e) = results {
//         return Err("create invitations: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// pub fn read(path: &PathBuf, session_id: u64) -> Result<Option<()>, String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (invitations table)".to_string()),
//     };

//     let results = conn.execute(
//         "SELECT invitations
//         WHERE id = ?1",
//         [session_id],
//     );

//     // iterate return

//     if let Err(e) = results {
//         return Err("read invitations: \n".to_string() + &e.to_string());
//     }

//     Ok(None)
// }

// pub fn read_by_contact_content(path: &PathBuf, contact_info: &str) -> Result<Option<()>, String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (invitations table)".to_string()),
//     };

//     let results = conn.execute(
//         "SELECT invitations
//         WHERE id = ?1",
//         [session_id],
//     );

//     // iterate return

//     if let Err(e) = results {
//         return Err("read invitations: \n".to_string() + &e.to_string());
//     }

//     Ok(None)
// }

// pub fn delete(path: &PathBuf, session_id: u64, timestamp_ms: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (invitations table)".to_string()),
//     };

//     let results = conn.execute(
//         "UPDATE invitations
//         SET deleted_at = ?1
//         WHERE id = ?2",
//         (timestamp_ms, session_id),
//     );

//     if let Err(e) = results {
//         return Err("delete invitations: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }

// Invitations Maintenance

// pub fn dangerously_delete(path: &PathBuf, people_id: u64, timestamp_ms: u64) -> Result<(), String> {
//     let conn = match Connection::open(path) {
//         Ok(cn) => cn,
//         Err(e) => return Err("falled to connect to sqlite db (invitations table)".to_string()),
//     };

//     let results = conn.execute(
//         "DELETE invitations
//         WHERE id = ?1",
//         [people_id],
//     );

//     if let Err(e) = results {
//         return Err("dangerously delete invitations: \n".to_string() + &e.to_string());
//     }

//     Ok(())
// }
