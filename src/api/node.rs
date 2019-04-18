use api::models::{NodeConstants, NodeStatus};
use api::Result;
use http::client::Client;

pub struct Node {
    client: Client,
}

impl Node {
    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(&self) -> Result<NodeStatus> {
        self.client.get("node/status")
    }

    pub fn constants(&self) -> Result<NodeConstants> {
        self.client.get("node/constants")
    }
}
