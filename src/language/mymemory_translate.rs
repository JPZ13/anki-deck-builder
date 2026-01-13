use crate::error::{AnkiDeckBuilderError, Result};
use crate::language::translator::Translator;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MyMemoryClient {
    client: Client,
    cache_dir: Option<PathBuf>,
}

#[derive(Deserialize)]
struct MyMemoryResponse {
    #[serde(rename = "responseData")]
    response_data: ResponseData,
}

#[derive(Deserialize)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

impl MyMemoryClient {
    pub fn new(cache_dir: Option<PathBuf>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(AnkiDeckBuilderError::HttpError)?;

        Ok(Self { client, cache_dir })
    }

    /// Try to load translation from cache
    fn try_load_from_cache(&self, text: &str, from: &str, to: &str) -> Option<String> {
        let cache_dir = self.cache_dir.as_ref()?;
        let cache_file = cache_dir
            .join("translations")
            .join(format!("{}_{}.json", from, to));

        if !cache_file.exists() {
            return None;
        }

        // Load cache file
        let content = std::fs::read_to_string(&cache_file).ok()?;
        let cache: HashMap<String, String> = serde_json::from_str(&content).ok()?;

        cache.get(text).cloned()
    }

    /// Save translation to cache
    fn save_to_cache(&self, text: &str, translation: &str, from: &str, to: &str) -> Result<()> {
        let cache_dir = match &self.cache_dir {
            Some(dir) => dir,
            None => return Ok(()), // No caching if no cache dir
        };

        let translations_dir = cache_dir.join("translations");
        std::fs::create_dir_all(&translations_dir)?;

        let cache_file = translations_dir.join(format!("{}_{}.json", from, to));

        // Load existing cache or create new
        let mut cache: HashMap<String, String> = if cache_file.exists() {
            let content = std::fs::read_to_string(&cache_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        // Add new translation
        cache.insert(text.to_string(), translation.to_string());

        // Save back to file
        let json = serde_json::to_string_pretty(&cache)?;
        std::fs::write(&cache_file, json)?;

        Ok(())
    }
}

#[async_trait]
impl Translator for MyMemoryClient {
    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String> {
        // Try cache first
        if let Some(cached) = self.try_load_from_cache(text, from, to) {
            tracing::debug!("Cache hit for: {}", text);
            return Ok(cached);
        }

        tracing::debug!("Translating '{}' from {} to {}", text, from, to);

        // MyMemory API uses language pairs like "en|es" for English to Spanish
        let lang_pair = format!("{}|{}", from, to);
        
        // MyMemory API endpoint
        let url = "https://api.mymemory.translated.net/get";
        
        let response = self
            .client
            .get(url)
            .query(&[("q", text), ("langpair", &lang_pair)])
            .send()
            .await
            .map_err(|e| {
                AnkiDeckBuilderError::TranslationError(format!("HTTP request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AnkiDeckBuilderError::TranslationError(format!(
                "Translation API returned {}: {}",
                status, error_text
            )));
        }

        let mymemory_response: MyMemoryResponse = response.json().await.map_err(|e| {
            AnkiDeckBuilderError::TranslationError(format!("Failed to parse response: {}", e))
        })?;

        let translation = mymemory_response.response_data.translated_text;

        // Save to cache
        if let Err(e) = self.save_to_cache(text, &translation, from, to) {
            tracing::warn!("Failed to cache translation: {}", e);
        }

        Ok(translation)
    }

    async fn translate_batch(&self, texts: &[String], from: &str, to: &str) -> Result<Vec<String>> {
        let mut results = Vec::new();

        // MyMemory allows multiple concurrent requests
        // We'll process in smaller batches with minimal delay
        for (i, text) in texts.iter().enumerate() {
            if i > 0 && i % 10 == 0 {
                // Small delay every 10 requests to be respectful to the API
                tokio::time::sleep(Duration::from_millis(50)).await;
            }

            let translation = self.translate(text, from, to).await?;
            results.push(translation);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    #[ignore] // Requires internet connection
    async fn test_translate() {
        let client = MyMemoryClient::new(None).unwrap();

        let result = client.translate("hello", "en", "es").await;
        assert!(result.is_ok());

        let translation = result.unwrap();
        assert!(!translation.is_empty());
        println!("Translation: hello -> {}", translation);
    }

    #[tokio::test]
    async fn test_caching() {
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        let client = MyMemoryClient::new(Some(cache_dir.clone())).unwrap();

        // Save to cache
        client.save_to_cache("test", "prueba", "en", "es").unwrap();

        // Load from cache
        let cached = client.try_load_from_cache("test", "en", "es");
        assert_eq!(cached, Some("prueba".to_string()));
    }
}
