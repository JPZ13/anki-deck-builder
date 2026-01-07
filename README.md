# Anki Deck Builder

A Rust CLI tool that automatically creates language learning decks on Anki with the most frequently used words in a target language, paired with translations in a base language.

## Features

- 🎯 Automatically generates Anki decks for language learning
- 📊 Uses frequency data to select the most common words
- 🔤 Organizes words by part of speech (nouns, verbs, adjectives, etc.)
- 🌐 Supports multiple translation services (DeepL, LibreTranslate)
- 💾 Caches data locally to minimize API calls
- 📦 Works with AnkiConnect for seamless integration

## MVP Configuration

- **Target Language:** Croatian (hr)
- **Base Language:** Spanish (es)
- **Default Cards:** ~800 (100 words per part of speech)

## Prerequisites

### NixOS Setup (Recommended)

This project includes a [`shell.nix`](shell.nix:1) file for NixOS users with all dependencies:

```bash
# Enter Nix development shell
nix-shell

# Or use direnv for automatic loading (optional)
echo "use nix" > .envrc
direnv allow
```

The Nix shell provides:
- Rust toolchain (rustc, cargo, clippy, rust-analyzer)
- Anki desktop application
- All build dependencies

### Manual Setup (Non-NixOS)

1. **Rust** - Install from [rustup.rs](https://rustup.rs)
2. **Anki Desktop** - Download from [apps.ankiweb.net](https://apps.ankiweb.net)

### AnkiConnect Add-on (All Systems)

Install AnkiConnect in Anki:
1. Open Anki
2. Tools → Add-ons → Get Add-ons
3. Code: `2055492159`
4. Restart Anki

**Verify the setup:**
```bash
# Test AnkiConnect connection
cargo run -- test
# or
make run ARGS="test"
```

See [📖 AnkiConnect Setup Guide](docs/ANKICONNECT_SETUP.md) for detailed instructions and troubleshooting.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/anki-deck-builder.git
cd anki-deck-builder

# NixOS: Enter development shell
nix-shell

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

## Configuration

Set environment variables for optimal experience:

```bash
# Optional: DeepL API key for better translations (free tier: 500k chars/month)
export DEEPL_API_KEY="your-api-key-here"

# Optional: Custom AnkiConnect URL (default: http://localhost:8765)
export ANKICONNECT_URL="http://localhost:8765"

# Optional: Custom LibreTranslate URL (default: https://libretranslate.com)
export LIBRETRANSLATE_URL="https://libretranslate.com"
```

## Usage

### Test AnkiConnect Connection

```bash
# Verify Anki and AnkiConnect are working
cargo run -- test
# or
make run ARGS="test"
```

### Create a Language Deck

**Interactive mode** (recommended for first use):
```bash
cargo run -- create
# or
make run ARGS="create"
```

The CLI will guide you through:
- 🎯 Selecting target language (language to learn)
- 🏠 Selecting base language (your native language)
- 📚 Choosing or customizing the deck name

**With command-line arguments:**
```bash
cargo run -- create \
  --target-language Croatian \
  --base-language Spanish \
  --words-per-pos 100

# Using language codes
cargo run -- create -t hr -b es

# Custom deck name
cargo run -- create \
  -t Croatian \
  -b Spanish \
  -d "My Croatian Learning Deck"
```

**Dry run mode** (test configuration without creating deck):
```bash
cargo run -- create --dry-run
# or
make run ARGS="create --dry-run"
```

**Example interactive session:**
```
🚀 Anki Deck Builder - Language Learning Deck Creator

? Select target language to learn ›
❯ Croatian (hr)
  Spanish (es)
  English (en)
  [... more ...]

? Select base language (for translations) ›
  Croatian (hr)
❯ Spanish (es)
  [... more ...]

? Use default deck name: 'Croatian → Spanish (Top 800 Words)'? › yes

📋 Configuration Summary:
  Target language: Croatian (hr)
  Base language: Spanish (es)
  Words per part of speech: 100
  Total cards: ~800 (8 parts of speech)
  Deck name: Croatian → Spanish (Top 800 Words)
```

### View Configuration

```bash
anki-deck-builder config --show
```

## Project Structure

```
anki-deck-builder/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # CLI interface
│   ├── config.rs            # Configuration management
│   ├── error.rs             # Error types
│   ├── ankiweb/
│   │   ├── mod.rs
│   │   ├── client.rs        # AnkiConnect client
│   │   └── models.rs        # Data models
│   └── language/
│       ├── mod.rs
│       ├── frequency.rs     # Word frequency data
│       └── translator.rs    # Translation service
├── Cargo.toml
└── README.md
```

## Development

This project includes a [`Makefile`](Makefile:1) with common development tasks:

```bash
# Show all available commands
make help

# Build the project
make build              # Debug mode
make build-release      # Release mode (optimized)

# Run the CLI
make run ARGS="config --show"
make run ARGS="create --target-language Croatian --base-language Spanish"

# Testing
make test               # Run all tests
make test-verbose       # Run tests with output

# Code quality
make fmt                # Format code
make lint               # Run clippy linter
make check              # Check for errors

# Development workflow
make dev                # Run format, lint, test, and build
make ci                 # Run all CI checks

# Other useful commands
make clean              # Clean build artifacts
make doc                # Generate and open documentation
make install            # Install binary to ~/.cargo/bin
```

### Quick Start

```bash
# NixOS: Enter development shell
nix-shell

# Build and run
make build
make run ARGS="config --show"

# Run with logging
RUST_LOG=debug make run ARGS="create"
```

### Documentation

- [📋 Project Plan](docs/PROJECT_PLAN.md) - Complete 8-phase development plan
- [📊 Progress Summary](docs/PROGRESS_SUMMARY.md) - Detailed progress tracking
- [✅ Phase 1 Summary](docs/PHASE1_SUMMARY.md) - Phase 1: Project setup
- [✅ Phase 2 Summary](docs/PHASE2_SUMMARY.md) - Phase 2: AnkiConnect integration
- [✅ Phase 3 Summary](docs/PHASE3_SUMMARY.md) - Phase 3: Interactive CLI interface
- [✅ Phase 4 Summary](docs/PHASE4_SUMMARY.md) - Phase 4: Word frequency data
- [📖 AnkiConnect Setup](docs/ANKICONNECT_SETUP.md) - Detailed setup and troubleshooting guide

## How It Works

1. **Connects to AnkiConnect** - Verifies Anki is running and accessible
2. **Fetches Frequency Data** - Downloads most common Croatian words by part of speech
3. **Translates Words** - Translates Croatian words to Spanish using DeepL/LibreTranslate
4. **Creates Deck** - Generates a new Anki deck with the specified name
5. **Adds Cards** - Creates flashcards with Croatian on front, Spanish on back

## Roadmap

### Phase 1: Project Setup ✅
- [x] Initialize Rust project
- [x] Set up dependencies
- [x] Create project structure

### Phase 2-8: Implementation 🚧
- [ ] AnkiConnect integration
- [ ] CLI interface with interactive prompts
- [ ] Word frequency data fetching
- [ ] Translation service implementation
- [ ] Complete workflow orchestration
- [ ] Testing and refinement

## Future Enhancements

- Direct AnkiWeb sync (no local Anki required)
- Bidirectional cards (Croatian→Spanish and Spanish→Croatian)
- Audio pronunciation
- Example sentences
- Images for nouns
- Custom word lists
- Multiple deck templates
- Support for more language pairs

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Troubleshooting

### AnkiConnect Not Found

Ensure:
1. Anki is running
2. AnkiConnect add-on is installed
3. No firewall blocking localhost:8765

### Translation Errors

If you encounter rate limits:
1. Add a DeepL API key (free tier available)
2. Reduce `--words-per-pos` to create smaller decks
3. Wait a few minutes and try again

### Missing Frequency Data

The tool will automatically download frequency data on first run. Ensure you have internet connectivity.

## Credits

- Frequency data sources: Leipzig Corpora Collection, Hermit Dave's FrequencyWords
- Translation: DeepL API, LibreTranslate
- Anki integration: AnkiConnect by FooSoft
