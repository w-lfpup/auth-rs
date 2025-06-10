use rusqlite::{Connection, Result};
use sqlite_interface::ip_address_action_kinds;
use type_flyweight::ip_address_rate_limits::IpAddressActionKind;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = ip_address_action_kinds::create_table(&mut conn) {
        assert!(false, "failed to create ip_address_action_kinds table");
    }

    let incorrect_ip_address_action_kind = IpAddressActionKind {
        id: 0,
        kind: "foul and filth preach sublime".to_string(),
        deleted_at: None,
    };

    // create
    let ip_address_action_kind =
        match ip_address_action_kinds::create(&mut conn, 1, "create_account") {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    // read by id
    let ip_address_action_kind_read_by_id = match ip_address_action_kinds::read(&mut conn, 1) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != ip_address_action_kind);
    assert!(ip_address_action_kind == ip_address_action_kind_read_by_id);
    assert!(Some(incorrect_ip_address_action_kind.clone()) != ip_address_action_kind_read_by_id);

    // read by kind
    let ip_address_action_kind_read_by_kind =
        match ip_address_action_kinds::read_by_kind(&mut conn, "create_account") {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(None != ip_address_action_kind);
    assert!(ip_address_action_kind == ip_address_action_kind_read_by_kind);
    assert!(Some(incorrect_ip_address_action_kind.clone()) != ip_address_action_kind_read_by_kind);

    Ok(())
}
