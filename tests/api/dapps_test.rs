use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_dapps() {
    let (_mock, body) = mock_http_request("dapps");
    {
        let client = mock_client();
        let actual = client.dapps.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        assert_eq!(
            actual.data[0].name,
            expected["data"][0]["name"].as_str().unwrap()
        );
    }
}
