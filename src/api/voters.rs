use http::client::Client;
use std::borrow::Borrow;

use api::models::DelegateWithVoters;
use api::Result;

pub struct Voters {
    client: Client,
}

impl Voters {
    pub fn new(client: Client) -> Voters {
        Voters { client }
    }

    pub fn all(&self) -> Result<DelegateWithVoters> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<DelegateWithVoters>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("voters", parameters)
    }
}
