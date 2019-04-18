use serde_json::{from_str, Value};
use *;

#[test]
fn test_status() {
    let (_mock, body) = mock_http_request("node/status");
    {
        let client = mock_client();
        let actual = client.node.status().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data.broadhash,
            expected["data"]["broadhash"].as_str().unwrap()
        );

        assert_eq!(
            actual.data.consensus,
            expected["data"]["consensus"].as_u64().unwrap()
        );
    }
}

#[test]
fn test_constants() {
    let (_mock, body) = mock_http_request("node/constants");
    {
        let client = mock_client();
        let actual = client.node.constants().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.nethash,
            expected["data"]["nethash"].as_str().unwrap()
        );

        assert_eq!(
            actual.data.version,
            expected["data"]["version"].as_str().unwrap()
        );
    }
}
