use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    #[clap(name = "commit")]
    Commit,
    #[clap(name = "prompt")]
    Prompt,
    #[clap(name = "pr")]
    Pr,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Commit => "commit",
            InputType::Prompt => "prompt",
            InputType::Pr => "pr",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LevelKey {
    A1,
    A2,
    B1,
    B2,
    C1,
}

impl LevelKey {
    pub fn all() -> [LevelKey; 5] {
        [LevelKey::A1, LevelKey::A2, LevelKey::B1, LevelKey::B2, LevelKey::C1]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LevelKey::A1 => "A1",
            LevelKey::A2 => "A2",
            LevelKey::B1 => "B1",
            LevelKey::B2 => "B2",
            LevelKey::C1 => "C1",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Missing,
    Replaced,
    Added,
    Capitalized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordChange {
    #[serde(rename = "type")]
    pub change_type: ChangeType,
    pub word: Option<String>,
    pub original: Option<String>,
    pub replacement: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabWord {
    pub word: String,
    pub definition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelSuggestion {
    pub corrected: String,
    pub changes: Vec<WordChange>,
    pub explanation: String,
    pub vocabulary: Option<Vec<VocabWord>>,
}

pub type SuggestionResponse = HashMap<LevelKey, LevelSuggestion>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SuggestRequest {
    pub text: String,
    pub input_type: InputType,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestResponse {
    pub suggestions: SuggestionResponse,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub text: String,
    pub input_type: InputType,
    pub suggestions: SuggestionResponse,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: String,
    pub groq_api_key: Option<String>,
    pub ollama_base_url: String,
    pub ollama_model: String,
    pub default_level: LevelKey,
    pub history_limit: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: "auto".to_string(),
            groq_api_key: None,
            ollama_base_url: "http://localhost:11434".to_string(),
            ollama_model: "gpt-oss:120b-cloud".to_string(),
            default_level: LevelKey::B1,
            history_limit: 50,
        }
    }
}
