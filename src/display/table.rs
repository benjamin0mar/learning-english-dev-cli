use crate::types::{LevelKey, LevelSuggestion, VocabWord};
use tabled::{Table, Tabled};

#[derive(Tabled)]
#[allow(dead_code)]
struct LevelRow {
    #[tabled(rename = "Level")]
    level: String,
    #[tabled(rename = "Text")]
    text: String,
    #[tabled(rename = "Explanation")]
    explanation: String,
}

#[derive(Tabled)]
struct VocabRow {
    #[tabled(rename = "Word")]
    word: String,
    #[tabled(rename = "Definition")]
    definition: String,
}

#[allow(dead_code)]
pub fn format_levels_table(
    levels: &[(LevelKey, LevelSuggestion)],
) -> String {
    let rows: Vec<LevelRow> = levels
        .iter()
        .map(|(key, sug)| LevelRow {
            level: key.as_str().to_string(),
            text: truncate(&sug.corrected, 40),
            explanation: truncate(&sug.explanation, 40),
        })
        .collect();

    if rows.is_empty() {
        return String::new();
    }

    Table::new(rows).to_string()
}

pub fn format_vocab_table(vocab: &[VocabWord]) -> String {
    let rows: Vec<VocabRow> = vocab
        .iter()
        .map(|v| VocabRow {
            word: v.word.clone(),
            definition: v.definition.clone(),
        })
        .collect();

    if rows.is_empty() {
        return String::new();
    }

    let table = Table::new(rows).to_string();
    format!("{}\n", table)
}

#[allow(dead_code)]
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
