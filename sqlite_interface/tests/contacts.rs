use rusqlite::{Connection, Result};
use sqlite_interface::contacts;
use type_flyweight::contacts::Contact;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = contacts::create_table(&mut conn) {
        assert!(false, "failed to create contacts table");
    }

    let incorrect_contact = Contact {
        id: 0,
        people_id: 42,
        contact_kind_id: 7,
        content: "blah@blah.blah".to_string(),
        verified_at: None,
        deleted_at: None,
    };

    // create
    let mut contact = match contacts::create(&mut conn, 1, 2, 3, "email@email.email", None) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let mut contact_read_by_id = match contacts::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(contact == contact_read_by_id);
    assert!(Some(incorrect_contact.clone()) != contact_read_by_id);

    // read by kind and content
    let mut contact_read_by_kind =
        match contacts::read_by_kind_id_and_content(&mut conn, 3, "email@email.email") {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(contact == contact_read_by_kind);
    assert!(Some(incorrect_contact.clone()) != contact_read_by_kind);

    Ok(())
}
