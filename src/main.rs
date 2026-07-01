mod ai;
mod cli;
mod config;
mod display;
mod history;
mod suggest;
mod types;

use clap::Parser;
use cli::{Cli, Commands, LevelFilter};
use colored::*;
use types::{HistoryEntry, InputType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = config::load_config()?;

    if cli.no_color {
        colored::control::set_override(false);
    }

    match cli.command {
        Some(Commands::History { last, json, clear }) => {
            handle_history(last, json, clear)?;
        }
        Some(Commands::Config { key, value }) => {
            handle_config(key, value, &cfg)?;
        }
        Some(Commands::Examples { input_type }) => {
            handle_examples(input_type);
        }
        Some(Commands::Analyze {
            text,
            input_type,
            level,
            provider,
            file,
            json,
            no_color,
        }) => {
            let effective_text = resolve_text(text, file)?;
            handle_analyze(effective_text, input_type, level, provider, json, no_color, &cfg).await?;
        }
        None => {
            let text = cli::get_text(&cli)
                .ok_or_else(|| anyhow::anyhow!("No input provided. Use learning-english-dev-cli --help for usage."))?;
            let text = text.trim().to_string();
            handle_analyze(
                text,
                cli.input_type,
                cli.level,
                cli.provider,
                cli.json,
                cli.no_color,
                &cfg,
            )
            .await?;
        }
    }

    Ok(())
}

async fn handle_analyze(
    text: String,
    input_type: InputType,
    level: LevelFilter,
    provider: String,
    json: bool,
    no_color: bool,
    config: &types::Config,
) -> anyhow::Result<()> {
    let effective_provider = if provider == "auto" {
        config.provider.clone()
    } else {
        provider
    };

    let response = suggest::suggest(&text, &input_type, &effective_provider, config).await?;

    let entry = HistoryEntry {
        id: uuid::Uuid::new_v4().to_string(),
        text: text.clone(),
        input_type: input_type.clone(),
        suggestions: response.suggestions.clone(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    if let Err(e) = history::save_entry(&entry, config) {
        eprintln!("{} Warning: failed to save history: {}", "[!]".yellow(), e);
    }

    let output = display::format_output(&response, &input_type, &level, json, no_color);
    println!("{}", output);

    Ok(())
}

fn handle_history(last: usize, json: bool, clear: bool) -> anyhow::Result<()> {
    if clear {
        history::clear_history()?;
        println!("{} History cleared.", "✓".green());
        return Ok(());
    }

    let entries = history::load_entries(last)?;

    if entries.is_empty() {
        println!("{} No history entries found.", "ℹ".cyan());
        return Ok(());
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&entries)?);
    } else {
        for entry in &entries {
            print!("{}", display::format_history_entry(entry, false));
        }
    }

    Ok(())
}

fn handle_config(key: Option<String>, value: Option<String>, current_config: &types::Config) -> anyhow::Result<()> {
    match (key, value) {
        (None, _) => {
            print!("{}", display::format_config(current_config));
        }
        (Some(key), None) => {
            match config::get_config_value(&key, current_config) {
                Some(val) => println!("{} = {}", key, val),
                None => eprintln!("{} Unknown config key: {}", "Error:".red().bold(), key),
            }
        }
        (Some(key), Some(value)) => {
            let updated = config::set_config_value(&key, &value)?;
            println!("{} Set {} = {}", "✓".green(), key.green(), value.yellow());
            println!("\nUpdated config:");
            print!("{}", display::format_config(&updated));
        }
    }

    Ok(())
}

fn handle_examples(input_type: Option<InputType>) {
    let examples = match input_type {
        Some(InputType::Commit) => vec![
            "fix: resolve user authentication issue",
            "refactor: extract validation logic to separate module",
            "docs: update API endpoint documentation",
            "feat: add rate limiting middleware",
        ],
        Some(InputType::Prompt) => vec![
            "write a function to sort a list of numbers",
            "explain how to implement a binary search tree in python",
            "create a dockerfile for a node.js app with postgres",
            "generate a sql query to find duplicate emails",
        ],
        Some(InputType::Pr) => vec![
            "add user authentication with jwt tokens",
            "fix the bug where the login page crashes on empty input",
            "refactor the database layer to use connection pooling",
            "implement dark mode support across all pages",
        ],
        None => vec![
            "learning-english-dev-cli \"fix the bug\" -t commit",
            "echo \"write a function\" | learning-english-dev-cli -t prompt",
            "learning-english-dev-cli --file pr.txt -t pr",
            "learning-english-dev-cli \"refactor auth\" -l B1 --json",
            "learning-english-dev-cli history --last 5",
            "learning-english-dev-cli config provider groq",
        ],
    };

    let title = match input_type {
        Some(ref t) => format!("Examples for {}:", t.as_str()),
        None => "Usage examples:".to_string(),
    };

    println!("{}\n", title.cyan().bold());
    for example in &examples {
        println!("  {} {}", "$".bright_black(), example);
    }
}

fn resolve_text(text: Option<String>, file: Option<String>) -> anyhow::Result<String> {
    if let Some(path) = file {
        let content = std::fs::read_to_string(&path)?;
        return Ok(content.trim().to_string());
    }

    if let Some(t) = text {
        return Ok(t);
    }

    Err(anyhow::anyhow!("No text provided. Provide text as an argument, use --file, or pipe via stdin."))
}
