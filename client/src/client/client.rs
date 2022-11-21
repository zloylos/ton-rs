use std::sync::{Arc, Mutex};

use log::{debug, info};

use crate::{request, types};

use super::{raw_receiver::RawReceiver, Config, RawClient};

pub struct Client {
    config: Config,
    raw_client: Arc<Mutex<RawClient>>,
    receiver: RawReceiver,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        let raw_client = Arc::new(Mutex::new(RawClient::new(config.log_level)));
        let receiver = RawReceiver::new(raw_client.clone(), config.request_timeout);

        let mut client = Self {
            config: config.clone(),
            raw_client,
            receiver,
        };
        client.init();
        return client;
    }

    pub fn send(&mut self, request: impl request::Request) -> String {
        let extra_info = request::extra();
        let req_str = request.serialize(&extra_info);

        info!("send request: {extra_info}: {req_str}");

        self.receiver.add_task(extra_info.as_str());

        debug!("receiver add task: {extra_info}");

        self.raw_client.lock().unwrap().send(&req_str);

        debug!("raw_client req sent: {extra_info}");

        return extra_info;
    }

    pub fn get_account_state(
        &mut self,
        address: &str,
    ) -> impl futures::Future<Output = Option<types::AccountState>> {
        let extra = self.send(request::AccountState {
            address: address.to_owned(),
        });
        let response = self.receiver.receive(extra.as_str());
        return async move {
            let result = response.await;
            match result {
                Ok(response) => {
                    let result = serde_json::from_value(response).unwrap();
                    return Some(result);
                }
                Err(err) => {
                    info!("get_account_state error: {err}");
                    return None;
                }
            }
        };
    }

    pub async fn get_transactions(
        &mut self,
        address: &str,
        from_transaction_lt: Option<String>,
        from_transaction_hash: Option<String>,
        to_transaction_lt: Option<String>,
        limit: Option<usize>,
    ) -> Vec<types::Transaction> {
        const DEFAULT_LIMIT: usize = 10;

        let txs_limit = limit.unwrap_or(DEFAULT_LIMIT);
        let to_lt = to_transaction_lt.unwrap_or("0".to_string());

        let mut from_lt = from_transaction_lt.clone();
        let mut from_hash = from_transaction_hash.clone();

        if from_transaction_lt.is_none() || from_transaction_hash.is_none() {
            let maybe_account_state = self.get_account_state(address).await;
            if maybe_account_state.is_none() {
                return Vec::new();
            }
            let account_state = maybe_account_state.unwrap();
            if from_transaction_lt.is_none() {
                _ = from_lt.replace(account_state.last_transaction_id.lt.clone());
            }
            if from_transaction_hash.is_none() {
                _ = from_hash.replace(account_state.last_transaction_id.hash.clone());
            }
        }

        let mut all_transactions = Vec::with_capacity(txs_limit);
        let mut reach_lt = false;
        while !reach_lt && all_transactions.len() < txs_limit {
            let extra = self.send(request::Transactions {
                address: address.to_owned(),
                from_transaction_lt: from_lt.clone(),
                from_transaction_hash: from_hash.clone(),
            });

            match self.receiver.receive(&extra).await {
                Ok(response) => {
                    let transactions: types::Transactions =
                        serde_json::from_value(response).unwrap();
                    let txs = transactions.transactions;
                    if txs.is_empty() {
                        break;
                    }
                    for tx in txs {
                        if tx.transaction_id.lt == to_lt {
                            reach_lt = true;
                            continue;
                        }
                        all_transactions.push(tx);
                    }
                    if let Some(next) = transactions.previous_transaction_id {
                        from_lt.replace(next.lt);
                        from_hash.replace(next.hash);
                    }
                }
                Err(err) => {
                    info!("get_transactions error: {err}");
                    break;
                }
            };
        }

        return all_transactions;
    }

    pub async fn get_master_chain_info(&mut self) -> Option<serde_json::Value> {
        let resp = self.send(request::MasterChainInfo {});
        match self.receiver.receive(&resp).await {
            Ok(response) => {
                let result = serde_json::from_value(response).unwrap();
                return Some(result);
            }
            Err(err) => {
                info!("master_chain_info error: {err}");
                return None;
            }
        };
    }

    pub async fn sync(&mut self) {
        let resp = self.send(request::SyncClient {});
        match self.receiver.receive(&resp).await {
            Ok(response) => {
                info!("sync success, response: {response}");
            }
            Err(err) => {
                info!("sync client error: {err}");
            }
        };
    }

    pub async fn lookup_block(
        &mut self,
        workchain: i32,
        shard: String,
        seqno: Option<usize>,
        lt: Option<String>,
        unix_time: Option<usize>,
    ) -> Result<types::BlockId, String> {
        assert!(seqno.is_some() || lt.is_some() || unix_time.is_some());

        let resp = self.send(request::LookupBlock {
            workchain,
            shard,
            seqno,
            lt,
            utime: unix_time,
        });
        match self.receiver.receive(&resp).await {
            Ok(response) => {
                info!("lookup_block response: {response}");
                let resp: types::BlockId = serde_json::from_value(response).unwrap();
                return Ok(resp);
            }
            Err(err) => {
                let msg = format!("lookup_block error: {err}");
                info!("{msg}");
                return Err(msg);
            }
        };
    }

    fn init(&mut self) {
        let init_extra = self.send(request::Init {
            lite_server_config: self.config.lite_server_config.clone(),
            key_store_directory: self.config.keystore_dir.clone(),
        });
        let fut = self.receiver.receive(init_extra.as_str());
        self.receiver.start();
        match futures::executor::block_on(fut) {
            Ok(response) => {
                info!("init success: {response}");
            }
            _ => self.init(),
        };
    }
}
