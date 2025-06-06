use rusqlite::{Connection, Result};
use sqlite_interface::signups;
use type_flyweight::signups::Signup;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = signups::create_table(&mut conn) {
        assert!(false, "failed to create signups table");
    }

    let incorrect_signup = Signup {
        id: 12341234,
        token: 123123123,
        contact_kind_id: 1,
        contact_content: "blah@blah.blah".to_string(),
        deleted_at: None,
    };

    // create
    let mut signup = match signups::create(&mut conn, 1234, 234234234, 1, "email@email.email") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let mut signup_read_by_id = match signups::read(&mut conn, 1234) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(signup == signup_read_by_id);
    assert!(Some(incorrect_signup.clone()) != signup_read_by_id);

    // read by kind and content
    let mut signup_read_by_contact =
        match signups::read_all_by_contact(&mut conn, 1, "email@email.email", 0, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    // THIS FEELS IFFY BUT IT SHOULD JUST FAIL?
    let sigup_confirmed = match signup {
        Some(su) => su,
        _ => incorrect_signup.clone(),
    };

    assert!(Vec::from([sigup_confirmed]) == signup_read_by_contact);
    assert!(Vec::<Signup>::new() != signup_read_by_contact);

    Ok(())
}
