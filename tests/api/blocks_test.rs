use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_blocks() {
    let (_mock, body) = mock_http_request("blocks");
    {
        let client = mock_client();
        let actual = client.blocks.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].id,
            expected["data"][0]["id"].as_str().unwrap()
        );
    }
}

#[test]
fn test_all_params_blocks() {
    let (_mock, body) = mock_http_request("blocks");
    {
        let client = mock_client();
        let params = [("limit", "1")].iter();
        let actual = client.blocks.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].id,
            expected["data"][0]["id"].as_str().unwrap()
        );
    }
}
