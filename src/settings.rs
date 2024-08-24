use config::{Config, ConfigError, File};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

mod log;
use log::Log;

const DEAFULT_CONFIG_FILE: &str = "configs/default";
const CONFIG_FILE_PREFIX: &str = "configs/";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Settings {
    pub env: String,
    pub log: Log,
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        dotenv().ok();
        let env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            .add_source(File::with_name(DEAFULT_CONFIG_FILE))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            // .set_override("env", env)?
            .build()?;

        settings.try_deserialize()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn default_settings() {
        std::env::set_var("RUST_ENV", "default");

        let default_settings = Settings::load().unwrap();

        assert_eq!(default_settings.env, "development");
        assert_eq!(default_settings.host, "localhost");
        assert_eq!(default_settings.log, Log::Warn);
        assert_eq!(default_settings.port, 5000);
    }

    #[tokio::test]
    #[serial]
    async fn development_settings() {
        std::env::set_var("RUST_ENV", "development");

        let development_settings = Settings::load().unwrap();

        assert_eq!(development_settings.env, "development");
        assert_eq!(development_settings.host, "0.0.0.0");
        assert_eq!(development_settings.log, Log::Debug);
        assert_eq!(development_settings.port, 5000);
    }

    #[tokio::test]
    #[serial]
    async fn production_settings() {
        std::env::set_var("RUST_ENV", "production");

        let production_settings = Settings::load().unwrap();

        println!("production_settings: {:?}", production_settings);

        assert_eq!(production_settings.env, "production");
        assert_eq!(production_settings.host, "0.0.0.0");
        assert_eq!(production_settings.log, Log::Error);
        assert_eq!(production_settings.port, 5000);
    }

    #[tokio::test]
    #[serial]
    async fn testing_settings() {
        std::env::set_var("RUST_ENV", "testing");

        let testing_settings = Settings::load().unwrap();

        assert_eq!(testing_settings.env, "testing");
        assert_eq!(testing_settings.host, "0.0.0.0");
        assert_eq!(testing_settings.log, Log::Debug);
        assert_eq!(testing_settings.port, 5000);
    }
}
