use rusqlite::{Connection, Result};
use sqlite_interface::people;
use type_flyweight::people::Person;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = people::create_table(&mut conn) {
        assert!(false, "failed to create people table");
    }

    let incorrect_person = Person {
        id: 0,
        password_hash_results: "fold a piece of paper into everything you love".to_string(),
        deleted_at: None,
    };

    // create
    let mut person = match people::create(
        &mut conn,
        1,
        "fold a piece of paper into something that you love",
    ) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let mut person_read_by_id = match people::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(person == person_read_by_id);
    assert!(Some(incorrect_person) != person_read_by_id);

    Ok(())
}
