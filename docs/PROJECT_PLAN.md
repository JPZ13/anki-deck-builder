# Anki Deck Builder - Project Plan

## Overview
A Rust CLI tool that automatically creates language learning decks on AnkiWeb with the most frequently used words in a target language, paired with translations in a base language.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Interface                        │
│  (User Input: credentials, target lang, base lang)           │
└─────────────────┬───────────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────────┐
│                    Main Orchestrator                         │
│  - Coordinate workflow                                       │
│  - Handle errors and retries                                 │
└──┬─────────┬──────────┬──────────┬──────────────────────────┘
   │         │          │          │
   │         │          │          │
┌──▼──────┐ │ ┌────────▼────┐ ┌───▼──────────┐
│ AnkiWeb │ │ │ Word Freq   │ │ Translation  │
│ Client  │ │ │ Service     │ │ Service      │
│         │ │ │             │ │              │
│ - Auth  │ │ │ - Get top   │ │ - Translate  │
│ - Deck  │ │ │   words by  │ │   words      │
│ - Cards │ │ │   POS       │ │              │
└─────────┘ │ └─────────────┘ └──────────────┘
            │
    ┌───────▼────────┐
    │ Local Storage  │
    │ - Cache        │
    │ - Config       │
    └────────────────┘
```

## Phase 1: Project Setup and Structure

### Goals
- Initialize Rust project with proper structure
- Set up dependencies
- Create project skeleton

### Tasks
1. **Initialize Cargo project**
   ```bash
   cargo new anki-deck-builder --bin
   cd anki-deck-builder
   ```

2. **Project Structure**
   ```
   anki-deck-builder/
   ├── Cargo.toml
   ├── src/
   │   ├── main.rs              # Entry point
   │   ├── cli.rs               # CLI interface
   │   ├── ankiweb/
   │   │   ├── mod.rs
   │   │   ├── auth.rs          # Authentication
   │   │   ├── client.rs        # API client
   │   │   └── models.rs        # Data models
   │   ├── language/
   │   │   ├── mod.rs
   │   │   ├── frequency.rs     # Word frequency data
   │   │   └── translator.rs    # Translation service
   │   ├── config.rs            # Configuration management
   │   └── error.rs             # Error types
   ├── tests/
   └── README.md
   ```

3. **Initial Dependencies** (Cargo.toml)
   ```toml
   [dependencies]
   # CLI
   clap = { version = "4.5", features = ["derive"] }
   dialoguer = "0.11"           # Interactive prompts
   indicatif = "0.17"           # Progress bars
   
   # HTTP/API
   reqwest = { version = "0.11", features = ["json", "cookies"] }
   
   # Async
   tokio = { version = "1", features = ["full"] }
   
   # Serialization
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   
   # Error handling
   anyhow = "1.0"
   thiserror = "1.0"
   
   # Configuration
   config = "0.14"
   directories = "5.0"          # Cross-platform config paths
   
   # Logging
   tracing = "0.1"
   tracing-subscriber = "0.3"
   ```

### Deliverables
- Initialized Cargo project
- Basic project structure
- Dependencies configured

---

## Phase 2: Research AnkiWeb API and Authentication

### Goals
- Understand AnkiWeb's API/sync protocol
- Implement authentication mechanism
- Handle session management

### Challenges & Solutions

**Challenge 1: AnkiWeb doesn't have an official public REST API**

**Solutions:**
1. **Option A: AnkiConnect** (Recommended)
   - Use AnkiConnect add-on running locally
   - Requires Anki desktop to be running
   - Well-documented JSON-RPC API
   - PRO: Stable, documented, widely used
   - CON: Requires local Anki installation

2. **Option B: AnkiWeb Sync Protocol**
   - Reverse-engineer the sync protocol
   - Direct communication with AnkiWeb
   - PRO: Direct to AnkiWeb, no local app needed
   - CON: Undocumented, may break with updates

3. **Option C: Genanki + Manual Upload**
   - Generate .apkg file locally using genanki-rs
   - User manually imports to AnkiWeb
   - PRO: Simple, reliable
   - CON: Not fully automated

**Recommended Approach: Start with AnkiConnect**
- Most practical for initial implementation
- Can add direct AnkiWeb sync later if needed

### Tasks

1. **Research AnkiConnect API**
   - Document endpoints needed:
     - `version`
     - `createDeck`
     - `addNote`
     - `modelNames`
     - `deckNames`

2. **Implement AnkiConnect Client**
   ```rust
   pub struct AnkiClient {
       base_url: String,
       client: reqwest::Client,
   }
   
   impl AnkiClient {
       pub async fn create_deck(&self, name: &str) -> Result<i64>;
       pub async fn add_note(&self, note: Note) -> Result<i64>;
       pub async fn get_decks(&self) -> Result<Vec<String>>;
   }
   ```

3. **Alternative: Implement .apkg Generator**
   - Research Anki package format
   - Use SQLite for deck database
   - Generate media files if needed

### Deliverables
- AnkiConnect client implementation OR
- .apkg generator with manual upload instructions
- Authentication/connection verification

---

## Phase 3: Implement CLI Interface

### Goals
- Create user-friendly command-line interface
- Collect required inputs
- Validate inputs

### Tasks

1. **Define CLI Structure**
   ```rust
   use clap::Parser;
   
   #[derive(Parser)]
   #[command(name = "anki-deck-builder")]
   #[command(about = "Build language learning Anki decks automatically")]
   struct Cli {
       #[command(subcommand)]
       command: Commands,
   }
   
   enum Commands {
       /// Create a new language deck
       Create {
           #[arg(short, long)]
           target_language: Option<String>,
           
           #[arg(short, long)]
           base_language: Option<String>,
           
           #[arg(short, long, default_value = "100")]
           words_per_pos: usize,
       },
       /// Configure AnkiConnect settings
       Config {
           #[arg(long)]
           ankiconnect_url: Option<String>,
       },
   }
   ```

2. **Interactive Prompts**
   ```rust
   use dialoguer::{Input, Select, theme::ColorfulTheme};
   
   fn get_target_language() -> Result<String> {
       let languages = vec!["Spanish", "French", "German", "Italian", "Japanese", "Korean", "Chinese"];
       let selection = Select::with_theme(&ColorfulTheme::default())
           .with_prompt("Select target language")
           .items(&languages)
           .interact()?;
       Ok(languages[selection].to_string())
   }
   ```

3. **Input Validation**
   - Validate language codes (ISO 639-1)
   - Verify AnkiConnect connection
   - Check if deck name already exists

### Deliverables
- Working CLI interface
- Interactive prompts for missing arguments
- Input validation

---

## Phase 4: Integrate Word Frequency Data Source

### Goals
- Find reliable source for most frequent words by POS
- Integrate data retrieval
- Cache data locally

### Data Sources

**Option 1: Wiktionary Frequency Lists**
- URL: `https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists`
- PRO: Free, multi-language
- CON: May need web scraping

**Option 2: OpenSubtitles Frequency Lists**
- URL: Various GitHub repositories have compiled lists
- PRO: Pre-compiled, comprehensive
- CON: Static data

**Option 3: Leipzig Corpora Collection**
- URL: `https://wortschatz.uni-leipzig.de/`
- PRO: Academic quality, many languages
- CON: May require API access

**Option 4: Hermit Dave's Frequency Lists**
- URL: `https://github.com/hermitdave/FrequencyWords`
- PRO: Pre-compiled JSON/CSV for 50+ languages
- CON: Static, may be outdated

**Recommended: Start with Hermit Dave's + Leipzig backup**

### Tasks

1. **Implement Frequency Data Loader**
   ```rust
   pub struct FrequencyData {
       language: String,
       words: HashMap<PartOfSpeech, Vec<Word>>,
   }
   
   #[derive(Debug, Serialize, Deserialize)]
   pub struct Word {
       text: String,
       pos: PartOfSpeech,
       frequency: usize,
       rank: usize,
   }
   
   #[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
   pub enum PartOfSpeech {
       Noun,
       Verb,
       Adjective,
       Adverb,
       Preposition,
       Pronoun,
       Conjunction,
       Interjection,
   }
   ```

2. **Implement Data Fetcher**
   ```rust
   pub async fn fetch_frequency_data(language: &str) -> Result<FrequencyData> {
       // Try multiple sources with fallback
       fetch_from_github(language)
           .or_else(|_| fetch_from_wiktionary(language))
           .or_else(|_| fetch_from_leipzig(language))
   }
   ```

3. **Implement Caching**
   - Store fetched data in `~/.local/share/anki-deck-builder/`
   - Cache for 30 days
   - Allow manual refresh

### Alternative Approach: POS Tagging

If frequency lists don't include POS tags:
1. Use a general frequency list
2. Use NLP library (like `nlprule`) to tag POS
3. Filter and categorize based on tags

### Deliverables
- Frequency data fetcher
- POS categorization
- Local caching system

---

## Phase 5: Implement Translation Service

### Goals
- Translate words from target to base language
- Handle batch translations efficiently
- Implement rate limiting and error handling

### Translation Service Options

**Option 1: LibreTranslate (Free/Self-hosted)**
- URL: `https://libretranslate.com/`
- PRO: Free tier available, open-source
- CON: Rate limits, quality varies

**Option 2: Google Translate (Unofficial API)**
- Libraries: `google-translator`
- PRO: High quality
- CON: Against ToS, may break

**Option 3: DeepL API (Paid)**
- URL: `https://www.deepl.com/pro-api`
- PRO: Excellent quality, free tier (500k chars/month)
- CON: Requires API key

**Option 4: Multiple Services with Fallback**
- Try DeepL → LibreTranslate → Google (unofficial)
- Best reliability

**Recommended: DeepL with LibreTranslate fallback**

### Tasks

1. **Implement Translation Client**
   ```rust
   #[async_trait]
   pub trait Translator {
       async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String>;
       async fn translate_batch(&self, texts: &[String], from: &str, to: &str) -> Result<Vec<String>>;
   }
   
   pub struct DeepLTranslator {
       api_key: String,
       client: reqwest::Client,
   }
   
   pub struct LibreTranslateClient {
       base_url: String,
       api_key: Option<String>,
       client: reqwest::Client,
   }
   ```

2. **Implement Rate Limiting**
   ```rust
   use std::time::Duration;
   use tokio::time::sleep;
   
   pub struct RateLimiter {
       requests_per_second: u32,
       last_request: std::time::Instant,
   }
   ```

3. **Implement Caching**
   - Cache translations to avoid re-translating
   - Key: `{source_lang}_{target_lang}_{word}`
   - Store in SQLite or JSON file

### Deliverables
- Translation service implementation
- Rate limiting
- Translation caching
- Fallback mechanism

---

## Phase 6: Implement AnkiWeb/AnkiConnect Client

### Goals
- Create robust client for Anki interaction
- Handle deck creation
- Add cards with proper formatting

### Tasks

1. **Implement AnkiConnect Protocol**
   ```rust
   #[derive(Serialize)]
   struct AnkiRequest<T> {
       action: String,
       version: u32,
       params: T,
   }
   
   #[derive(Deserialize)]
   struct AnkiResponse<T> {
       result: Option<T>,
       error: Option<String>,
   }
   ```

2. **Implement Deck Operations**
   ```rust
   impl AnkiClient {
       pub async fn create_deck(&self, name: &str) -> Result<i64> {
           let request = AnkiRequest {
               action: "createDeck".to_string(),
               version: 6,
               params: json!({ "deck": name }),
           };
           // Send request and handle response
       }
       
       pub async fn add_note(&self, deck: &str, front: &str, back: &str) -> Result<i64> {
           let note = json!({
               "deckName": deck,
               "modelName": "Basic",
               "fields": {
                   "Front": front,
                   "Back": back
               },
               "tags": ["auto-generated", "language-learning"]
           });
           // Add note
       }
   }
   ```

3. **Implement Card Formatting**
   ```rust
   struct CardFormatter;
   
   impl CardFormatter {
       fn format_card(word: &str, translation: &str, pos: &PartOfSpeech) -> (String, String) {
           let front = format!("{}", word);
           let back = format!(
               "{}\n<br><small><i>{:?}</i></small>",
               translation,
               pos
           );
           (front, back)
       }
   }
   ```

4. **Error Handling**
   - Retry failed requests
   - Handle connection errors
   - Validate deck exists before adding cards

### Deliverables
- Working AnkiConnect client
- Deck creation functionality
- Card addition with formatting
- Robust error handling

---

## Phase 7: Orchestrate Main Workflow

### Goals
- Coordinate all components
- Implement main business logic
- Add progress reporting

### Tasks

1. **Implement Main Workflow**
   ```rust
   pub async fn build_deck(
       target_lang: &str,
       base_lang: &str,
       words_per_pos: usize,
   ) -> Result<()> {
       // 1. Validate connection to AnkiConnect
       let anki_client = AnkiClient::new("http://localhost:8765")?;
       anki_client.verify_connection().await?;
       
       // 2. Fetch frequency data
       let freq_data = fetch_frequency_data(target_lang).await?;
       
       // 3. Select top words for each POS
       let words_to_translate = select_top_words(&freq_data, words_per_pos);
       
       // 4. Translate words
       let translator = get_translator()?;
       let translations = translate_words(&translator, &words_to_translate, target_lang, base_lang).await?;
       
       // 5. Create deck
       let deck_name = format!("{} to {} - Top Words", target_lang, base_lang);
       anki_client.create_deck(&deck_name).await?;
       
       // 6. Add cards
       add_cards_to_deck(&anki_client, &deck_name, &translations).await?;
       
       Ok(())
   }
   ```

2. **Add Progress Indicators**
   ```rust
   use indicatif::{ProgressBar, ProgressStyle};
   
   fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
       let pb = ProgressBar::new(total);
       pb.set_style(
           ProgressStyle::default_bar()
               .template("{msg} [{bar:40}] {pos}/{len} ({percent}%)")
               .unwrap()
       );
       pb.set_message(message.to_string());
       pb
   }
   ```

3. **Implement Batch Processing**
   - Process words in batches (e.g., 50 at a time)
   - Add delays between batches to avoid rate limits
   - Handle partial failures gracefully

4. **Add Logging**
   ```rust
   use tracing::{info, warn, error};
   
   info!("Starting deck creation for {} -> {}", target_lang, base_lang);
   warn!("Translation failed for word: {}, using fallback", word);
   error!("Failed to connect to AnkiConnect: {}", err);
   ```

### Deliverables
- Complete workflow orchestration
- Progress reporting
- Comprehensive logging
- Batch processing with rate limiting

---

## Phase 8: Testing and Refinement

### Goals
- Test all components
- Handle edge cases
- Improve user experience

### Tasks

1. **Unit Tests**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[tokio::test]
       async fn test_anki_client_connection() {
           // Test connection
       }
       
       #[test]
       fn test_card_formatting() {
           // Test formatting
       }
   }
   ```

2. **Integration Tests**
   - Test full workflow with small dataset
   - Test error recovery
   - Test caching mechanisms

3. **Edge Cases**
   - Unsupported language
   - AnkiConnect not running
   - Network failures
   - Duplicate deck names
   - Invalid translations

4. **User Experience Improvements**
   - Add `--dry-run` flag to preview without creating
   - Add `--verbose` flag for detailed logging
   - Add resume functionality for interrupted jobs
   - Add deck preview before creation

5. **Documentation**
   ```markdown
   # Usage
   
   ## Prerequisites
   1. Install Anki desktop
   2. Install AnkiConnect add-on
   3. Ensure Anki is running
   
   ## Basic Usage
   ```bash
   anki-deck-builder create \
     --target-language Spanish \
     --base-language English \
     --words-per-pos 100
   ```
   
   ## Configuration
   Optional DeepL API key for better translations:
   ```bash
   export DEEPL_API_KEY="your-api-key"
   ```
   ```

### Deliverables
- Comprehensive test suite
- Edge case handling
- User documentation
- Polished CLI experience

---

## Technical Challenges & Solutions

### Challenge 1: Rate Limiting
**Solution:** Implement exponential backoff and batch processing with delays

### Challenge 2: Translation Quality
**Solution:** Use multiple translation services with fallback, allow manual review

### Challenge 3: POS Tagging Accuracy
**Solution:** Use pre-tagged frequency lists where available, implement manual overrides

### Challenge 4: AnkiConnect Availability
**Solution:** Provide clear error messages, check connection before starting, offer .apkg export alternative

### Challenge 5: Language Support Coverage
**Solution:** Start with major languages, graceful degradation for unsupported languages

---

## Minimum Viable Product (MVP) Scope

For initial release, focus on:
1. ✅ **Target Language:** Croatian (hr)
2. ✅ **Base Language:** Spanish (es)
3. ✅ AnkiConnect only (no direct AnkiWeb)
4. ✅ Basic card format (Croatian word on front, Spanish translation on back)
5. ✅ Top 100 words per POS (total ~800 cards)
6. ✅ DeepL/LibreTranslate for Croatian→Spanish translations
7. ✅ Static frequency lists (no web scraping)

### Croatian Language Considerations
- Croatian uses Latin alphabet with diacritics (č, ć, dž, đ, lj, nj, š, ž)
- Ensure proper UTF-8 handling in Anki cards
- Croatian frequency lists available from:
  - Leipzig Corpora Collection (has Croatian corpus)
  - Hermit Dave's repository (verify Croatian availability)
  - Alternative: hrWaC corpus data
- DeepL supports Croatian (hr) ↔ Spanish (es) translation
- LibreTranslate also supports both languages as fallback

## Future Enhancements

- **Phase 2 Features:**
  - Direct AnkiWeb sync (no local Anki required)
  - Bidirectional cards (target→base and base→target)
  - Audio pronunciation
  - Example sentences
  - Images for nouns
  - Spaced repetition optimization
  - Custom word lists
  - Multiple deck templates

---

## Timeline Estimate

- **Phase 1:** 1-2 hours (Setup)
- **Phase 2:** 3-4 hours (AnkiConnect integration)
- **Phase 3:** 2-3 hours (CLI interface)
- **Phase 4:** 4-6 hours (Frequency data)
- **Phase 5:** 4-6 hours (Translation service)
- **Phase 6:** 3-4 hours (Anki client)
- **Phase 7:** 3-4 hours (Orchestration)
- **Phase 8:** 4-6 hours (Testing)

**Total: ~24-35 hours for MVP**

---

## Getting Started

Ready to begin? Let's start with Phase 1: Project Setup and Structure.
