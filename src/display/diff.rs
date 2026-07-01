use colored::*;

use crate::types::{ChangeType, WordChange};

pub fn format_change(change: &WordChange) -> String {
    match change.change_type {
        ChangeType::Missing | ChangeType::Added => {
            let word = change.word.as_deref().unwrap_or("?");
            format!("{}  {} {}", "+".green().bold(), word.green().bold(), "(added by AI)".bright_black())
        }
        ChangeType::Replaced => {
            let original = change.original.as_deref().unwrap_or("?");
            let replacement = change.replacement.as_deref().unwrap_or("?");
            format!(
                "{}  {}  →  {}",
                "~".yellow().bold(),
                original.red().bold(),
                replacement.yellow().bold()
            )
        }
    }
}

#[allow(dead_code)]
pub fn highlight_diff(original: &str, changes: &[WordChange]) -> String {
    let mut result = original.to_string();

    for change in changes {
        match change.change_type {
            ChangeType::Replaced => {
                if let Some(ref original_word) = change.original {
                    if let Some(ref replacement) = change.replacement {
                        result = result.replace(
                            original_word,
                            &format!("{}", replacement.on_yellow().black()),
                        );
                    }
                }
            }
            ChangeType::Missing | ChangeType::Added => {
                if let Some(ref word) = change.word {
                    result = format!("{} {}", result, format!("{}", word.on_red().white().bold()));
                }
            }
        }
    }

    result
}
