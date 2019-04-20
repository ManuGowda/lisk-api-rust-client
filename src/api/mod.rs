pub mod blocks;
pub mod dapps;
pub mod models;
pub mod node;
pub mod peers;

use self::blocks::Blocks;
use self::dapps::Dapps;
use self::models::Response;
use self::node::Node;
use self::peers::Peers;

use super::error::Error;
use http::client::Client;

pub type Result<T> = std::result::Result<Response<T>, Error>;

pub struct Api {
    pub node: Node,
    pub client: Client,
    pub blocks: Blocks,
    pub dapps: Dapps,
    pub peers: Peers,
}

impl Api {
    pub fn new(host: &str) -> Api {
        Api::new_with_client(&Client::new(host))
    }

    pub fn new_with_client(client: &Client) -> Api {
        let client = client.clone();

        Api {
            node: Node::new(client.clone()),
            blocks: Blocks::new(client.clone()),
            dapps: Dapps::new(client.clone()),
            peers: Peers::new(client.clone()),
            client,
        }
    }
}
