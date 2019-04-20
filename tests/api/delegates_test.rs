use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_delegates() {
    let (_mock, body) = mock_http_request("delegates");
    {
        let client = mock_client();
        let actual = client.delegates.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].username,
            expected["data"][0]["username"].as_str().unwrap()
        );
    }
}

#[test]
fn test_all_params_delegates() {
    let (_mock, body) = mock_http_request("delegates");
    {
        let client = mock_client();
        let params = [("limit", "1")].iter();
        let actual = client.delegates.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].username,
            expected["data"][0]["username"].as_str().unwrap()
        );
    }
}
