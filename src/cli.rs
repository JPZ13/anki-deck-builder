use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "anki-deck-builder")]
#[command(about = "Build language learning Anki decks automatically", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Test connection to AnkiConnect
    Test,

    /// Create a new language learning deck
    Create {
        /// Target language to learn (e.g., "Croatian", "hr")
        #[arg(short, long)]
        target_language: Option<String>,

        /// Base language for translations (e.g., "Spanish", "es")
        #[arg(short, long)]
        base_language: Option<String>,

        /// Number of words per part of speech
        #[arg(short, long, default_value = "100")]
        words_per_pos: usize,

        /// Name of the deck to create
        #[arg(short, long)]
        deck_name: Option<String>,

        /// Dry run - preview without creating the deck
        #[arg(long, default_value = "false")]
        dry_run: bool,

        /// Create bidirectional cards (both target→base and base→target)
        #[arg(long, default_value = "true")]
        bidirectional: bool,
    },

    /// Configure AnkiConnect settings
    Config {
        /// AnkiConnect URL
        #[arg(long)]
        ankiconnect_url: Option<String>,

        /// Show current configuration
        #[arg(long, default_value = "false")]
        show: bool,
    },
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Test => handle_test().await,
        Commands::Create {
            target_language,
            base_language,
            words_per_pos,
            deck_name,
            dry_run,
            bidirectional,
        } => {
            handle_create(
                target_language,
                base_language,
                words_per_pos,
                deck_name,
                dry_run,
                bidirectional,
            )
            .await
        }
        Commands::Config {
            ankiconnect_url,
            show,
        } => handle_config(ankiconnect_url, show).await,
    }
}

async fn handle_test() -> Result<()> {
    use crate::{AnkiClient, Config};

    println!("🔍 Testing AnkiConnect connection...\n");

    let config = Config::new()?;
    println!("📍 AnkiConnect URL: {}", config.ankiconnect_url);

    let client = AnkiClient::new(config.ankiconnect_url.clone())?;

    match client.verify_connection().await {
        Ok(()) => {
            println!("✅ Successfully connected to AnkiConnect!\n");

            // Try to get decks
            match client.get_decks().await {
                Ok(decks) => {
                    println!("📚 Available decks ({}):", decks.len());
                    for deck in decks.iter().take(10) {
                        println!("  - {}", deck);
                    }
                    if decks.len() > 10 {
                        println!("  ... and {} more", decks.len() - 10);
                    }
                }
                Err(e) => {
                    println!("⚠️  Could not retrieve decks: {}", e);
                }
            }

            Ok(())
        }
        Err(e) => {
            println!("❌ Failed to connect to AnkiConnect");
            println!("\nError: {}\n", e);
            println!("💡 Troubleshooting:");
            println!("  1. Make sure Anki is running");
            println!("  2. Verify AnkiConnect add-on is installed (code: 2055492159)");
            println!(
                "  3. Check that AnkiConnect is accessible at {}",
                config.ankiconnect_url
            );
            println!("  4. Try restarting Anki if the add-on was just installed");

            Err(e.into())
        }
    }
}

async fn handle_create(
    target_language: Option<String>,
    base_language: Option<String>,
    words_per_pos: usize,
    deck_name: Option<String>,
    dry_run: bool,
    bidirectional: bool,
) -> Result<()> {
    use crate::language::{get_language, get_prioritized_languages};
    use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

    println!("🚀 Anki Deck Builder - Language Learning Deck Creator\n");

    // Get target language (either from arg or interactive prompt)
    let target_lang = match target_language {
        Some(lang_input) => {
            match get_language(&lang_input) {
                Some(lang) => {
                    println!("🎯 Target language: {} ({})", lang.name, lang.code);
                    lang
                }
                None => {
                    eprintln!("❌ Unsupported language: {}", lang_input);
                    eprintln!("Use 'Croatian', 'hr', or run without --target-language for a selection menu");
                    return Err(anyhow::anyhow!("Unsupported language: {}", lang_input));
                }
            }
        }
        None => {
            let languages = get_prioritized_languages();
            let lang_names: Vec<String> = languages
                .iter()
                .map(|l| format!("{} ({})", l.name, l.code))
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select target language to learn")
                .items(&lang_names)
                .default(0) // Croatian by default
                .interact()?;

            let selected = languages[selection].clone();
            println!("🎯 Target language: {} ({})", selected.name, selected.code);
            selected
        }
    };

    // Get base language (either from arg or interactive prompt)
    let base_lang = match base_language {
        Some(lang_input) => match get_language(&lang_input) {
            Some(lang) => {
                println!("🏠 Base language: {} ({})", lang.name, lang.code);
                lang
            }
            None => {
                eprintln!("❌ Unsupported language: {}", lang_input);
                return Err(anyhow::anyhow!("Unsupported language: {}", lang_input));
            }
        },
        None => {
            let languages = get_prioritized_languages();
            let lang_names: Vec<String> = languages
                .iter()
                .map(|l| format!("{} ({})", l.name, l.code))
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select base language (for translations)")
                .items(&lang_names)
                .default(1) // Spanish by default
                .interact()?;

            let selected = languages[selection].clone();
            println!("🏠 Base language: {} ({})", selected.name, selected.code);
            selected
        }
    };

    // Validate that target and base languages are different
    if target_lang.code == base_lang.code {
        eprintln!("❌ Target and base languages must be different!");
        return Err(anyhow::anyhow!("Target and base languages are the same"));
    }

    // Get deck name (either from arg or generate/prompt)
    let final_deck_name = match deck_name {
        Some(name) => {
            println!("📚 Deck name: {}", name);
            name
        }
        None => {
            let default_name = format!(
                "{} → {} (Top {} Words)",
                target_lang.name,
                base_lang.name,
                words_per_pos * 8
            ); // 8 parts of speech

            let use_default = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Use default deck name: '{}'?", default_name))
                .default(true)
                .interact()?;

            if use_default {
                println!("📚 Deck name: {}", default_name);
                default_name
            } else {
                let custom_name: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter custom deck name")
                    .interact_text()?;
                println!("📚 Deck name: {}", custom_name);
                custom_name
            }
        }
    };

    println!("\n📋 Configuration Summary:");
    println!(
        "  Target language: {} ({})",
        target_lang.name, target_lang.code
    );
    println!("  Base language: {} ({})", base_lang.name, base_lang.code);
    println!("  Words per part of speech: {}", words_per_pos);
    let estimated_cards = if bidirectional {
        words_per_pos * 8 * 2 // Double for bidirectional
    } else {
        words_per_pos * 8
    };
    println!(
        "  Total cards: ~{} (8 parts of speech{})",
        estimated_cards,
        if bidirectional { ", bidirectional" } else { "" }
    );
    println!("  Deck name: {}", final_deck_name);
    println!(
        "  Bidirectional: {}",
        if bidirectional { "yes" } else { "no" }
    );
    println!("  Dry run: {}", dry_run);

    if dry_run {
        println!("\n🔍 Dry run mode - no deck will be created");
        println!("✅ Configuration validated successfully!");
        return Ok(());
    }

    // Phase 4: Load frequency data
    println!("\n📊 Loading {} word frequency data...", target_lang.name);

    use crate::{language::load_frequency_data, Config};
    use indicatif::{ProgressBar, ProgressStyle};

    let config = Config::new()?;
    let cache_dir = config.cache_dir().clone();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Fetching {} frequency data...", target_lang.name));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let freq_data = load_frequency_data(&target_lang.code, &cache_dir).await?;
    spinner.finish_with_message(format!("✅ Loaded {} word data", target_lang.name));

    // Get top words for each POS
    use crate::language::PartOfSpeech;
    let all_words = freq_data.get_all_top_words(words_per_pos);

    println!("\n📝 Word selection:");
    println!(
        "  Nouns: {} words",
        freq_data
            .get_top_words(&PartOfSpeech::Noun, words_per_pos)
            .len()
    );
    println!(
        "  Verbs: {} words",
        freq_data
            .get_top_words(&PartOfSpeech::Verb, words_per_pos)
            .len()
    );
    println!(
        "  Adjectives: {} words",
        freq_data
            .get_top_words(&PartOfSpeech::Adjective, words_per_pos)
            .len()
    );
    println!("  Total: {} words selected", all_words.len());

    if !all_words.is_empty() {
        println!("\n🔤 Sample words:");
        for word in all_words.iter().take(5) {
            println!("  - {} ({:?})", word.text, word.pos);
        }
        if all_words.len() > 5 {
            println!("  ... and {} more", all_words.len() - 5);
        }
    }

    // Phase 5: Translate words
    println!(
        "\n🌐 Translating {} words from {} to {}...",
        all_words.len(),
        target_lang.name,
        base_lang.name
    );

    use crate::language::{MyMemoryClient, Translator};

    let translator = MyMemoryClient::new(Some(cache_dir.clone()))?;

    let progress = ProgressBar::new(all_words.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("=>-"),
    );
    progress.set_message("Translating");

    let mut translations: Vec<(String, String, PartOfSpeech)> = Vec::new();

    for word in &all_words {
        let translation = translator
            .translate(&word.text, &target_lang.code, &base_lang.code)
            .await?;
        translations.push((word.text.clone(), translation, word.pos.clone()));
        progress.inc(1);
    }

    progress.finish_with_message("✅ Translation complete");

    println!("\n📝 Sample translations:");
    for (croatian, spanish, pos) in translations.iter().take(10) {
        println!("  {} → {} ({:?})", croatian, spanish, pos);
    }
    if translations.len() > 10 {
        println!("  ... and {} more", translations.len() - 10);
    }

    // Phase 6-7: Create Anki deck and add cards
    println!("\n📚 Creating Anki deck: '{}'...", final_deck_name);

    use crate::AnkiClient;

    let anki_client = AnkiClient::new(config.ankiconnect_url.clone())?;

    // Verify AnkiConnect is running
    let verify_spinner = ProgressBar::new_spinner();
    verify_spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    verify_spinner.set_message("Checking AnkiConnect connection...");
    verify_spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    match anki_client.verify_connection().await {
        Ok(()) => {
            verify_spinner.finish_with_message("✅ Connected to AnkiConnect");
        }
        Err(e) => {
            verify_spinner.finish_with_message("❌ Failed to connect");
            eprintln!("\n❌ Could not connect to AnkiConnect: {}", e);
            eprintln!("\n💡 Make sure:");
            eprintln!("  1. Anki is running");
            eprintln!("  2. AnkiConnect add-on is installed");
            eprintln!("  3. Try running: make run ARGS=\"test\"");
            return Err(e.into());
        }
    }

    // Create deck
    match anki_client.create_deck(&final_deck_name).await {
        Ok(deck_id) => {
            println!("✅ Created deck with ID: {}", deck_id);
        }
        Err(e) => {
            // Deck might already exist, which is ok
            tracing::warn!("Deck creation returned: {}", e);
            println!("ℹ️  Using existing deck '{}'", final_deck_name);
        }
    }

    // Add cards
    let total_cards = if bidirectional {
        translations.len() * 2
    } else {
        translations.len()
    };

    println!(
        "\n📝 Adding {} cards to deck{}",
        total_cards,
        if bidirectional {
            " (bidirectional)"
        } else {
            ""
        }
    );

    use crate::Note;

    let card_progress = ProgressBar::new(total_cards as u64);
    card_progress.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("=>-"),
    );
    card_progress.set_message("Adding cards");

    let mut success_count = 0;
    let mut error_count = 0;

    for (croatian, spanish, _pos) in &translations {
        // Direction 1: Croatian (target) → Spanish (base)
        // You see Croatian and recall the Spanish meaning
        let front1 = croatian.clone();
        let back1 = spanish.clone();
        let note1 = Note::new(final_deck_name.clone(), front1, back1).with_tags(vec![
            "auto-generated".to_string(),
            "croatian-to-spanish".to_string(),
        ]);

        match anki_client.add_note(&note1).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                tracing::warn!("Failed to add note for '{}→{}': {}", croatian, spanish, e);
                error_count += 1;
            }
        }
        card_progress.inc(1);

        // Direction 2 (if bidirectional): Spanish (base) → Croatian (target)
        // You see Spanish and recall the Croatian word
        if bidirectional {
            let front2 = spanish.clone();
            let back2 = croatian.clone();
            let note2 = Note::new(final_deck_name.clone(), front2, back2).with_tags(vec![
                "auto-generated".to_string(),
                "spanish-to-croatian".to_string(),
            ]);

            match anki_client.add_note(&note2).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    tracing::warn!("Failed to add note for '{}→{}': {}", spanish, croatian, e);
                    error_count += 1;
                }
            }
            card_progress.inc(1);
        }
    }

    card_progress.finish_with_message("✅ Cards added");

    println!("\n🎉 Deck creation complete!");
    println!("  ✅ {} cards added successfully", success_count);
    if error_count > 0 {
        println!("  ⚠️  {} cards failed (may be duplicates)", error_count);
    }
    println!("  📚 Deck name: {}", final_deck_name);
    println!(
        "\n💡 Open Anki to start studying your {} words!",
        success_count
    );

    Ok(())
}

async fn handle_config(ankiconnect_url: Option<String>, show: bool) -> Result<()> {
    use crate::Config;

    if show {
        let config = Config::new()?;
        println!("Current configuration:");
        println!("  AnkiConnect URL: {}", config.ankiconnect_url);
        println!("  Translation Service: MyMemory (no API key required)");
        println!("  Cache directory: {}", config.cache_dir.display());
        return Ok(());
    }

    if let Some(url) = ankiconnect_url {
        println!("Setting AnkiConnect URL to: {}", url);
        println!("Note: Use environment variable ANKICONNECT_URL={}", url);
    }

    Ok(())
}
