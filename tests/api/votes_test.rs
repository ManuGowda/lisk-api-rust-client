use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_votes() {
    let (_mock, body) = mock_http_request("votes");
    {
        let client = mock_client();
        let actual = client.votes.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data.address,
            expected["data"]["address"].as_str().unwrap()
        );
    }
}

#[test]
fn test_all_params_votes() {
    let (_mock, body) = mock_http_request("votes");
    {
        let client = mock_client();
        let params = [("limit", "1")].iter();
        let actual = client.votes.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data.address,
            expected["data"]["address"].as_str().unwrap()
        );
    }
}
