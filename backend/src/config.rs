use std::env::var;
use std::sync::OnceLock;

use crate::error::AppError;

/// This function returns a reference to a singleton instance of `Config`. It means that the same instance is returned on all calls.
/// The `Config` instance is created the first time this function is called, and the same instance is returned on all subsequent calls.
/// The `Config` instance is created by loading configuration values from environment variables.
/// If the environment variables cannot be loaded, the function panics.
pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();  // This is a static variable that is initialized only once.

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {err:?}")
        })
    })
}


/// This struct represents the configuration for the web service.
/// It currently contains only one field, `web_folder`, but more fields can be added as needed.
pub struct Config {
    pub web_folder: String,
}

impl Config {
    /// This function attempts to load the configuration from environment variables.
    /// It returns a `Result` that is `Ok` if the environment variables could be loaded, and `Err` otherwise.
    fn load_from_env() -> Result<Config, AppError> {
        Ok(Config {
            // Web
            web_folder: get_env("SERVICE_WEB_FOLDER")?,
            // // Database
            // database_url: get_env("SERVICE_DATABASE_URL")?,
            // // Logging
            // log_level: get_env("SERVICE_LOG_LEVEL")?,
        })
    }
}

/// This function attempts to get the value of an environment variable.
/// It returns a `Result` that is `Ok` if the environment variable exists, and `Err` otherwise.
fn get_env(key: &'static str) -> Result<String, AppError> {
    var(key).map_err(|_| AppError::EnvVarNotFound(key))
}