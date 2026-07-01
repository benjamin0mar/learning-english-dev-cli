use clap::{Parser, Subcommand};
use std::str::FromStr;

use crate::types::{InputType, LevelKey};

#[derive(Parser, Debug)]
#[command(
    name = "learning-english-dev-cli",
    version,
    about = "AI-powered English writing assistant for software developers",
    long_about = "DevCoach helps software developers write better English for commits, PRs, and AI prompts.\n\nSupports 5 CEFR levels (A1-C1) with corrections, vocabulary definitions, and explanations.\n\nExamples:\n  learning-english-dev-cli \"fix the bug\"\n  echo \"commit message\" | learning-english-dev-cli\n  learning-english-dev-cli --file msg.txt -t prompt\n  learning-english-dev-cli history --last 5"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(help = "Text to analyze (omit for stdin)")]
    pub text: Option<String>,

    #[arg(short = 't', long = "input-type", default_value = "commit")]
    pub input_type: InputType,

    #[arg(short = 'l', long = "level", default_value = "all", value_parser = parse_level)]
    pub level: LevelFilter,

    #[arg(short = 'p', long = "provider", default_value = "auto")]
    pub provider: String,

    #[arg(short = 'f', long = "file")]
    pub file: Option<String>,

    #[arg(long = "json")]
    pub json: bool,

    #[arg(long = "no-color")]
    pub no_color: bool,
}

#[derive(Debug, Clone)]
pub enum LevelFilter {
    All,
    Single(LevelKey),
}

impl FromStr for LevelFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(LevelFilter::All),
            "a1" => Ok(LevelFilter::Single(LevelKey::A1)),
            "a2" => Ok(LevelFilter::Single(LevelKey::A2)),
            "b1" => Ok(LevelFilter::Single(LevelKey::B1)),
            "b2" => Ok(LevelFilter::Single(LevelKey::B2)),
            "c1" => Ok(LevelFilter::Single(LevelKey::C1)),
            _ => Err(format!("Invalid level '{}'. Use: all, A1, A2, B1, B2, or C1", s)),
        }
    }
}

fn parse_level(s: &str) -> Result<LevelFilter, String> {
    s.parse()
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Analyze text (default command)")]
    Analyze {
        #[arg(help = "Text to analyze")]
        text: Option<String>,

        #[arg(short = 't', long = "input-type", default_value = "commit")]
        input_type: InputType,

        #[arg(short = 'l', long = "level", default_value = "all", value_parser = parse_level)]
        level: LevelFilter,

        #[arg(short = 'p', long = "provider", default_value = "auto")]
        provider: String,

        #[arg(short = 'f', long = "file")]
        file: Option<String>,

        #[arg(long = "json")]
        json: bool,

        #[arg(long = "no-color")]
        no_color: bool,
    },
    #[command(about = "View analysis history")]
    History {
        #[arg(short = 'n', long = "last", default_value = "10")]
        last: usize,

        #[arg(long = "json")]
        json: bool,

        #[arg(long = "clear")]
        clear: bool,
    },
    #[command(about = "View or edit configuration")]
    Config {
        #[arg(help = "Config key to view/edit")]
        key: Option<String>,

        #[arg(help = "New value for the key")]
        value: Option<String>,
    },
    #[command(about = "Show usage examples")]
    Examples {
        #[arg(short = 't', long = "input-type")]
        input_type: Option<InputType>,
    },
}

pub fn get_text(cli: &Cli) -> Option<String> {
    if let Some(ref path) = cli.file {
        std::fs::read_to_string(path).ok()
    } else if let Some(ref text) = cli.text {
        Some(text.clone())
    } else {
        read_stdin()
    }
}

fn read_stdin() -> Option<String> {
    use std::io::Read;
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_ok() {
        let trimmed = input.trim().to_string();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    None
}
