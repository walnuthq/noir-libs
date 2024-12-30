use ::config::{Config, Environment, File};
use config::Settings;

pub mod config;
pub mod filesystem;
pub mod manifest;
pub mod network;
pub mod ops;
pub mod path;

pub const MANIFEST_FILE_NAME: &str = "Nargo.toml";

pub fn load_settings() -> Settings {
    let settings = Config::builder()
        .add_source(File::with_name("config")) // Load from config file (config.toml, config.json, etc.)
        .add_source(Environment::with_prefix("APP")) // Load from environment variables with prefix "APP"
        .build() // Build the configuration
        .unwrap(); // Handle errors appropriately in production code

    // Deserialize into the Settings struct
    settings.try_deserialize::<Settings>().unwrap()
}
