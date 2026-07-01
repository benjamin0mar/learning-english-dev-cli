#[cfg(test)]
mod tests {
    use learning_english_dev_cli::cli::LevelFilter;
    use learning_english_dev_cli::types::LevelKey;
    use std::str::FromStr;

    #[test]
    fn test_level_filter_from_str() {
        assert!(matches!(LevelFilter::from_str("all"), Ok(LevelFilter::All)));
        assert!(matches!(LevelFilter::from_str("A1"), Ok(LevelFilter::Single(LevelKey::A1))));
        assert!(matches!(LevelFilter::from_str("C1"), Ok(LevelFilter::Single(LevelKey::C1))));
        assert!(LevelFilter::from_str("invalid").is_err());
    }

    #[test]
    fn test_level_filter_case_insensitive() {
        assert!(matches!(LevelFilter::from_str("b2"), Ok(LevelFilter::Single(LevelKey::B2))));
    }
}
