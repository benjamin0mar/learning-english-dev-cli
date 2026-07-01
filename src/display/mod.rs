pub mod diff;
pub mod table;

use colored::*;

use crate::cli::LevelFilter;
use crate::types::{LevelKey, SuggestResponse};

pub fn format_output(response: &SuggestResponse, level_filter: &LevelFilter, json_mode: bool, no_color: bool) -> String {
    if no_color {
        colored::control::set_override(false);
    }

    if json_mode {
        return format_json_output(response);
    }

    format_ansi_output(response, level_filter)
}

fn format_json_output(response: &SuggestResponse) -> String {
    serde_json::to_string_pretty(response).unwrap_or_default()
}

fn format_ansi_output(response: &SuggestResponse, level_filter: &LevelFilter) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "{}\n",
        "╭──────────────────────────────────────────────╮".bright_black()
    ));
    output.push_str(&format!(
        "{} {} {}\n",
        "│".bright_black(),
        "DevCoach — English for devs".cyan().bold(),
        "│".bright_black(),
    ));
    output.push_str(&format!(
        "{}  Provider: {}  |  Input: commit  │{}\n",
        "│".bright_black(),
        response.provider.green(),
        "│".bright_black(),
    ));
    output.push_str(&format!(
        "{}\n",
        "╰──────────────────────────────────────────────╯".bright_black()
    ));

    let all_levels = LevelKey::all();
    let levels: Vec<&LevelKey> = match level_filter {
        LevelFilter::All => all_levels.iter().collect(),
        LevelFilter::Single(ref level) => vec![level],
    };

    for level in &levels {
        if let Some(suggestion) = response.suggestions.get(level) {
            output.push_str(&format!(
                "\n {} {} {}\n",
                "──".bright_black(),
                format!(" Level {} ", level.as_str()).white().bold(),
                "─".repeat(40).bright_black(),
            ));

            output.push_str(&format!("\n{} {}\n", "Corrected Text:".green().bold(), suggestion.corrected.cyan()));

            if !suggestion.changes.is_empty() {
                output.push_str(&format!("\n{}\n", " Changes:".yellow().bold()));
                for change in &suggestion.changes {
                    let change_str = diff::format_change(change);
                    output.push_str(&format!("  {}\n", change_str));
                }
            }

            output.push_str(&format!("\n{}\n", "Explanation:".white().bold()));
            output.push_str(&format!("  {}\n", suggestion.explanation.white().italic()));

            if let Some(ref vocab) = suggestion.vocabulary {
                if !vocab.is_empty() {
                    output.push_str(&format!("\n{}\n", "Vocabulary:".purple().bold()));
                    output.push_str(&table::format_vocab_table(vocab));
                }
            }
        }
    }

    if matches!(level_filter, LevelFilter::All) {
        output.push_str(&format!("\n{}\n", "─".repeat(50).bright_black()));
        let level_strs: Vec<String> = LevelKey::all()
            .iter()
            .map(|l| {
                if response.suggestions.contains_key(l) {
                    format!("[{}]", l.as_str()).cyan().to_string()
                } else {
                    format!("[{}]", l.as_str()).bright_black().to_string()
                }
            })
            .collect();
        output.push_str(&format!("  {}\n", level_strs.join("  ")));
        output.push_str(&format!("{}\n", "─".repeat(50).bright_black()));
    }

    colored::control::set_override(true);

    output
}

pub fn format_history_entry(entry: &crate::types::HistoryEntry, json_mode: bool) -> String {
    if json_mode {
        return serde_json::to_string_pretty(entry).unwrap_or_default();
    }

    let time = chrono::DateTime::from_timestamp(entry.timestamp, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "unknown".to_string());

    format!(
        "[{}] {} ({}) — {} levels\n  {}\n",
        time,
        entry.input_type.as_str().cyan(),
        &entry.id[..8],
        entry.suggestions.len(),
        entry.text,
    )
}

pub fn format_config(config: &crate::types::Config) -> String {
    format!(
        r#"{} = "{}"
{} = "{}"
{} = "{}"
{} = "{}"
{} = "{}"
{} = {}
"#,
        "provider".green(),
        config.provider,
        "groq_api_key".green(),
        config.groq_api_key.as_deref().unwrap_or("(not set)"),
        "ollama_base_url".green(),
        config.ollama_base_url,
        "ollama_model".green(),
        config.ollama_model,
        "default_level".green(),
        config.default_level.as_str(),
        "history_limit".green(),
        config.history_limit,
    )
}
