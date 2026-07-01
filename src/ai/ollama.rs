use anyhow::Result;
use async_trait::async_trait;

use crate::ai::AIService;
use crate::types::Config;

pub struct OllamaService {
    base_url: String,
    model: String,
}

impl OllamaService {
    pub fn new(config: &Config) -> Self {
        OllamaService {
            base_url: config.ollama_base_url.clone(),
            model: config.ollama_model.clone(),
        }
    }
}

#[async_trait]
impl AIService for OllamaService {
    async fn complete(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?;

        let body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
            "temperature": 0.1
        });

        let resp = client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let error_body = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Ollama API error ({}): {}", status, error_body));
        }

        let data: serde_json::Value = resp.json().await?;
        let content = data["response"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Unexpected Ollama API response format"))?
            .to_string();

        Ok(content)
    }
}
