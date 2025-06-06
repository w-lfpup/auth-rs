use rusqlite::{Connection, Result};
use type_flyweight::contacts::ContactKind;
use sqlite_interface::contact_kinds;

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
    let contact_kind_maybe = match contact_kinds::create(&mut conn, 1, "email") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    let contact_kind = match contact_kind_maybe {
        Some(ck) => ck,
        _ => return Err("failed to create a contact_kind".into())
    };


    // read by id
    let contact_kind_read_by_id_maybe = match contact_kinds::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    let contact_kind_read_by_id = match contact_kind_read_by_id_maybe {
        Some(ck) => ck,
        _ => return Err("failed to read a contact_kind by id".into())
    };

    assert!(contact_kind == contact_kind_read_by_id);
    assert!(incorrect_contact_kind != contact_kind_read_by_id);

        // read by id
    let contact_kind_read_by_kind_maybe = match contact_kinds::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    let contact_kind_read_by_kind = match contact_kind_read_by_kind_maybe {
        Some(ck) => ck,
        _ => return Err("failed to read a contact_kind by id".into())
    };

    assert!(contact_kind == contact_kind_read_by_kind);
    assert!(incorrect_contact_kind != contact_kind_read_by_kind);

    println!("{:?}", contact_kind_read_by_kind);

    Ok(())
}
