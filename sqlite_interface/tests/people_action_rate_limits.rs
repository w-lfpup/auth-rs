use rusqlite::{Connection, Result};
use sqlite_interface::people_action_rate_limits;
use type_flyweight::people_action_rate_limits::PeopleActionRateLimit;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = people_action_rate_limits::create_table(&mut conn) {
        assert!(false, "failed to create public_session table");
    }

    let incorrect_incorrect_people_action_rate_limit = PeopleActionRateLimit {
        people_id: 32,
        kind_id: 3,
        window_count: 42,
        prev_window_count: 51,
        updated_at: 27,
        deleted_at: None,
    };

    // create
    let people_action_rate_limit =
        match people_action_rate_limits::rate_limit_people_action(&mut conn, 64, 1, 20, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(None != people_action_rate_limit);

    // second rate limit
    let second_people_action_rate_limit =
        match people_action_rate_limits::rate_limit_people_action(&mut conn, 64, 1, 20, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(people_action_rate_limit != second_people_action_rate_limit);
    assert!(
        Some(incorrect_incorrect_people_action_rate_limit.clone())
            != second_people_action_rate_limit
    );

    Ok(())
}
