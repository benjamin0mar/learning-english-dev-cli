use anyhow::Result;

use crate::ai::make_service;
use crate::types::{Config, InputType, SuggestionResponse, SuggestResponse};

pub async fn suggest(
    text: &str,
    input_type: &InputType,
    provider: &str,
    config: &Config,
) -> Result<SuggestResponse> {
    if text.len() < 3 {
        return Err(anyhow::anyhow!("Text must be at least 3 characters long"));
    }
    if text.len() > 500 {
        return Err(anyhow::anyhow!("Text must be at most 500 characters long"));
    }

    let prompt = build_prompt(text, input_type);
    let service = make_service(provider, config)?;
    let raw = service.complete(&prompt).await?;

    let clean = extract_json(&raw)?;
    let parsed: SuggestionResponse = serde_json::from_str(&clean)?;

    Ok(SuggestResponse {
        suggestions: parsed,
        provider: provider.to_string(),
    })
}

fn build_prompt(text: &str, input_type: &InputType) -> String {
    let type_guide = match input_type {
        InputType::Commit => "a Git commit message",
        InputType::Prompt => "an AI prompt for code generation",
        InputType::Pr => "a Pull Request description",
    };

    format!(
        r#"You are a senior software engineer and English coach. Correct the following {type_guide} written by a non-native speaker.

Provide corrections at 5 CEFR levels (A1, A2, B1, B2, C1) in valid JSON format.

Rules:
- A1: Fix only critical grammar/spelling errors.
- A2: Fix grammar + improve clarity slightly.
- B1: Fix grammar + improve clarity + use standard dev terminology.
- B2: Professional tone + precise technical vocabulary.
- C1: Refined, idiomatic, senior-level English.

For each level, provide:
- "corrected": the corrected text
- "changes": array of {{"type": "missing"|"replaced"|"added", "word": "...", "original": "...", "replacement": "..."}}
- "explanation": brief explanation of key changes
- "vocabulary": array of {{"word": "...", "definition": "..."}} (can be null)

Return ONLY valid JSON with this exact structure:
{{
  "A1": {{ "corrected": "...", "changes": [], "explanation": "...", "vocabulary": null }},
  "A2": {{ "corrected": "...", "changes": [], "explanation": "...", "vocabulary": null }},
  "B1": {{ "corrected": "...", "changes": [], "explanation": "...", "vocabulary": [] }},
  "B2": {{ "corrected": "...", "changes": [], "explanation": "...", "vocabulary": [] }},
  "C1": {{ "corrected": "...", "changes": [], "explanation": "...", "vocabulary": [] }}
}}

Text to correct: {text}"#,
        type_guide = type_guide,
        text = text
    )
}

fn extract_json(raw: &str) -> Result<String> {
    let raw = raw.trim();

    if let Some(start) = raw.find('{') {
        if let Some(end) = raw.rfind('}') {
            Ok(raw[start..=end].to_string())
        } else {
            Err(anyhow::anyhow!("No valid JSON found in AI response"))
        }
    } else {
        Err(anyhow::anyhow!("No valid JSON found in AI response"))
    }
}


