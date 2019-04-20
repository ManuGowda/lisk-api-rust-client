pub mod accounts;
pub mod blocks;
pub mod dapps;
pub mod delegates;
pub mod models;
pub mod node;
pub mod peers;
pub mod transactions;
pub mod voters;
pub mod votes;

use self::accounts::Accounts;
use self::blocks::Blocks;
use self::dapps::Dapps;
use self::delegates::Delegates;
use self::models::Response;
use self::node::Node;
use self::peers::Peers;
use self::transactions::Transactions;
use self::voters::Voters;
use self::votes::Votes;

use super::error::Error;
use http::client::Client;

pub type Result<T> = std::result::Result<Response<T>, Error>;

pub struct Api {
    pub client: Client,
    pub node: Node,
    pub blocks: Blocks,
    pub dapps: Dapps,
    pub peers: Peers,
    pub accounts: Accounts,
    pub delegates: Delegates,
    pub transactions: Transactions,
    pub voters: Voters,
    pub votes: Votes,
}

impl Api {
    pub fn new(host: &str) -> Api {
        Api::new_with_client(&Client::new(host))
    }

    pub fn new_with_client(client: &Client) -> Api {
        let client = client.clone();

        Api {
            node: Node::new(client.clone()),
            blocks: Blocks::new(client.clone()),
            dapps: Dapps::new(client.clone()),
            peers: Peers::new(client.clone()),
            accounts: Accounts::new(client.clone()),
            delegates: Delegates::new(client.clone()),
            transactions: Transactions::new(client.clone()),
            voters: Voters::new(client.clone()),
            votes: Votes::new(client.clone()),
            client,
        }
    }
}
