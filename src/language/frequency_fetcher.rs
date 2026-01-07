use crate::error::{AnkiDeckBuilderError, Result};
use crate::language::frequency::{FrequencyData, PartOfSpeech, Word};
use reqwest::Client;
use std::time::Duration;

/// Fetch Croatian frequency data from external sources
pub async fn fetch_croatian_frequency() -> Result<FrequencyData> {
    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(AnkiDeckBuilderError::HttpError)?;

    // Try Hermit Dave's FrequencyWords repository first
    tracing::info!("Fetching Croatian frequency data from GitHub...");

    let url = "https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018/hr/hr_50k.txt";

    let response = client.get(url).send().await.map_err(|e| {
        AnkiDeckBuilderError::FrequencyDataNotFound(format!(
            "Failed to fetch Croatian frequency data: {}",
            e
        ))
    })?;

    if !response.status().is_success() {
        return Err(AnkiDeckBuilderError::FrequencyDataNotFound(format!(
            "HTTP {}: Could not download Croatian frequency list",
            response.status()
        )));
    }

    let text = response.text().await.map_err(|e| {
        AnkiDeckBuilderError::FrequencyDataNotFound(format!("Failed to read frequency data: {}", e))
    })?;

    parse_frequency_file(&text, "hr")
}

/// Parse frequency file in format: "word frequency"
fn parse_frequency_file(content: &str, language_code: &str) -> Result<FrequencyData> {
    let mut data = FrequencyData::new(language_code.to_string());

    for (rank, line) in content.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            continue;
        }

        let word_text = parts[0];

        // Skip very short words (likely articles/prepositions)
        if word_text.len() < 2 {
            continue;
        }

        // Categorize by POS using simple heuristics for Croatian
        // TODO: Use actual POS tagging in future versions
        let pos = guess_croatian_pos(word_text);

        data.add_word(Word {
            text: word_text.to_string(),
            pos,
            frequency: 0,
            rank: rank + 1,
        });
    }

    tracing::info!(
        "Parsed {} total words from frequency list",
        data.words.values().map(|v| v.len()).sum::<usize>()
    );

    Ok(data)
}

/// Simple POS guessing for Croatian based on word endings
/// This is a heuristic approach - not perfect but functional for MVP
fn guess_croatian_pos(word: &str) -> PartOfSpeech {
    let word_lower = word.to_lowercase();

    // Common Croatian verb endings (infinitive and conjugations)
    if word_lower.ends_with("ti")
        || word_lower.ends_with("ći")
        || word_lower.ends_with("am")
        || word_lower.ends_with("aš")
        || word_lower.ends_with("im")
        || word_lower.ends_with("iš")
    {
        return PartOfSpeech::Verb;
    }

    // Common adjective endings
    if word_lower.ends_with("ski")
        || word_lower.ends_with("ski")
        || word_lower.ends_with("ški")
        || word_lower.ends_with("čki")
    {
        return PartOfSpeech::Adjective;
    }

    // Common adverb markers
    if word_lower.ends_with("no")
        || word_lower.ends_with("ko")
        || word_lower.ends_with("je") && word_lower.len() > 4
    {
        return PartOfSpeech::Adverb;
    }

    // Common prepositions (small set)
    if matches!(
        word_lower.as_str(),
        "u" | "na" | "za" | "s" | "sa" | "iz" | "do" | "od" | "po" | "prema" | "kroz"
    ) {
        return PartOfSpeech::Preposition;
    }

    // Common pronouns
    if matches!(
        word_lower.as_str(),
        "ja" | "ti" | "on" | "ona" | "ono" | "mi" | "vi" | "oni" | "me" | "te" | "se"
    ) {
        return PartOfSpeech::Pronoun;
    }

    // Common conjunctions
    if matches!(
        word_lower.as_str(),
        "i" | "ali" | "ili" | "da" | "ako" | "jer" | "kad" | "dok"
    ) {
        return PartOfSpeech::Conjunction;
    }

    // Default to Noun (most common category)
    PartOfSpeech::Noun
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_croatian_pos() {
        assert_eq!(guess_croatian_pos("biti"), PartOfSpeech::Verb);
        assert_eq!(guess_croatian_pos("doći"), PartOfSpeech::Verb);
        assert_eq!(guess_croatian_pos("hrvatski"), PartOfSpeech::Adjective);
        assert_eq!(guess_croatian_pos("u"), PartOfSpeech::Preposition);
        assert_eq!(guess_croatian_pos("ja"), PartOfSpeech::Pronoun);
        assert_eq!(guess_croatian_pos("i"), PartOfSpeech::Conjunction);
        assert_eq!(guess_croatian_pos("dan"), PartOfSpeech::Noun);
        assert_eq!(guess_croatian_pos("kuća"), PartOfSpeech::Noun);
    }

    #[test]
    fn test_parse_frequency_file() {
        let sample = "biti 12345\ndan 11000\nhrvatski 9000\nu 8000";
        let data = parse_frequency_file(sample, "hr").unwrap();

        assert_eq!(data.language, "hr");
        assert!(data.words.get(&PartOfSpeech::Verb).is_some());
        assert!(data.words.get(&PartOfSpeech::Noun).is_some());
    }

    #[tokio::test]
    #[ignore] // Requires internet connection
    async fn test_fetch_croatian_frequency() {
        let result = fetch_croatian_frequency().await;
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.language, "hr");
        assert!(!data.words.is_empty());
    }
}
