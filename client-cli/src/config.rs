use std::fs::read_to_string;

pub fn get_lite_server_config(config: &str) -> String {
    match reqwest::Url::parse(config) {
        Ok(config_url) => {
            log::debug!("init config from url: {config_url}");

            let config_base64 = base64::encode_config(config_url.as_str(), base64::URL_SAFE);
            let cached_path = std::env::temp_dir()
                .join("ton-rs")
                .join(config_base64.as_str());
            if cached_path.exists() {
                log::debug!(
                    "cached config exists by path: {}, read from cache",
                    cached_path.display()
                );
                return read_to_string(cached_path).unwrap();
            }

            log::debug!("get config from url: {config_url}");
            let config_text = reqwest::blocking::get(config_url).unwrap().text().unwrap();

            log::debug!("cache config to file: {}", cached_path.display());
            std::fs::create_dir_all(cached_path.parent().unwrap()).unwrap();
            std::fs::write(cached_path, &config_text).unwrap();

            return config_text;
        }
        Err(_) => read_to_string(config).unwrap(),
    }
}
