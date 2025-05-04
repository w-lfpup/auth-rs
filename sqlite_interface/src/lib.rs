use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use snowprints::{Settings as SnowprintSettings, Snowprint};
use std::path::PathBuf;
use std::time::Duration;
use std::time::UNIX_EPOCH;
use rusqlite::{Connection, Result};

mod connector;
mod invitations;

use connector::Connector;

// An intentionally limited, structured, and journey driven API

// origin time duration
pub struct AuthDb {
    origin_time_ms: u64,
    connector: Connector,
}

impl AuthDb {
    pub fn from(db_path: &PathBuf, origin_time_ms: u64) -> Result<AuthDb, String> {
        // get duration
        let connector = match Connector::from(db_path, 12, 12) {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string()),
        };
        
        Ok(AuthDb {
            origin_time_ms: origin_time_ms.clone(),
            connector: connector,
        })
    }
}

// UTILITY functions

pub fn create_snowprints(
    origin_system_time: u64,
    volume_params: Option<(u64, u64)>,
) -> Result<Snowprint, String> {
    let origin_time_duration = Duration::from_millis(origin_system_time);

    let (logical_volume_base, logical_volume_length) = match volume_params {
        Some(vp) => vp,
        _ => (0, 8192),
    };

    let snowprint_settings = SnowprintSettings {
        origin_system_time: UNIX_EPOCH + origin_time_duration,
        logical_volume_base: logical_volume_base,
        logical_volume_length: logical_volume_length,
    };

    match Snowprint::new(snowprint_settings) {
        Ok(sp) => Ok(sp),
        Err(e) => return Err("failed to create snowprints".to_string()),
    }
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(ph) => Ok(ph.to_string()),
        Err(e) => return Err("person, create error:\n".to_string() + &e.to_string()),
    }
}

pub fn validate_password(password: &str, password_hash_params: &str) -> bool {
    let parsed_hash = match PasswordHash::new(&password_hash_params) {
        Ok(ph) => ph,
        Err(e) => return false,
    };

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => true,
        _ => false,
    }
}

// AUTH JOURNEYS

// (context incoming request)
// no session?
//   create_guest_session(ip)
//      -> ratelimit ip
//      -> return guest session (base64 string)
//

// (context incoming request)
// ratelimit_session
//   -> ratelimit session
//   -> if no session or session is invalid return error
//   -> return Result<Option<u64>, String>
//        -> If none, valid guest session
//   ->

// now can ratelimit sessions from spamming resources

// ZONE OF GUEST ACCESS
//   can see login pages
//   can see public resources
//

// BUT I WANT TO LOGIN
// (context i want to use blog building tools in my blog)
// (context incoming request)
// create_people_session_by_email(session, email, password)
//   -> guest session exists?
//   -> return (session string, people_id u64)

// (context incoming request and needs to validate reading from protected resource)
//   session_has_role(session bas64 string, role)
//   -> session exists?
//   -> ratelimit session
//   -> return people_id Option<u64>
//


// async fn setup_dbs(config: &Config) -> Result<(), String> {
//     // create tables
//     if let Err(e) = emails::create_table(&config.sqlite_db_auth) {
//         return Err(e.to_string());
//     };

//     if let Err(e) = people::create_table(&config.sqlite_db_auth) {
//         return Err(e.to_string());
//     };

//     if let Err(e) = roles::create_table(&config.sqlite_db_auth) {
//         return Err(e.to_string());
//     };

//     if let Err(e) = roles_to_people::create_table(&config.sqlite_db_auth) {
//         return Err(e.to_string());
//     };

//     if let Err(e) = sessions::create_table(&config.sqlite_db_auth) {
//         return Err(e.to_string());
//     };

//     // create fallback person (ME!)
//     let mut snowprints = match create_snowprints(config.origin_time, None) {
//         Ok(sp) => sp,
//         Err(e) => return Err(e),
//     };

//     let fallback_path_buf = match env::args().nth(3) {
//         Some(fbjs) => PathBuf::from(fbjs),
//         _ => return Err("arg[3] fallback path not included.".to_string()),
//     };

//     let fallback_account = match FallbackUser::from_filepath(&fallback_path_buf).await {
//         Ok(fu) => fu,
//         Err(e) => return Err(e),
//     };

//     let people_id = match snowprints.compose() {
//         Ok(id) => id,
//         _ => return Err("couldn't create people_id for fallback user.".to_string()),
//     };

//     let email_id = match snowprints.compose() {
//         Ok(id) => id,
//         _ => return Err("couldn't create email_id for fallback user.".to_string()),
//     };

//     // email
//     if let Err(e) = emails::create(
//         &config.sqlite_db_auth,
//         email_id,
//         people_id,
//         &fallback_account.email,
//     ) {
//         return Err("couldn't create email entry for fallback user.".to_string());
//     };

//     let password_hash_params = match hash_password(&fallback_account.password) {
//         Ok(id) => id,
//         _ => return Err("couldn't hash password for fallback user.".to_string()),
//     };

//     if let Err(e) = people::create(&config.sqlite_db_auth, people_id, &password_hash_params) {
//         return Err("couldn't create people entry for fallback user.".to_string());
//     }

//     // create roles

//     for role in roles {
//         println!("creating role: {}", role);

//         let role_id = match snowprints.compose() {
//             Ok(role_id) => role_id,
//             _ => {
//                 println!("failed to create role_id");
//                 continue;
//             }
//         };

//         if let Err(e) = roles::create(&config.sqlite_db_auth, role_id, role) {
//             return Err("couldn't create role entry.".to_string());
//         };

//         let roles_to_people_id = match snowprints.compose() {
//             Ok(roles_to_people_id) => roles_to_people_id,
//             _ => continue,
//         };

//         if let Err(e) = roles_to_people::create(
//             &config.sqlite_db_auth,
//             roles_to_people_id,
//             role_id,
//             people_id,
//         ) {
//             return Err("couldn't create role_to_people entry.".to_string());
//         };
//     }

//     Ok(())
// }

// async fn clean_up_dbs(config: &Config) -> Result<(), String> {
//     println!("clean_up_dbs()");


//     Ok(())
// }
