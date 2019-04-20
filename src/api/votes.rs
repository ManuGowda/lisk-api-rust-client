use http::client::Client;
use std::borrow::Borrow;

use api::models::DelegateWithVotes;
use api::Result;

pub struct Votes {
    client: Client,
}

impl Votes {
    pub fn new(client: Client) -> Votes {
        Votes { client }
    }

    pub fn all(&self) -> Result<DelegateWithVotes> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<DelegateWithVotes>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("votes", parameters)
    }
}
