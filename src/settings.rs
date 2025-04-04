use serde::Deserialize;
use config::{Config, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub db_commits: String,
    pub db_branches: String,
    pub db_current_branch: String,
    pub files_project: String,
    pub db_tree: String
}

pub fn load_settings() -> Settings {
    let environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "dev".into());

    Config::builder()
        .add_source(File::with_name(&format!("config.{}", environment)))
        .build().expect("Failed to load config")
        .try_deserialize::<Settings>().expect("Error trying deserialization")
}
