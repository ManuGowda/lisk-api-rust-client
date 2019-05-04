use http::client::Client;
use std::borrow::Borrow;

use api::models::{DelegateWithAccount, Forger, ForgingStats};
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

    pub fn forgers(&self) -> Result<Vec<Forger>> {
        self.forgers_params(Vec::<(String, String)>::new())
    }

    pub fn forgers_params<I, K, V>(&self, parameters: I) -> Result<Vec<Forger>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("delegates/forgers", parameters)
    }

    pub fn forging_statistics(&self, address: &str) -> Result<Vec<ForgingStats>> {
        self.forging_statistics_params(address, Vec::<(String, String)>::new())
    }

    pub fn forging_statistics_params<I, K, V>(
        &self,
        address: &str,
        parameters: I,
    ) -> Result<Vec<ForgingStats>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("/delegates/{}/forging_statistics", address);
        self.client.get_with_params(&endpoint, parameters)
    }
}
