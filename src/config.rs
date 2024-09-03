use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub aliases_path: Option<String>,
}

mod tests {
    #[allow(unused_imports)]
    use std::fs;

    #[allow(unused_imports)]
    use crate::config::Config;

    #[test]
    fn test_config_file() {
        let config_path = if let Some(path) = std::env::var_os("STREAMDEX_CONFIG") {
            path.into_string().unwrap()
        } else {
            "/home/alimulap/.config/streamdex/config.toml".to_string()
        };
        let config_str = fs::read_to_string(config_path).unwrap();
        let config = toml::from_str::<Config>(&config_str).unwrap();
        assert!(config.aliases_path.is_some());
    }
}
