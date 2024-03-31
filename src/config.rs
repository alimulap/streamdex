use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub aliases_path: Option<String>
}

mod tests {
    #[allow(unused_imports)]
    use std::fs;

    #[allow(unused_imports)]
    use crate::config::Config;

    #[test]
    fn test_config_file() {
        let config_str = fs::read_to_string("/home/alimulap/.config/streamdex/config.toml").unwrap();
        let config = toml::from_str::<Config>(&config_str).unwrap();
        assert!(config.aliases_path.is_some());
    }
}
