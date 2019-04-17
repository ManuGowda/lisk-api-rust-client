pub mod models;
pub mod node;

use self::models::Response;
use self::node::Node;

use super::error::Error;
use http::client::Client;

pub type Result<T> = std::result::Result<Response<T>, Error>;

pub struct Api {
    pub node: Node,
    pub client: Client,
}

impl Api {
    pub fn new(host: &str) -> Api {
        Api::new_with_client(&Client::new(host))
    }

    pub fn new_with_client(client: &Client) -> Api {
        let client = client.clone();

        Api {
            node: Node::new(client.clone()),
            client,
        }
    }
}