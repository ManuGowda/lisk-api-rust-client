use http::client::Client;
use std::borrow::Borrow;

use api::models::Peer;
use api::Result;

pub struct Peers {
    client: Client,
}

impl Peers {
    pub fn new(client: Client) -> Peers {
        Peers { client }
    }

    pub fn all(&self) -> Result<Vec<Peer>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Peer>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("peers", parameters)
    }
}
