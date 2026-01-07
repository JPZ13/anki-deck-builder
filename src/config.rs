use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ankiconnect_url: String,
    pub deepl_api_key: Option<String>,
    pub libretranslate_url: String,
    pub cache_dir: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "anki-deck-builder", "anki-deck-builder")
            .ok_or_else(|| anyhow::anyhow!("Could not determine project directories"))?;

        let cache_dir = project_dirs.data_dir().to_path_buf();
        
        // Create cache directory if it doesn't exist
        std::fs::create_dir_all(&cache_dir)?;

        Ok(Config {
            ankiconnect_url: std::env::var("ANKICONNECT_URL")
                .unwrap_or_else(|_| "http://localhost:8765".to_string()),
            deepl_api_key: std::env::var("DEEPL_API_KEY").ok(),
            libretranslate_url: std::env::var("LIBRETRANSLATE_URL")
                .unwrap_or_else(|_| "https://libretranslate.com".to_string()),
            cache_dir,
        })
    }

    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    pub fn frequency_cache_dir(&self) -> PathBuf {
        self.cache_dir.join("frequency")
    }

    pub fn translation_cache_dir(&self) -> PathBuf {
        self.cache_dir.join("translations")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Failed to create default config")
    }
}
