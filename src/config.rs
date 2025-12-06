use std::{env, fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub aliases_path: String,
    pub new_aliases: PathBuf,
    pub client_secret: PathBuf,
    pub presist_token: PathBuf,
    pub saved_yt_channel_ids: PathBuf,
}

impl Config {
    pub fn get() -> Self {
        let config_path = env::var("STREAMDEX_CONFIG").unwrap_or({
            let home = std::env::var("HOME").unwrap_or(std::env::var("USERPROFILE").unwrap());
            format!("{home}/.config/streamdex/config.toml")
        });
        let config_str = fs::read_to_string(&config_path)
            .inspect_err(|e| match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("Config file not found at: {}", &config_path);
                    std::process::exit(1);
                }
                _ => eprintln!("Error reading config file: {}", e),
            })
            .unwrap();
        toml::from_str::<Config>(&config_str).unwrap()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::config::Config;
//     use std::fs;

//     #[test]
//     fn test_config_file() {
//         let config_path = if let Some(path) = std::env::var_os("STREAMDEX_CONFIG") {
//             path.into_string().unwrap()
//         } else {
//             "/home/alimulap/.config/streamdex/config.toml".to_string()
//         };
//         let config_str = fs::read_to_string(config_path).unwrap();
//         let config = toml::from_str::<Config>(&config_str).unwrap();
//         assert!(config.aliases_path.is_some());
//     }
// }
