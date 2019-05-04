use http::client::Client;
use std::borrow::Borrow;
use std::collections::HashMap;

use api::models::Transaction;
use api::Result;

pub struct Transactions {
    client: Client,
}

impl Transactions {
    pub fn new(client: Client) -> Transactions {
        Transactions { client }
    }

    pub fn all(&self) -> Result<Vec<Transaction>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("transactions", parameters)
    }

    pub fn create(&self, transactions: Vec<&str>) -> Result<Transaction> {
        let mut payload = HashMap::<&str, Vec<&str>>::new();
        payload.insert("transactions", transactions);

        self.client.post("transactions", Some(payload))
    }
}
