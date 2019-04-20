use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_transactions() {
    let (_mock, body) = mock_http_request("transactions");
    {
        let client = mock_client();
        let actual = client.transactions.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].id,
            expected["data"][0]["id"].as_str().unwrap()
        );
    }
}

#[test]
fn test_all_params_transactions() {
    let (_mock, body) = mock_http_request("transactions");
    {
        let client = mock_client();
        let params = [("limit", "1")].iter();
        let actual = client.transactions.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].id,
            expected["data"][0]["id"].as_str().unwrap()
        );
    }
}
