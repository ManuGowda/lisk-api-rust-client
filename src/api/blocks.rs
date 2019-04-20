use http::client::Client;
use std::borrow::Borrow;

use api::models::Block;
use api::Result;

pub struct Blocks {
    client: Client,
}

impl Blocks {
    pub fn new(client: Client) -> Blocks {
        Blocks { client }
    }

    pub fn all(&self) -> Result<Vec<Block>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Block>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("blocks", parameters)
    }
}
