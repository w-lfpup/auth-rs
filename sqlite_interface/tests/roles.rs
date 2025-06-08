use rusqlite::{Connection, Result};
use sqlite_interface::roles;
use type_flyweight::roles::Role;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = roles::create_table(&mut conn) {
        assert!(false, "failed to create roles table");
    }

    let incorrect_role = Role {
        id: 0,
        kind: "lovers stand sublime together".to_string(),
        deleted_at: None,
    };

    // create
    let role = match roles::create(&mut conn, 1, "lovers thatch time together") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let role_read_by_id = match roles::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != role);
    assert!(role == role_read_by_id);
    assert!(Some(incorrect_role.clone()) != role_read_by_id);

    // read by kind
    let role_read_by_kind = match roles::read_by_kind(&mut conn, "lovers thatch time together")
    {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != role);
    assert!(role == role_read_by_kind);
    assert!(Some(incorrect_role.clone()) != role_read_by_kind);

    Ok(())
}
