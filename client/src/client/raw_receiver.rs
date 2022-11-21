use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use log::trace;

use super::RawClient;

type ReceiverDataResult = Option<Result<serde_json::Value, String>>;
type ReceiveResult = Result<serde_json::Value, String>;

pub struct RawReceiver {
    raw_client: Arc<Mutex<RawClient>>,
    data: Arc<Mutex<BTreeMap<String, ReceiverDataResult>>>,
    timeout: std::time::Duration,
}

impl RawReceiver {
    pub fn new(raw_client: Arc<Mutex<RawClient>>, timeout: std::time::Duration) -> Self {
        Self {
            raw_client,
            data: Arc::new(Mutex::new(BTreeMap::new())),
            timeout,
        }
    }

    pub fn add_task(&mut self, extra: &str) {
        self.data.lock().unwrap().insert(extra.to_string(), None);
    }

    pub fn receive(&mut self, extra: &str) -> impl futures::Future<Output = ReceiveResult> {
        let data = Arc::clone(&self.data);
        let timeout = self.timeout;
        let extra_str = extra.to_string();

        async move {
            let stop_time = std::time::Instant::now() + timeout;
            while std::time::Instant::now() < stop_time {
                match data.try_lock() {
                    Ok(mut data_unwrapped) => {
                        match data_unwrapped.get(&extra_str) {
                            Some(Some(_)) => {
                                return data_unwrapped.remove(&extra_str).unwrap().unwrap();
                            }
                            _ => {}
                        };
                    }
                    _ => {}
                };
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
            _ = data.lock().unwrap().remove(&extra_str).unwrap();
            return Err("timeout".to_string());
        }
    }

    pub fn start(&mut self) {
        let raw_client = self.raw_client.clone();
        let data = self.data.clone();
        std::thread::Builder::new()
            .name("raw_client_receive_looper".to_owned())
            .spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_millis(2));

                let msg = match raw_client.try_lock() {
                    Ok(mut cl) => cl.receive(),
                    _ => continue,
                };

                if msg.is_none() {
                    continue;
                }

                trace!("reiceived msg: {:?}", msg);
                let json_msg: serde_json::Value = serde_json::from_str(&msg.unwrap()).unwrap();
                let extra_data = json_msg["@extra"].clone();
                let is_error = match json_msg["@type"].as_str() {
                    Some("error") => true,
                    _ => false,
                };
                if extra_data.is_string() {
                    let extra = extra_data.as_str().unwrap();
                    let mut d = data.lock().unwrap();
                    if !d.contains_key(extra) {
                        continue;
                    }

                    let res = match !is_error {
                        true => Ok(json_msg),
                        false => Err(json_msg["message"].as_str().unwrap_or_default().to_string()),
                    };
                    d.insert(extra.to_string(), Some(res));
                }
            })
            .unwrap();
    }
}
