use http::client::Client;
use std::borrow::Borrow;

use api::models::Account;
use api::Result;

pub struct Accounts {
    client: Client,
}

impl Accounts {
    pub fn new(client: Client) -> Accounts {
        Accounts { client }
    }

    pub fn all(&self) -> Result<Vec<Account>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Account>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("accounts", parameters)
    }
}
