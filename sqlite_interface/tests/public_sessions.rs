use rusqlite::{Connection, Result};
use sqlite_interface::public_sessions;
use type_flyweight::sessions::PublicSession;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = public_sessions::create_table(&mut conn) {
        assert!(false, "failed to create public_session table");
    }

    let incorrect_public_session = PublicSession {
        id: 7,
        people_id: Some(47),
        token: 1234,
        session_id: 4321,
        window_count: 42,
        prev_window_count: 51,
        updated_at: 27,
        deleted_at: None,
    };

    // create
    let mut public_session = match public_sessions::create(&mut conn, 16, Some(64), 7654, 19) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let mut public_session_read_by_id = match public_sessions::read(&mut conn, 16) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != public_session);
    assert!(public_session == public_session_read_by_id);
    assert!(Some(incorrect_public_session.clone()) != public_session_read_by_id);

    let public_session_read_all_by_session_id =
        match public_sessions::read_all_by_session_id(&mut conn, 19, 0, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };
    // read by kind and content
    let public_session_read_all_by_people_id =
        match public_sessions::read_all_by_people_id(&mut conn, Some(64), 0, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(None != public_session);
    assert!(public_session_read_all_by_people_id == public_session_read_all_by_session_id);
    assert!(Vec::<PublicSession>::new() != public_session_read_all_by_people_id);
    assert!(Vec::<PublicSession>::new() != public_session_read_all_by_session_id);

    // rate_limit_session

    let mut public_session_rate_limit =
        match public_sessions::rate_limit_session(&mut conn, 16, 20, 10, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };
    assert!(None != public_session_rate_limit);
    println!("{:?}", public_session_rate_limit);

    Ok(())
}
