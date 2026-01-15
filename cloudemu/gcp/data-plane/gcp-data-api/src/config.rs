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
        let port = env::var("GCP_DATA_PORT")
            .unwrap_or_else(|_| "4567".to_string()) // Distinct port from Azure
            .parse()
            .unwrap_or(4567);
            
        let host = env::var("GCP_DATA_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
            
        let data_dir = env::var("GCP_DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_default();
                PathBuf::from(home).join(".cloudemu").join("gcp")
            });

        Self {
            port,
            host,
            data_dir,
        }
    }
}
