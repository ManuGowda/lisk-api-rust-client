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
pub struct Response<T> {
    pub data: T,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct TransactionsInPool {
    pub confirmed   : u64,
    pub unconfirmed : u64,
    pub unprocessed : u64,
    pub unsigned    : u64,
    pub total   : u64
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    pub broadhash: String,
    pub consensus   : u64,
    pub current_time : u64,
    pub seconds_since_epoch   : u64,
    pub height  : u64,
    pub loaded  : bool,
    pub network_height   : i64,
    pub syncing : bool,
    pub transactions: TransactionsInPool,
}
