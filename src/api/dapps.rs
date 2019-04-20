use http::client::Client;
use std::borrow::Borrow;

use api::models::Dapp;
use api::Result;

pub struct Dapps {
    client: Client,
}

impl Dapps {
    pub fn new(client: Client) -> Dapps {
        Dapps { client }
    }

    pub fn all(&self) -> Result<Vec<Dapp>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Dapp>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("dapps", parameters)
    }
}
