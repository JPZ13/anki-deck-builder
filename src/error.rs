use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnkiDeckBuilderError {
    #[error("AnkiConnect is not running or unreachable at {url}")]
    AnkiConnectNotRunning { url: String },

    #[error("AnkiConnect returned an error: {0}")]
    AnkiConnectError(String),

    #[error("Failed to translate text: {0}")]
    TranslationError(String),

    #[error("Frequency data not available for language: {0}")]
    FrequencyDataNotFound(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Deck already exists: {0}")]
    DeckAlreadyExists(String),

    #[error("Invalid configuration: {0}")]
    ConfigurationError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AnkiDeckBuilderError>;
