extern crate lisk_api_rust_client;

use lisk_api_rust_client::connection::Connection;

const MOCK_HOST: &str = "http://127.0.0.1:4000/api/";

#[test]
fn test_connection() {
    let connection = Connection::new(MOCK_HOST);

    assert_eq!(connection.client.host, MOCK_HOST);
}
