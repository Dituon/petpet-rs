use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "address_default")]
    pub address: String,
    #[serde(default = "data_path_default", rename="dataPath")]
    pub data_path: Vec<String>,
}

impl ServerConfig {
    pub fn read_or_save(path: &str) -> ServerConfig {
        if let Ok(str) = std::fs::read_to_string(path) {
            let jd = &mut serde_json::Deserializer::from_str(&str);
            serde_path_to_error::deserialize(jd)
                .unwrap_or_else(|_| save_config(path))
        } else {
            save_config(path)
        }
    }
}

fn save_config(path: &str) -> ServerConfig {
    let mut file = File::create(path).unwrap();
    let default_config = ServerConfig {
        address: address_default(),
        data_path: data_path_default(),
    };
    let _ = serde_json::to_writer_pretty(&mut file, &default_config);
    default_config
}

fn address_default() -> String {
    "0.0.0.0:3000".to_string()
}

fn data_path_default() -> Vec<String> {
    vec!["./data".to_string()]
}