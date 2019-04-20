use serde_json::{from_str, Value};
use *;

#[test]
fn test_all_blocks() {
    let (_mock, body) = mock_http_request("blocks");
    {
        let client = mock_client();
        let actual = client.blocks.all().unwrap();
        let expected: Value = from_str(&body).unwrap();
        print!("{:?}", actual);
        print!("{:?}", expected);
    }
}
