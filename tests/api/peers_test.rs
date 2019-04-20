use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_peers() {
    let (_mock, body) = mock_http_request("peers");
    {
        let client = mock_client();
        let actual = client.peers.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].ip,
            expected["data"][0]["ip"].as_str().unwrap()
        );
    }
}
