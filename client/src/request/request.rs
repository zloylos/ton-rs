use serde_json::json;

use crate::types;

pub trait Request {
    fn serialize(&self, extra: &str) -> String;
}

pub struct Init {
    pub lite_server_config: String,
    pub key_store_directory: String,
}

impl Request for Init {
    fn serialize(&self, extra: &str) -> String {
        let query = json!({
          "@type": "init",
          "@extra": extra,
          "options": {
            "@type": "options",
            "config": {
              "@type": "config",
              "config": self.lite_server_config,
              "use_callbacks_for_network": false,
              "blockchain_name": "",
              "ignore_cache": false
            },
            "keystore_type": {
              "@type": "keyStoreTypeDirectory",
              "directory": self.key_store_directory
            }
          }
        });
        serde_json::to_string(&query).unwrap()
    }
}

pub struct AccountState {
    pub address: String,
}

impl Request for AccountState {
    fn serialize(&self, extra: &str) -> String {
        json!({
          "@type": "raw.getAccountState",
          "@extra": extra,
          "account_address": {
            "account_address": self.address
          }
        })
        .to_string()
    }
}

pub struct Transactions {
    pub address: String,
    pub from_transaction_lt: Option<String>,
    pub from_transaction_hash: Option<String>,
}

impl Request for Transactions {
    fn serialize(&self, extra: &str) -> String {
        json!({
          "@type": "raw.getTransactions",
          "@extra": extra,
          "account_address": {
            "account_address": self.address
          },
          "from_transaction_id": {
              "@type": "internal.transactionId",
              "lt": self.from_transaction_lt,
              "hash": self.from_transaction_hash
          }
        })
        .to_string()
    }
}

pub struct MasterChainInfo {}

impl Request for MasterChainInfo {
    fn serialize(&self, extra: &str) -> String {
        json!({
          "@type": "blocks.getMasterchainInfo",
          "@extra": extra
        })
        .to_string()
    }
}

pub struct SyncClient {}

impl Request for SyncClient {
    fn serialize(&self, extra: &str) -> String {
        json!({
          "@type": "sync",
          "@extra": extra
        })
        .to_string()
    }
}

pub struct BlockTransactions {
    pub fullblock: Option<String>,
    pub count: usize,
    pub after_tx: Option<types::AccountTransactionId>,
}

impl Request for BlockTransactions {
    fn serialize(&self, extra: &str) -> String {
        let mut mode: usize = 7;
        let mut after_tx = json!({
          "@type": "blocks.accountTransactionId",
          "account": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
          "lt": 0
        });

        if let Some(account_tx_id) = self.after_tx.as_ref() {
            mode = 7 + 128;
            after_tx["account"] = json!(account_tx_id.account);
            after_tx["lt"] = json!(account_tx_id.lt);
        }

        return json!({
            "@type": "blocks.transactions",
            "@extra": extra,
            "id": self.fullblock,
            "mode": mode,
            "count": self.count,
            "after": after_tx
        })
        .to_string();
    }
}

pub struct LookupBlock {
    pub workchain: i32,
    pub shard: String,
    pub lt: Option<String>,
    pub utime: Option<usize>,
    pub seqno: Option<usize>,
}

impl Request for LookupBlock {
    fn serialize(&self, extra: &str) -> String {
        let mut mode = 0;
        if self.seqno.is_some() {
            mode += 1;
        }
        if self.lt.is_some() {
            mode += 2;
        }
        if self.utime.is_some() {
            mode += 4
        }

        json!({
          "@type": "blocks.lookupBlock",
          "@extra": extra,
          "mode": mode,
          "id": {
              "@type": "ton.blockId",
              "workchain": self.workchain,
              "shard": self.shard,
              "seqno": self.seqno
          },
          "lt": self.lt,
          "utime": self.utime
        })
        .to_string()
    }
}
