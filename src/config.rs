use anyhow::Result;
use std::{env, fs, path::PathBuf};

use crate::types::Config;

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config")
}

fn config_path() -> PathBuf {
    data_dir().join("config.toml")
}

fn env_path() -> PathBuf {
    data_dir().join(".env")
}

fn create_default_config() -> Result<Config> {
    let dir = data_dir();
    fs::create_dir_all(&dir)?;

    let config = Config::default();
    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(config_path(), toml_str)?;

    Ok(config)
}

pub fn load_config() -> Result<Config> {
    let env_file = env_path();
    if env_file.exists() {
        dotenvy::from_path(&env_file).ok();
    }

    let path = config_path();
    let mut config: Config = if path.exists() {
        let content = fs::read_to_string(&path)?;
        toml::from_str(&content)?
    } else {
        create_default_config()?
    };

    if let Ok(key) = env::var("GROQ_API_KEY") {
        config.groq_api_key = Some(key);
    }
    if let Ok(url) = env::var("OLLAMA_BASE_URL") {
        config.ollama_base_url = url;
    }
    if let Ok(model) = env::var("OLLAMA_MODEL") {
        config.ollama_model = model;
    }
    if let Ok(provider) = env::var("DEVCOACH_PROVIDER") {
        config.provider = provider;
    }

    Ok(config)
}

pub fn get_config_value(key: &str, config: &Config) -> Option<String> {
    match key {
        "provider" => Some(config.provider.clone()),
        "groq_api_key" => config.groq_api_key.clone(),
        "ollama_base_url" => Some(config.ollama_base_url.clone()),
        "ollama_model" => Some(config.ollama_model.clone()),
        "default_level" => Some(config.default_level.as_str().to_string()),
        "history_limit" => Some(config.history_limit.to_string()),
        _ => None,
    }
}

fn load_config_raw() -> Result<Config> {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        Ok(toml::from_str(&content)?)
    } else {
        Ok(Config::default())
    }
}

pub fn set_config_value(key: &str, value: &str) -> Result<Config> {
    let mut config = load_config_raw()?;
    match key {
        "provider" => config.provider = value.to_string(),
        "groq_api_key" => config.groq_api_key = Some(value.to_string()),
        "ollama_base_url" => config.ollama_base_url = value.to_string(),
        "ollama_model" => config.ollama_model = value.to_string(),
        "default_level" => {
            config.default_level = match value.to_lowercase().as_str() {
                "a1" => crate::types::LevelKey::A1,
                "a2" => crate::types::LevelKey::A2,
                "b1" => crate::types::LevelKey::B1,
                "b2" => crate::types::LevelKey::B2,
                "c1" => crate::types::LevelKey::C1,
                _ => return Err(anyhow::anyhow!("Invalid level: {}. Use A1-A2-B1-B2-C1", value)),
            };
        }
        "history_limit" => {
            config.history_limit = value.parse::<usize>()?;
        }
        _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
    }
    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(config_path(), toml_str)?;
    Ok(config)
}
