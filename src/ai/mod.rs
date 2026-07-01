use anyhow::Result;

use crate::types::Config;

pub mod groq;
pub mod ollama;

#[async_trait::async_trait]
pub trait AIService: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String>;
}

pub fn make_service(provider: &str, config: &Config) -> Result<Box<dyn AIService>> {
    match provider {
        "groq" => Ok(Box::new(groq::GroqService::new(config)?)),
        "ollama" => Ok(Box::new(ollama::OllamaService::new(config))),
        "auto" => {
            if config.groq_api_key.is_some() {
                Ok(Box::new(groq::GroqService::new(config)?))
            } else {
                Ok(Box::new(ollama::OllamaService::new(config)))
            }
        }
        _ => Err(anyhow::anyhow!("Unknown provider: {}", provider)),
    }
}
