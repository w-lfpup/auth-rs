use rusqlite::{Connection, Result};
use sqlite_interface::roles_to_people;
use type_flyweight::roles::RoleToPerson;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = roles_to_people::create_table(&mut conn) {
        assert!(false, "failed to create roles table");
    }

    let incorrect_role_to_person = RoleToPerson {
        id: 0,
        role_id: 36,
        people_id: 42,
        deleted_at: None,
    };

    // create
    let role_to_person = match roles_to_people::create(&mut conn, 1, 31, 41) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let role_to_person_read_by_id = match roles_to_people::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != role_to_person);
    assert!(role_to_person == role_to_person_read_by_id);
    assert!(Some(incorrect_role_to_person.clone()) != role_to_person_read_by_id);

    // read by kind and content
    let role_to_person_read_by_role_id_and_people_id =
        match roles_to_people::read_by_role_id_and_people_id(&mut conn, 31, 41) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(None != role_to_person);
    assert!(role_to_person == role_to_person_read_by_role_id_and_people_id);
    assert!(Some(incorrect_role_to_person.clone()) != role_to_person_read_by_role_id_and_people_id);

    Ok(())
}
