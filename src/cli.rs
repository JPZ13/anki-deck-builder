use clap::{Parser, Subcommand};
use anyhow::Result;

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
        } => {
            handle_create(
                target_language,
                base_language,
                words_per_pos,
                deck_name,
                dry_run,
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
    use crate::{Config, AnkiClient};
    
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
            println!("  3. Check that AnkiConnect is accessible at {}", config.ankiconnect_url);
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
) -> Result<()> {
    use crate::language::{get_language, get_prioritized_languages};
    use dialoguer::{Select, Input, Confirm, theme::ColorfulTheme};
    
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
            let lang_names: Vec<String> = languages.iter()
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
        Some(lang_input) => {
            match get_language(&lang_input) {
                Some(lang) => {
                    println!("🏠 Base language: {} ({})", lang.name, lang.code);
                    lang
                }
                None => {
                    eprintln!("❌ Unsupported language: {}", lang_input);
                    return Err(anyhow::anyhow!("Unsupported language: {}", lang_input));
                }
            }
        }
        None => {
            let languages = get_prioritized_languages();
            let lang_names: Vec<String> = languages.iter()
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
            let default_name = format!("{} → {} (Top {} Words)",
                                       target_lang.name,
                                       base_lang.name,
                                       words_per_pos * 8); // 8 parts of speech
            
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
    println!("  Target language: {} ({})", target_lang.name, target_lang.code);
    println!("  Base language: {} ({})", base_lang.name, base_lang.code);
    println!("  Words per part of speech: {}", words_per_pos);
    println!("  Total cards: ~{} (8 parts of speech)", words_per_pos * 8);
    println!("  Deck name: {}", final_deck_name);
    println!("  Dry run: {}", dry_run);
    
    if dry_run {
        println!("\n🔍 Dry run mode - no deck will be created");
        println!("✅ Configuration validated successfully!");
        return Ok(());
    }
    
    println!("\n⚠️  Deck creation not yet implemented (Phase 4-7)");
    println!("💡 This will be implemented in the next phases:");
    println!("  - Phase 4: Fetch {} word frequency data", target_lang.name);
    println!("  - Phase 5: Translate words from {} to {}", target_lang.name, base_lang.name);
    println!("  - Phase 6-7: Create deck and add cards to Anki");
    
    Ok(())
}

async fn handle_config(ankiconnect_url: Option<String>, show: bool) -> Result<()> {
    use crate::Config;

    if show {
        let config = Config::new()?;
        println!("Current configuration:");
        println!("  AnkiConnect URL: {}", config.ankiconnect_url);
        println!("  DeepL API Key: {}", 
            config.deepl_api_key.as_deref().unwrap_or("Not set"));
        println!("  LibreTranslate URL: {}", config.libretranslate_url);
        println!("  Cache directory: {}", config.cache_dir.display());
        return Ok(());
    }

    if let Some(url) = ankiconnect_url {
        println!("Setting AnkiConnect URL to: {}", url);
        println!("Note: Use environment variable ANKICONNECT_URL={}", url);
    }

    Ok(())
}
