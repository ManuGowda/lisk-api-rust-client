use api::models::{ForgingStats, ForgingStatus, NodeConstants, NodeStatus};
use api::Result;
use http::client::Client;
use std::collections::HashMap;

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

    pub fn forging_status(&self, public_key: &str) -> Result<ForgingStats> {
        let endpoint = format!("node/status/forging?publicKey={}", public_key);
        self.client.get(&endpoint)
    }

    pub fn update_forging_status(
        &self,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<ForgingStatus> {
        self.client.post("node/status/forging", data)
    }
}
