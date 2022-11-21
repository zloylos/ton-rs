#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BlockId {
    pub file_hash: String,
    pub root_hash: String,
    pub seqno: usize,
    pub shard: String,
    pub workchain: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TransactionId {
    pub hash: String,
    pub lt: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AccountState {
    pub balance: String,
    pub block_id: BlockId,
    pub code: String,
    pub data: String,
    pub frozen_hash: String,
    pub last_transaction_id: TransactionId,
    pub sync_utime: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Transactions {
    pub previous_transaction_id: Option<TransactionId>,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    pub transaction_id: TransactionId,
    pub data: String,
    pub fee: String,
    pub other_fee: String,
    pub storage_fee: String,
    pub utime: usize,

    pub in_msg: TransactionMessage,
    pub out_msgs: Vec<TransactionMessage>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TransactionMessage {
    destination: TransactionMessageAddress,
    source: TransactionMessageAddress,
    pub body_hash: String,
    pub created_lt: String,
    pub fwd_fee: String,
    pub ihr_fee: String,
    pub value: String,
}

impl TransactionMessage {
    pub fn source(&self) -> &str {
        return self.source.account_address.as_str();
    }

    pub fn destination(&self) -> &str {
        return self.destination.account_address.as_str();
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TransactionMessageAddress {
    pub account_address: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MasterChainInfo {
    pub init: BlockId,
    pub last: BlockId,
    pub state_root_hash: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AccountTransactionId {
    pub account: String,
    pub lt: usize,
}
