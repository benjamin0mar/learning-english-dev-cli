use anyhow::Result;
use async_trait::async_trait;

use crate::ai::AIService;
use crate::types::Config;

pub struct GroqService {
    api_key: String,
}

impl GroqService {
    pub fn new(config: &Config) -> Result<Self> {
        let api_key = config
            .groq_api_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("GROQ_API_KEY not set"))?;

        Ok(GroqService { api_key })
    }
}

#[async_trait]
impl AIService for GroqService {
    async fn complete(&self, prompt: &str) -> Result<String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let body = serde_json::json!({
            "model": "llama-3.3-70b-versatile",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "max_tokens": 1500
        });

        let resp = client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let error_body = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Groq API error ({}): {}", status, error_body));
        }

        let data: serde_json::Value = resp.json().await?;
        let content = data["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Unexpected Groq API response format"))?
            .to_string();

        Ok(content)
    }
}
