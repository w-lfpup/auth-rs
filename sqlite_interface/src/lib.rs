pub mod emails;
pub mod people;
pub mod roles;
pub mod roles_to_people;
pub mod sessions;
pub mod session_rate_limits;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use snowprints::{Settings as SnowprintSettings, Snowprint};
use std::path::PathBuf;
use std::time::Duration;
use std::time::UNIX_EPOCH;

// An intentionally limited, structured, and journey driven API

pub struct AuthDb {
    db_path: PathBuf,
    snowprints: Snowprint,
}

impl AuthDb {
    pub fn from(db_path: &PathBuf, origin_time_ms: u64) -> Result<AuthDb, String> {
        let snowprints = match create_snowprints(origin_time_ms, None) {
            Ok(sp) => sp,
            Err(e) => return Err("failed to create snowprints".to_string()),
        };

        Ok(AuthDb {
            db_path: db_path.clone(),
            snowprints: snowprints,
        })
    }

    // session_exists()

    // create_guest_session()
    //   -> rate limit ip to guest session
    //   -> if okay return guest session

    // rate_limit_session() / session exists
    //   -> yes no

    // create_person_session_by_email()
    //   -> rate limit person session create because password are expensive
    //   ->

    // uwer has roles
}

pub struct AuthMaintenanceDb  {}

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
