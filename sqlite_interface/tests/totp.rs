use rusqlite::{Connection, Result};
use sqlite_interface::totp;
use type_flyweight::totp::Totp;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = totp::create_table(&mut conn) {
        assert!(false, "failed to create totp table");
    }

    let incorrect_totp = Totp {
        id: 0,
        people_id: 64,
        secret_key: "great strides make thicc thighs".to_string(),
        algorithm: None,
        period: None,
        digits: None,
        deleted_at: None,
    };

    // create
    let totp = match totp::create(&mut conn, 1, 64, "walk your heart into the sea") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read by id
    let totp_read_by_id = match totp::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != totp);
    assert!(totp == totp_read_by_id);
    assert!(Some(incorrect_totp) != totp_read_by_id);

    Ok(())
}
