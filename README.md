# Anki Deck Builder

**A Rust CLI tool that automatically creates language learning decks for Anki.**

Automatically generate Anki flashcard decks with the most frequently used words in your target language, paired with translations in your native language.

**🇪🇸 → 🇭🇷 Spanish to Croatian language learning**

Fetches **50,000 Croatian words** from Hermit Dave's FrequencyWords repository, automatically categorizes by part of speech, and creates Anki flashcards with Spanish translations.

---

## Features

- 🎯 **Automatic deck generation** - Create full Anki decks with one command
- 🔄 **Bidirectional cards** - Practice both recognition and production (default)
- � **Frequency-based** - Learn the most common words first
- 🔤 **Organized by grammar** - Words categorized by part of speech
- 🌐 **Auto-translation** - Powered by LibreTranslate API
- 💾 **Smart caching** - Fast repeat runs, works offline
- 🎨 **Beautiful CLI** - Interactive prompts with progress indicators

---

## Quick Start

### Prerequisites (NixOS)

```bash
# Enter development environment (includes Rust and Anki)
nix-shell

# Or use direnv for automatic activation
direnv allow
```

### Prerequisites (Other Systems)

1. Install [Rust](https://rustup.rs)
2. Install [Anki Desktop](https://apps.ankiweb.net)

### Install AnkiConnect

In Anki:
1. Tools → Add-ons → Get Add-ons
2. Enter code: `2055492159`
3. Restart Anki

[Detailed setup guide →](docs/ANKICONNECT_SETUP.md)

---

## Usage

### Test Connection

```bash
make run ARGS="test"
```

### Create a Deck

**Interactive mode** (recommended):
```bash
make run ARGS="create"
```

The CLI will prompt you for:
- Target language (language to learn)
- Base language (your known language)  
- Deck name

**Command-line mode:**
```bash
# Create bidirectional deck (default - 90 cards: 45 words × 2 directions)
make run ARGS="create -t hr -b es"

# Create unidirectional deck (45 cards: Croatian→Spanish only)
make run ARGS="create -t hr -b es --bidirectional=false"

# Customize word count
make run ARGS="create -t hr -b es --words-per-pos 50"

# Custom deck name
make run ARGS="create -t hr -b es -d 'My Vocabulary'"

# Test configuration without creating
make run ARGS="create --dry-run"
```

---

## Example Session

```bash
$ make run ARGS="create -t Croatian -b Spanish"

🚀 Anki Deck Builder - Language Learning Deck Creator

📊 Loading Croatian word frequency data...
✅ Loaded Croatian word data (45 words)

🌐 Translating 45 words from Croatian to Spanish...
[========================================] 45/45 (100%)
✅ Translation complete

📚 Creating Anki deck...
✅ Connected to AnkiConnect
✅ Created deck: 'Croatian → Spanish (Top 800 Words)'

📝 Adding 45 cards to deck...
[========================================] 45/45 (100%)
✅ Cards added

🎉 Deck creation complete!
  ✅ 45 cards added successfully
  📚 Deck: Croatian → Spanish (Top 800 Words)

💡 Open Anki to start studying!
```

---

## Card Format

### Bidirectional Cards (Default)

By default, creates **2 cards per word** for comprehensive learning:

**Card 1 - Recognition:** Croatian → Spanish
- **Front:** `dan`
- **Back:** `día` *(Noun)*
- *You see Croatian and recall the Spanish meaning*

**Card 2 - Production:** Spanish → Croatian
- **Front:** `día`
- **Back:** `dan` *(Noun)*
- *You see Spanish and produce the Croatian word*

### Unidirectional Cards

Use `--bidirectional=false` for recognition-only (Croatian →  Spanish):
- **Front:** `dan`
- **Back:** `día` *(Noun)*

---

## Configuration

### Environment Variables

```bash
# Optional: Custom AnkiConnect URL (default: http://localhost:8765)
export ANKICONNECT_URL="http://localhost:8765"

# Optional: Custom LibreTranslate server
export LIBRETRANSLATE_URL="https://libretranslate.com"
```

### View Configuration

```bash
make run ARGS="config --show"
```

### Cache Location

Data is cached in `~/.local/share/anki-deck-builder/`:
- `frequency/` - Word frequency lists
- `translations/` - Translated words

**Clear cache:**
```bash
rm -rf ~/.local/share/anki-deck-builder/
```

---

## Development

### Build & Test

```bash
# Show all available commands
make help

# Build
make build              # Debug mode
make build-release      # Release mode

# Test
make test               # Run unit tests
make check              # Quick compile check

# Code quality
make fmt                # Format code
make-lint               # Run linter
make dev                # Run all checks
```

### Project Structure

```
anki-deck-builder/
├── src/
│   ├── cli.rs               # CLI interface
│   ├── ankiweb/             # AnkiConnect client
│   └── language/            # Translation & frequency data
├── tests/                   # Integration tests
├── docs/                    # Documentation
├── Makefile                 # Development commands
└── shell.nix                # NixOS environment
```

---

## Supported Languages

Croatian, Spanish, English, French, German, Italian, Portuguese, Russian, Japanese, Korean, Chinese, Arabic, Hindi, Dutch, Polish, Swedish, Norwegian, Danish, Finnish, Greek, Turkish

Note: Full frequency data currently available for Croatian and Spanish. Other languages have basic support.

---

## Troubleshooting

### "Failed to connect to AnkiConnect"

1. Make sure Anki is running
2. Verify AnkiConnect is installed (code: 2055492159)
3. Test with: `make run ARGS="test"`

[Complete troubleshooting guide →](docs/ANKICONNECT_SETUP.md)

### "Translation failed"

- Requires internet connection for first run
- Subsequent runs use cached translations
- Try again if API is temporarily unavailable

### "Cards failed (may be duplicates)"

This is normal if you've run the tool multiple times. Anki prevents duplicate cards. Use a different deck name or delete the existing deck first.

---

## Documentation

**User Guides:**
- [User Guide](docs/USER_GUIDE.md) - Complete usage manual
- [AnkiConnect Setup](docs/ANKICONNECT_SETUP.md) - Installation & troubleshooting

**Developer Docs:**
- [Project Complete](docs/PROJECT_COMPLETE.md) - Project summary and statistics
- [Project Plan](docs/PROJECT_PLAN.md) - Technical architecture

---

## Future Enhancements

- Expand word datasets (800+ words)
- External data sources (Leipzig Corpora)
- Audio pronunciation
- Example sentences
- Image support
- More language pairs
- DeepL API integration

---

## License

MIT License

## Contributing

Contributions welcome! The project has a modular architecture that's easy to extend.

---

**Built with:** Rust 2021 | **Dependencies:** Tokio, reqwest, clap, dialoguer, indicatif  
**Platforms:** Linux, macOS, Windows (via rustls-tls)
