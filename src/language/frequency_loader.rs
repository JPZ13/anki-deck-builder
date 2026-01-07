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
    // For MVP, use embedded sample data
    // TODO: In production, fetch from Leipzig Corpora or hrWaC
    tracing::info!("Loading Croatian frequency data (embedded sample)");

    let mut data = FrequencyData::new("hr".to_string());

    // Sample Croatian nouns (most common)
    let nouns = vec![
        ("dan", 1),
        ("vrijeme", 2),
        ("dio", 3),
        ("način", 4),
        ("godina", 5),
        ("život", 6),
        ("država", 7),
        ("ljudi", 8),
        ("svijet", 9),
        ("posao", 10),
        ("mjesto", 11),
        ("kuća", 12),
        ("grad", 13),
        ("stvar", 14),
        ("dijete", 15),
        ("čovjek", 16),
        ("ime", 17),
        ("ruka", 18),
        ("glava", 19),
        ("oko", 20),
    ];

    for (word, rank) in nouns {
        data.add_word(Word {
            text: word.to_string(),
            pos: PartOfSpeech::Noun,
            frequency: 0,
            rank,
        });
    }

    // Sample Croatian verbs
    let verbs = vec![
        ("biti", 1),
        ("moći", 2),
        ("htjeti", 3),
        ("imati", 4),
        ("reći", 5),
        ("znati", 6),
        ("ići", 7),
        ("vidjeti", 8),
        ("dati", 9),
        ("doći", 10),
        ("uzeti", 11),
        ("raditi", 12),
        ("pričati", 13),
        ("misliti", 14),
        ("naći", 15),
    ];

    for (word, rank) in verbs {
        data.add_word(Word {
            text: word.to_string(),
            pos: PartOfSpeech::Verb,
            frequency: 0,
            rank,
        });
    }

    // Sample Croatian adjectives
    let adjectives = vec![
        ("dobar", 1),
        ("veliki", 2),
        ("mali", 3),
        ("novi", 4),
        ("stari", 5),
        ("prvi", 6),
        ("drugi", 7),
        ("vlastiti", 8),
        ("pravi", 9),
        ("važan", 10),
    ];

    for (word, rank) in adjectives {
        data.add_word(Word {
            text: word.to_string(),
            pos: PartOfSpeech::Adjective,
            frequency: 0,
            rank,
        });
    }

    Ok(data)
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
    async fn test_load_croatian_data() {
        let data = load_croatian_data().await.unwrap();
        assert_eq!(data.language, "hr");

        let nouns = data.get_top_words(&PartOfSpeech::Noun, 10);
        assert!(!nouns.is_empty());
        assert_eq!(nouns[0].text, "dan");
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
