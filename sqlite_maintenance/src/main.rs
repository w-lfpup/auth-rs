// takes args input like

// weblog_sqlite -- setup
// weblog_sqlite -- clean
// weblog_sqlite -- article

use std::env;
use std::path::PathBuf;

use config::{Config, FallbackUser};
use rusqlite::Result;
use sqlite_interface::{
    create_snowprints, emails, hash_password, people, roles, roles_to_people, sessions,
};

const roles: [&str; 3] = ["administrator", "roles", "editor"];

// Creating tables and creating fall back users and roles
// is the same action.
//

#[tokio::main]
async fn main() -> Result<(), String> {
    // get args 0
    let action = match env::args().nth(1) {
        Some(actn) => actn,
        _ => return Err("no action found at arg[1]".to_string()),
    };

    let config_path_buf = match env::args().nth(2) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[2] config path not included.".to_string()),
    };
    println!("{:?}", config_path_buf);

    let config = match Config::from_filepath(&config_path_buf).await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let results = match action.as_str() {
        "setup_dbs" => setup_dbs(&config).await,
        _ => return Err("no action function matched arg[0]".to_string()),
    };

    if let Err(e) = results {
        println!("{:?}", e);
    }

    Ok(())
}

// sqlite_db_auth pathbuf
// origin_time
// email
// password

async fn setup_dbs(config: &Config) -> Result<(), String> {
    // create tables
    if let Err(e) = emails::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = people::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = roles::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = roles_to_people::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = sessions::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    // create fallback person (ME!)
    let mut snowprints = match create_snowprints(config.origin_time, None) {
        Ok(sp) => sp,
        Err(e) => return Err(e),
    };

    let fallback_path_buf = match env::args().nth(3) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[3] fallback path not included.".to_string()),
    };

    let fallback_account = match FallbackUser::from_filepath(&fallback_path_buf).await {
        Ok(fu) => fu,
        Err(e) => return Err(e),
    };

    let people_id = match snowprints.compose() {
        Ok(id) => id,
        _ => return Err("couldn't create people_id for fallback user.".to_string()),
    };

    let email_id = match snowprints.compose() {
        Ok(id) => id,
        _ => return Err("couldn't create email_id for fallback user.".to_string()),
    };

    // email
    if let Err(e) = emails::create(
        &config.sqlite_db_auth,
        email_id,
        people_id,
        &fallback_account.email,
    ) {
        return Err("couldn't create email entry for fallback user.".to_string());
    };

    let password_hash_params = match hash_password(&fallback_account.password) {
        Ok(id) => id,
        _ => return Err("couldn't hash password for fallback user.".to_string()),
    };

    if let Err(e) = people::create(&config.sqlite_db_auth, people_id, &password_hash_params) {
        return Err("couldn't create people entry for fallback user.".to_string());
    }

    // create roles

    for role in roles {
        println!("creating role: {}", role);

        let role_id = match snowprints.compose() {
            Ok(role_id) => role_id,
            _ => {
                println!("failed to create role_id");
                continue;
            }
        };

        if let Err(e) = roles::create(&config.sqlite_db_auth, role_id, role) {
            return Err("couldn't create role entry.".to_string());
        };

        let roles_to_people_id = match snowprints.compose() {
            Ok(roles_to_people_id) => roles_to_people_id,
            _ => continue,
        };

        if let Err(e) = roles_to_people::create(
            &config.sqlite_db_auth,
            roles_to_people_id,
            role_id,
            people_id,
        ) {
            return Err("couldn't create role_to_people entry.".to_string());
        };
    }

    Ok(())
}

async fn clean_up_dbs(config: &Config) -> Result<(), String> {
    println!("clean_up_dbs()");


    Ok(())
}
