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
    use crate::config::Config;

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
