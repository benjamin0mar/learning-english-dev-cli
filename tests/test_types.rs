#[cfg(test)]
mod tests {
    use learning_english_dev_cli::types::{InputType, LevelKey, Config};

    #[test]
    fn test_input_type_as_str() {
        assert_eq!(InputType::Commit.as_str(), "commit");
        assert_eq!(InputType::Prompt.as_str(), "prompt");
        assert_eq!(InputType::Pr.as_str(), "pr");
    }

    #[test]
    fn test_level_key_all() {
        let levels = LevelKey::all();
        assert_eq!(levels.len(), 5);
        assert_eq!(levels[0], LevelKey::A1);
        assert_eq!(levels[4], LevelKey::C1);
    }

    #[test]
    fn test_level_key_as_str() {
        assert_eq!(LevelKey::A1.as_str(), "A1");
        assert_eq!(LevelKey::C1.as_str(), "C1");
    }

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.provider, "auto");
        assert_eq!(config.ollama_base_url, "http://localhost:11434");
        assert_eq!(config.history_limit, 50);
    }
}
