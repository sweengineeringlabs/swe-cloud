use std::path::PathBuf;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub data_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let port = env::var("AZURE_DATA_PORT")
            .unwrap_or_else(|_| "4566".to_string())
            .parse()
            .unwrap_or(4566);
            
        let host = env::var("AZURE_DATA_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
            
        let data_dir = env::var("AZURE_DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_default();
                PathBuf::from(home).join(".cloudemu").join("azure")
            });

        Self {
            port,
            host,
            data_dir,
        }
    }
}
