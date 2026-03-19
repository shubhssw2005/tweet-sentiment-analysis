// Sentiment analysis utilities
// This module can be extended with additional sentiment analysis features

pub fn clean_text(text: &str) -> String {
    text.to_lowercase()
        .trim()
        .to_string()
}

pub fn is_valid_text(text: &str) -> bool {
    !text.trim().is_empty() && text.len() < 10000
}
