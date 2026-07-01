use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::types::{Config, HistoryEntry};

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config")
}

fn history_path() -> PathBuf {
    data_dir().join("history.json")
}

pub fn save_entry(entry: &HistoryEntry, config: &Config) -> Result<()> {
    let dir = data_dir();
    fs::create_dir_all(&dir)?;

    let path = history_path();
    let mut entries: Vec<HistoryEntry> = if path.exists() {
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        vec![]
    };

    entries.insert(0, entry.clone());
    entries.truncate(config.history_limit);

    let json = serde_json::to_string_pretty(&entries)?;
    fs::write(&path, json)?;

    Ok(())
}

pub fn load_entries(last_n: usize) -> Result<Vec<HistoryEntry>> {
    let path = history_path();

    if !path.exists() {
        return Ok(vec![]);
    }

    let content = fs::read_to_string(&path)?;
    let entries: Vec<HistoryEntry> = serde_json::from_str(&content)?;
    let count = last_n.min(entries.len());

    Ok(entries.into_iter().take(count).collect())
}

pub fn clear_history() -> Result<()> {
    let path = history_path();

    if path.exists() {
        fs::write(&path, "[]")?;
    }

    Ok(())
}
