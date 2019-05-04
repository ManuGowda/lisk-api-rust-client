use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub count: Option<u64>,
    pub from_timestamp: Option<u64>,
    pub to_timestamp: Option<u64>,
    pub last_block: Option<u64>,
    pub last_block_slot: Option<u64>,
    pub current_slot: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub data: T,
    pub meta: Meta,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamErrorMessage {
    pub code: String,
    pub message: String,
    pub description: String,
    pub path: Vec<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamError {
    pub code: String,
    pub name: String,
    pub message: String,
    pub errors: Vec<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParamErrorResponse {
    pub message: String,
    pub errors: Vec<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct TransactionsInPool {
    pub confirmed: u64,
    pub unconfirmed: u16,
    pub unprocessed: u16,
    pub unsigned: u16,
    pub total: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub public_key: String,
    pub balance: Option<String>,
    pub unconfirmed_balance: Option<String>,
    pub second_public_key: Option<String>,
    pub min: Option<u8>,
    pub lifetime: Option<u32>,
    pub delegate: Option<DelegateWithAccount>,
    pub members: Option<Vec<Member>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub address: String,
    pub public_key: String,
    pub second_public_key: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub version: u64,
    pub timestamp: u64,
    pub height: u64,
    pub number_of_transactions: u64,
    pub total_amount: String,
    pub total_fee: String,
    pub reward: String,
    pub payload_length: u64,
    pub payload_hash: String,
    pub generator_public_key: String,
    pub block_signature: String,
    pub confirmations: u64,
    pub total_forged: String,
    pub generator_address: String,
    pub previous_block_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub height: u64,
    pub block_id: String,
    pub r#type: u8,
    pub timestamp: u64,
    pub sender_public_key: String,
    pub recipient_public_key: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub amount: String,
    pub fee: String,
    pub signature: String,
    pub signatures: Option<Vec<Signature>>,
    #[serde(skip_serializing_if = "Asset::is_none")]
    pub asset: Asset,
    pub confirmations: u64,
    pub sender_second_public_key: Option<String>,
    pub sign_signature: Option<String>,
    pub relays: Option<u16>,
    pub ready: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    pub transaction_id: String,
    pub public_key: String,
    pub signature: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dapp {
    pub transaction_id: String,
    pub icon: String,
    pub category: u8,
    pub r#type: u8,
    pub link: String,
    pub tags: String,
    pub description: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub ip: String,
    pub http_port: Option<u64>,
    pub ws_port: u64,
    pub os: Option<String>,
    pub version: String,
    pub protocol_version: Option<String>,
    pub state: Option<u64>,
    pub height: Option<u64>,
    pub broadhash: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConstants {
    pub epoch: String,
    pub milestone: String,
    pub build: String,
    pub commit: String,
    pub version: String,
    pub protocol_version: String,
    pub nethash: String,
    pub supply: String,
    pub reward: String,
    pub nonce: String,
    pub fees: Fees,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub send: String,
    pub vote: String,
    pub second_signature: String,
    pub delegate: String,
    pub multisignature: String,
    pub dapp_registration: String,
    pub dapp_withdrawal: String,
    pub dapp_deposit: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    pub broadhash: String,
    pub consensus: u64,
    pub current_time: u64,
    pub seconds_since_epoch: u64,
    pub height: u64,
    pub loaded: bool,
    pub network_height: i64,
    pub syncing: bool,
    pub transactions: TransactionsInPool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgingStats {
    pub fees: String,
    pub rewards: String,
    pub forged: String,
    pub count: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgingStatus {
    pub forging: bool,
    pub public_key: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forger {
    pub username: String,
    pub address: String,
    pub public_key: String,
    pub next_slot: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWithAccount {
    pub username: String,
    pub vote: String,
    pub rewards: String,
    pub produced_blocks: u32,
    pub missed_blocks: u32,
    pub rank: u16,
    pub productivity: f32,
    pub approval: f32,
    pub account: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWithVoters {
    pub address: String,
    pub public_key: String,
    pub balance: String,
    pub votes: u32,
    pub username: String,
    pub voters: Vec<Voter>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWithVotes {
    pub address: String,
    pub balance: String,
    pub username: String,
    pub public_key: String,
    pub votes_used: u32,
    pub votes_available: u32,
    pub votes: Vec<Vote>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Voter {
    pub address: String,
    pub public_key: String,
    pub balance: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vote {
    pub address: String,
    pub public_key: String,
    pub balance: String,
    pub username: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Asset {
    #[serde(skip)]
    None,
    Data(String),
    Signature {
        #[serde(rename = "publicKey")]
        public_key: String,
    },
    Delegate {
        #[serde(rename = "publicKey")]
        public_key: String,
        username: String,
        address: String,
    },
    Votes(Vec<String>),
    Multisignature {
        min: u8,
        lifetime: u64,
        keysgroup: Vec<String>,
    },
    Dapp {
        icon: String,
        link: String,
        name: String,
        tags: String,
        r#type: u8,
        category: u8,
        description: String,
    },
}

impl Asset {
    pub fn is_none(&self) -> bool {
        match *self {
            Asset::None => true,
            _ => false,
        }
    }
}

impl Default for Asset {
    fn default() -> Self {
        Asset::None
    }
}
