use crate::error::Result;
use crate::language::frequency::{FrequencyData, PartOfSpeech, Word};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Frequency word entry from data source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyEntry {
    pub word: String,
    pub pos: String,
    pub rank: usize,
}

/// Load frequency data for a given language
pub async fn load_frequency_data(
    language_code: &str,
    cache_dir: &std::path::Path,
) -> Result<FrequencyData> {
    // Try cache first
    if let Some(cached_data) = try_load_from_cache(language_code, cache_dir)? {
        tracing::info!("Loaded frequency data from cache for {}", language_code);
        return Ok(cached_data);
    }

    // Fetch from sources
    tracing::info!("Fetching frequency data for {}", language_code);
    let data = fetch_frequency_data(language_code).await?;

    // Save to cache
    save_to_cache(language_code, &data, cache_dir)?;

    Ok(data)
}

/// Try to load frequency data from cache
fn try_load_from_cache(
    language_code: &str,
    cache_dir: &std::path::Path,
) -> Result<Option<FrequencyData>> {
    let cache_file = get_cache_file_path(language_code, cache_dir);

    if !cache_file.exists() {
        return Ok(None);
    }

    // Check if cache is stale (older than 30 days)
    let metadata = std::fs::metadata(&cache_file)?;
    if let Ok(modified) = metadata.modified() {
        let age = modified.elapsed().unwrap_or_default();
        if age.as_secs() > 30 * 24 * 60 * 60 {
            tracing::warn!("Cache is stale, will refetch");
            return Ok(None);
        }
    }

    let content = std::fs::read_to_string(&cache_file)?;
    let data: FrequencyData = serde_json::from_str(&content)?;

    Ok(Some(data))
}

/// Save frequency data to cache
fn save_to_cache(
    language_code: &str,
    data: &FrequencyData,
    cache_dir: &std::path::Path,
) -> Result<()> {
    let cache_file = get_cache_file_path(language_code, cache_dir);

    // Create cache directory if it doesn't exist
    if let Some(parent) = cache_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(data)?;
    std::fs::write(&cache_file, json)?;

    tracing::info!("Saved frequency data to cache: {}", cache_file.display());

    Ok(())
}

/// Get cache file path for a language
fn get_cache_file_path(language_code: &str, cache_dir: &std::path::Path) -> PathBuf {
    cache_dir
        .join("frequency")
        .join(format!("{}_frequency.json", language_code))
}

/// Fetch frequency data from sources
async fn fetch_frequency_data(language_code: &str) -> Result<FrequencyData> {
    match language_code {
        "hr" => load_croatian_data().await,
        "es" => load_spanish_data().await,
        _ => {
            // For now, use embedded sample data for other languages
            load_sample_data(language_code).await
        }
    }
}

/// Load Croatian frequency data
async fn load_croatian_data() -> Result<FrequencyData> {
    use crate::language::frequency_fetcher::fetch_croatian_frequency;

    tracing::info!("Fetching Croatian frequency data from online sources...");

    // Fetch from Hermit Dave's FrequencyWords repository (50k words)
    fetch_croatian_frequency().await
}

/// Load Spanish frequency data
async fn load_spanish_data() -> Result<FrequencyData> {
    tracing::info!("Loading Spanish frequency data (embedded sample)");

    let mut data = FrequencyData::new("es".to_string());

    // Sample Spanish words (for testing)
    let nouns = vec![
        ("día", 1),
        ("tiempo", 2),
        ("año", 3),
        ("cosa", 4),
        ("hombre", 5),
        ("mundo", 6),
        ("vida", 7),
        ("mano", 8),
        ("parte", 9),
        ("niño", 10),
    ];

    for (word, rank) in nouns {
        data.add_word(Word {
            text: word.to_string(),
            pos: PartOfSpeech::Noun,
            frequency: 0,
            rank,
        });
    }

    Ok(data)
}

/// Load sample data for other languages
async fn load_sample_data(language_code: &str) -> Result<FrequencyData> {
    tracing::warn!(
        "No specific data for {}, returning empty dataset",
        language_code
    );
    Ok(FrequencyData::new(language_code.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    #[ignore] // Requires internet connection to fetch real data
    async fn test_load_croatian_data() {
        let data = load_croatian_data().await.unwrap();
        assert_eq!(data.language, "hr");

        // Should have words in multiple categories
        let all_words = data.get_all_top_words(10);
        assert!(!all_words.is_empty(), "Should have fetched Croatian words");
        assert!(all_words.len() >= 10, "Should have at least 10 words");
    }

    #[tokio::test]
    async fn test_caching() {
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        // First load (will create cache)
        let data1 = load_frequency_data("hr", &cache_dir).await.unwrap();

        // Second load (should use cache)
        let data2 = load_frequency_data("hr", &cache_dir).await.unwrap();

        assert_eq!(data1.language, data2.language);
        assert_eq!(
            data1.get_all_top_words(5).len(),
            data2.get_all_top_words(5).len()
        );

        // Verify cache file exists
        let cache_file = get_cache_file_path("hr", &cache_dir);
        assert!(cache_file.exists());
    }
}
