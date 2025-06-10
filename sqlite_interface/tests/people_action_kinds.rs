use rusqlite::{Connection, Result};
use sqlite_interface::people_action_kinds;
use type_flyweight::people_action_rate_limits::PeopleActionKind;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = people_action_kinds::create_table(&mut conn) {
        assert!(false, "failed to create people_action_kinds table");
    }

    let incorrect_people_action_kind = PeopleActionKind {
        id: 0,
        kind: "foul and filth preach sublime".to_string(),
        deleted_at: None,
    };

    // create
    let people_action_kind = match people_action_kinds::create(&mut conn, 1, "create_account") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let people_action_kind_read_by_id = match people_action_kinds::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != people_action_kind);
    assert!(people_action_kind == people_action_kind_read_by_id);
    assert!(Some(incorrect_people_action_kind.clone()) != people_action_kind_read_by_id);

    // read by kind
    let people_action_kind_read_by_kind =
        match people_action_kinds::read_by_kind(&mut conn, "create_account") {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(None != people_action_kind);
    assert!(people_action_kind == people_action_kind_read_by_kind);
    assert!(Some(incorrect_people_action_kind.clone()) != people_action_kind_read_by_kind);

    Ok(())
}
