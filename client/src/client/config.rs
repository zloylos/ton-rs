#[derive(Debug, Clone)]
pub struct Config {
    pub lite_server_config: String,
    pub keystore_dir: String,
    pub request_timeout: std::time::Duration,
    pub log_level: i8,
}
