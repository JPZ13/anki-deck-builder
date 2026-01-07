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
    // TODO: Implement in Phase 3
    println!("Create command placeholder");
    println!("Target: {:?}", target_language);
    println!("Base: {:?}", base_language);
    println!("Words per POS: {}", words_per_pos);
    println!("Deck name: {:?}", deck_name);
    println!("Dry run: {}", dry_run);
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
