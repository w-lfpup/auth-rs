use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use snowprints::{Settings as SnowprintSettings, Snowprints};
use std::path::PathBuf;

mod connector;
mod invitations;

use connector::Connector;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

// An intentionally limited, structured, and journey driven API

// origin time duration
pub struct AuthDb {
    origin_time_ms: u64,
    connector: Connector,
}

impl AuthDb {
    pub fn from(db_path: &PathBuf, origin_time_ms: u64) -> Result<AuthDb, String> {
        // get duration
        let connector = match Connector::from(db_path, 8) {
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
    origin_time_ms: u64,
    volume_params: Option<(u64, u64)>,
) -> Result<Snowprints, String> {
    let (logical_volume_base, logical_volume_length) = match volume_params {
        Some(vp) => vp,
        _ => (0, 8192),
    };

    let snowprint_settings = SnowprintSettings {
        origin_time_ms: origin_time_ms,
        logical_volume_base: logical_volume_base,
        logical_volume_length: logical_volume_length,
    };

    match Snowprints::new(snowprint_settings) {
        Ok(sp) => Ok(sp),
        Err(_e) => return Err("failed to create snowprints".to_string()),
    }
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(ph) => Ok(ph.to_string()),
        Err(e) => return Err(e.to_string()),
    }
}

pub fn validate_password(password: &str, password_hash_params: &str) -> bool {
    let parsed_hash = match PasswordHash::new(&password_hash_params) {
        Ok(ph) => ph,
        _ => return false,
    };

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => true,
        _ => false,
    }
}

fn create_invitation_and_session_as_base64(invitation_id: u64, session: u64) -> String {
    let mut invitation: String = "".to_string();

    invitation.push_str(&URL_SAFE.encode(invitation_id.to_ne_bytes()));
    invitation.push(':');
    invitation.push_str(&URL_SAFE.encode(session.to_ne_bytes()));

    invitation
}

fn get_arry_u8(data_vec: Vec<u8>) -> Result<[u8; 8], String> {
    if 8 != data_vec.len() {
        return Err("required length not found".to_string());
    }

    let mut data: [u8; 8] = [0; 8];
    let mut index = 0;
    for pip in data {
        data[index] = pip;
        index += 1;
    }

    Ok(data)
}

fn get_invitation_and_session_from_base64(invitation_base64: &str) -> Result<(u64, u64), String> {
    let mut splitted = invitation_base64.split(":");

    let mut invitation_u64: Option<u64> = None;
    if let Some(invitation_base64) = splitted.next() {
        if let Ok(invitation_vec_bytes) = URL_SAFE.decode(invitation_base64.as_bytes()) {
            if let Ok(invitation_arr) = get_arry_u8(invitation_vec_bytes) {
                invitation_u64 = Some(u64::from_ne_bytes(invitation_arr));
            }
        }
    }

    let mut session_u64: Option<u64> = None;
    if let Some(session_base64) = splitted.next() {
        if let Ok(session_vec_bytes) = URL_SAFE.decode(session_base64.as_bytes()) {
            if let Ok(session_arr) = get_arry_u8(session_vec_bytes) {
                session_u64 = Some(u64::from_ne_bytes(session_arr));
            }
        };
    }

    if let (Some(invitation), Some(session)) = (invitation_u64, session_u64) {
        return Ok((invitation, session));
    }

    Err("didnt' make it!".to_string())
}
