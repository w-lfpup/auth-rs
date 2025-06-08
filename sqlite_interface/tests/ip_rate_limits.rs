use rusqlite::{Connection, Result};
use sqlite_interface::ip_address_rate_limits;
use type_flyweight::ip_address_rate_limits::IpAddressRateLimit;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = ip_address_rate_limits::create_table(&mut conn) {
        assert!(false, "failed to create public_session table");
    }

    let incorrect_incorrect_ip_address_rate_limit = IpAddressRateLimit {
        ip_address: "127.0.0.1".to_string(),
        window_count: 42,
        prev_window_count: 51,
        updated_at: 27,
        deleted_at: None,
    };

    // create
    let ip_address_rate_limit =
        match ip_address_rate_limits::rate_limit_ip_address(&mut conn, "127.0.0.1", 20, 10, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(None != ip_address_rate_limit);

    // second rate limit
    let second_ip_address_rate_limit =
        match ip_address_rate_limits::rate_limit_ip_address(&mut conn, "127.0.0.1", 20, 10, 5) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(ip_address_rate_limit != second_ip_address_rate_limit);

    Ok(())
}
