use http::client::Client;
use std::borrow::Borrow;

use api::models::DelegateWithAccount;
use api::Result;

pub struct Delegates {
    client: Client,
}

impl Delegates {
    pub fn new(client: Client) -> Delegates {
        Delegates { client }
    }

    pub fn all(&self) -> Result<Vec<DelegateWithAccount>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<DelegateWithAccount>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("delegates", parameters)
    }
}
