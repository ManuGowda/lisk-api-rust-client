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
    pub balance: Option<u64>,
    pub unconfirmed_balance: u64,
    pub second_public_key: String,
    pub min: u8,
    pub lifetime: u32,
    pub delegate: Option<Delegate>,
    pub members: Option<Vec<Member>>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub address: String,
    pub public_key: String,
    pub second_public_key: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub version: u8,
    pub height: u64,
    pub timestamp: u64,
    pub generator_address: String,
    pub generator_public_key: String,
    pub payload_length: u16,
    pub payload_hash: String,
    pub block_signature: String,
    pub confirmations: u16,
    pub previous_block_id: String,
    pub number_of_transactions: u16,
    pub total_amount: u64,
    pub total_fee: u64,
    pub reward: u64,
    pub total_forged: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub fee: u64,
    pub r#type: u8,
    pub height: u64,
    pub block_id: String,
    pub timestamp: String,
    pub sender_id: String,
    pub sender_public_key: String,
    pub sender_second_public_key: String,
    pub recipient_id: String,
    pub recipient_public_key: String,
    pub signature: String,
    pub sign_signature: String,
    pub signatures: Signature,
    pub confirmations: u64,
    pub asset: HashMap<String, String>,
    pub relays: u16,
    pub ready: bool,
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
    pub http_port: u8,
    pub ws_port: u8,
    pub os: String,
    pub version: String,
    pub protocol_version: String,
    pub state: u8,
    pub height: u64,
    pub broadhash: String,
    pub nonce: String,
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
    pub send: u64,
    pub vote: u64,
    pub second_signature: u64,
    pub delegate: u64,
    pub multisignature: u64,
    pub dapp_registration: u64,
    pub dapp_withdrawal: u64,
    pub dapp_deposit: u64,
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
pub struct Delegate {
    pub username: String,
    pub vote: u64,
    pub rewards: u64,
    pub produced_blocks: u64,
    pub missed_blocks: u64,
    pub approval: f32,
    pub productivity: f32,
    pub rank: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateWithAccount {
    pub username: String,
    pub vote: u64,
    pub rewards: String,
    pub produced_blocks: u32,
    pub missed_blocks: u32,
    pub approval: f32,
    pub productivity: f32,
    pub account: Account,
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
pub struct Voter {
    pub address: String,
    pub public_key: String,
    pub balance: String,
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
pub struct Vote {
    pub address: String,
    pub public_key: String,
    pub balance: String,
    pub username: String,
}
