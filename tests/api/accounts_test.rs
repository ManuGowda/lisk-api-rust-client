use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_accounts() {
    let (_mock, body) = mock_http_request("accounts");
    {
        let client = mock_client();
        let actual = client.accounts.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].address,
            expected["data"][0]["address"].as_str().unwrap()
        );
    }
}

#[test]
fn test_all_params_accounts() {
    let (_mock, body) = mock_http_request("accounts");
    {
        let client = mock_client();
        let params = [("limit", "1")].iter();
        let actual = client.accounts.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].address,
            expected["data"][0]["address"].as_str().unwrap()
        );
    }
}
