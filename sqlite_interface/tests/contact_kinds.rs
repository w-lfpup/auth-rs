use rusqlite::{Connection, Result};
use sqlite_interface::contact_kinds;
use type_flyweight::contacts::ContactKind;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = contact_kinds::create_table(&mut conn) {
        assert!(false, "failed to create contact_kinds table");
    }

    let incorrect_contact_kind = ContactKind {
        id: 0,
        kind: "paper_cup_and_string".to_string(),
        deleted_at: None,
    };

    // create
    let mut contact_kind = match contact_kinds::create(&mut conn, 1, "email") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let mut contact_kind_read_by_id = match contact_kinds::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(contact_kind == contact_kind_read_by_id);
    assert!(Some(incorrect_contact_kind.clone()) != contact_kind_read_by_id);

    // read by kind
    let mut contact_kind_read_by_kind = match contact_kinds::read_by_kind(&mut conn, "email") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(contact_kind == contact_kind_read_by_kind);
    assert!(Some(incorrect_contact_kind.clone()) != contact_kind_read_by_kind);

    Ok(())
}
