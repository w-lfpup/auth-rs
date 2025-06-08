use rusqlite::{Connection, Result};
use sqlite_interface::sessions;
use type_flyweight::sessions::Session;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = sessions::create_table(&mut conn) {
        assert!(false, "failed to create sessions table");
    }

    let incorrect_session = Session {
        id: 0,
        people_id: Some(47),
        deleted_at: None,
    };

    // create
    let session = match sessions::create(&mut conn, 16, Some(42)) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let session_read_by_id = match sessions::read(&mut conn, 16) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != session);
    assert!(session == session_read_by_id);
    assert!(Some(incorrect_session.clone()) != session_read_by_id);

    // read by kind and content
    let session_read_all_by_people_id =
        match sessions::read_all_by_people_id(&mut conn, 42, 0, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    // THIS FEELS IFFY BUT IT SHOULD JUST FAIL?
    let session_confirmed = match session {
        Some(su) => su,
        _ => incorrect_session.clone(),
    };

    assert!(Vec::from([session_confirmed]) == session_read_all_by_people_id);
    assert!(Vec::<Session>::new() != session_read_all_by_people_id);

    Ok(())
}
