# Anki Deck Builder - User Guide

## Quick Start

### 1. Setup (One-Time)

**For NixOS users:**
```bash
cd /home/jp/code/anki-deck-builder
nix-shell
# or with direnv
direnv allow
```

**Install Anki and AnkiConnect:**
1. Open Anki desktop application
2. Go to: Tools → Add-ons → Get Add-ons
3. Enter code: `2055492159`
4. Restart Anki
5. Keep Anki running when using the deck builder

### 2. Test Your Setup

```bash
make run ARGS="test"
```

Expected output:
```
🔍 Testing AnkiConnect connection...
📍 AnkiConnect URL: http://localhost:8765
✅ Successfully connected to AnkiConnect!
📚 Available decks (1):
  - Default
```

### 3. Create Your First Deck

**Interactive mode (recommended):**
```bash
make run ARGS="create"
```

**Command-line mode:**
```bash
make run ARGS="create -t hr -b es"
```

## Complete Workflow Example

```bash
$ make run ARGS="create -t Croatian -b Spanish"

🚀 Anki Deck Builder - Language Learning Deck Creator

🎯 Target language: Croatian (hr)
🏠 Base language: Spanish (es)
? Use default deck name: 'Croatian → Spanish (Top 800 Words)'? (y/n) › yes
📚 Deck name: Croatian → Spanish (Top 800 Words)

📋 Configuration Summary:
  Target language: Croatian (hr)
  Base language: Spanish (es)
  Words per part of speech: 100
  Total cards: ~800 (8 parts of speech)
  Deck name: Croatian → Spanish (Top 800 Words)
  Dry run: false

📊 Loading Croatian word frequency data...
✅ Loaded Croatian word data

📝 Word selection:
  Nouns: 20 words
  Verbs: 15 words
  Adjectives: 10 words
  Total: 45 words selected

🔤 Sample words:
  - dan (Noun)
  - vrijeme (Noun)
  - dio (Noun)
  - način (Noun)
  - godina (Noun)
  ... and 40 more

🌐 Translating 45 words from Croatian to Spanish...
Translating [========================================] 45/45 (100%)
✅ Translation complete

📝 Sample translations:
  dan → día (Noun)
  vrijeme → tiempo (Noun)
  dio → parte (Noun)
  način → manera (Noun)
  biti → ser (Verb)
  moći → poder (Verb)
  ... and 39 more

📚 Creating Anki deck: 'Croatian → Spanish (Top 800 Words)'...
✅ Connected to AnkiConnect
✅ Created deck with ID: 1705453200

📝 Adding 45 cards to deck...
Adding cards [========================================] 45/45 (100%)
✅ Cards added

🎉 Deck creation complete!
  ✅ 45 cards added successfully
  📚 Deck name: Croatian → Spanish (Top 800 Words)

💡 Open Anki to start studying your 45 words!
```

## Command Reference

### Create Deck

```bash
# Full interactive mode
cargo run -- create

# Specify languages
cargo run -- create --target-language Croatian --base-language Spanish

# Use language codes
cargo run -- create -t hr -b es

# Custom deck name
cargo run -- create -t hr -b es --deck-name "My Croatian Deck"

# Adjust word count per part of speech
cargo run -- create -t hr -b es --words-per-pos 50

# Dry run (test configuration without creating)
cargo run -- create -t hr -b es --dry-run
```

### Test Connection

```bash
cargo run -- test
# or
make run ARGS="test"
```

### View Configuration

```bash
cargo run -- config --show
```

## Card Format

Cards are created in the "Basic" Anki format:

**Front (Croatian):**
```
dan
```

**Back (Spanish with POS tag):**
```
día
<small><i>Noun</i></small>
```

## Supported Languages

The CLI currently supports 20+ languages:

**MVP Languages (prioritized):**
- 🇭🇷 Croatian (hr)
- 🇪🇸 Spanish (es)

**Additional Languages:**
- Arabic (ar), Chinese (zh), Danish (da), Dutch (nl)
- English (en), Finnish (fi), French (fr), German (de)
- Greek (el), Hindi (hi), Italian (it), Japanese (ja)
- Korean (ko), Norwegian (no), Polish (pl), Portuguese (pt)
- Russian (ru), Swedish (sv), Turkish (tr)

## Configuration

### Environment Variables

```bash
# Optional: Custom AnkiConnect URL
export ANKICONNECT_URL="http://localhost:8765"

# Optional: Custom LibreTranslate server
export LIBRETRANSLATE_URL="https://libretranslate.com"

# Optional: DeepL API key (not yet implemented)
export DEEPL_API_KEY="your-key"
```

### Cache Locations

Data is cached in `~/.local/share/anki-deck-builder/`:

```
~/.local/share/anki-deck-builder/
├── frequency/
│   ├── hr_frequency.json      # Croatian word data
│   └── es_frequency.json      # Spanish word data
└── translations/
    └── hr_es.json              # Croatian→Spanish translations
```

**Benefits:**
- Faster subsequent runs
- Works offline after first run
- Reduces API calls

**Clear cache:**
```bash
rm -rf ~/.local/share/anki-deck-builder/
```

## Troubleshooting

### "Failed to connect to AnkiConnect"

1. **Is Anki running?**
   - Start Anki desktop application
   - Keep it running while using the deck builder

2. **Is AnkiConnect installed?**
   - Tools → Add-ons → Get Add-ons
   - Code: `2055492159`
   - Restart Anki after installation

3. **Test the connection:**
   ```bash
   make run ARGS="test"
   ```

4. **Manual verification:**
   ```bash
   curl http://localhost:8765 -X POST -d '{"action": "version", "version": 6}'
   ```
   Expected: `{"result": 6, "error": null}`

### "Translation failed"

1. **Check internet connection**
   - LibreTranslate requires internet access
   - First run may be slow due to API calls

2. **Try again**
   - Temporary API issues may resolve
   - Cached translations will be used on retry

3. **Check rate limits**
   - The tool waits 100ms between translations
   - For large decks, this is expected behavior

### "Cards failed (may be duplicates)"

This is normal! Anki prevents duplicate cards. If you run the tool multiple times with the same deck name, previously added cards will fail to add again.

**Solutions:**
- Use a different deck name
- Delete the deck in Anki and recreate
- Ignore these warnings (existing cards are preserved)

### "No words found for language"

Currently only Croatian (hr) and Spanish (es) have embedded word data. Other languages will show minimal or no words until external data sources are added in future versions.

## Tips & Best Practices

### First Run

1. **Start small:** Use `--words-per-pos 10` for testing
2. **Test connection:** Run `make run ARGS="test"` first
3. **Dry run:** Use `--dry-run` to verify configuration

### Optimizing Performance

1. **Cache is your friend:** Subsequent runs are 100x faster
2. **Batch processing:** The tool handles rate limiting automatically
3. **Offline mode:** After first run, works offline (except new translations)

### Studying Tips

1. **Review regularly:** Use Anki's spaced repetition schedule
2. **Start with high-frequency words:** Most bang for your buck
3. **Group by POS:** Create separate decks if you want focused practice
4. **Add context:** Edit cards in Anki to add example sentences

## Advanced Usage

### Custom Word Counts

```bash
# Small deck (50 words per POS = ~400 cards)
make run ARGS="create -t hr -b es --words-per-pos 50"

# Large deck (200 words per POS = ~1600 cards)
make run ARGS="create -t hr -b es --words-per-pos 200"
```

### Multiple Decks

Create different decks for different focuses:

```bash
# Basic vocabulary
make run ARGS="create -t hr -b es --words-per-pos 50 -d 'Croatian Basics'"

# Advanced vocabulary
make run ARGS="create -t hr -b es --words-per-pos 200 -d 'Croatian Advanced'"
```

## Development Commands

```bash
make help       # Show all commands
make build      # Build the project
make test       # Run unit tests
make lint       # Run clippy linter
make fmt        # Format code
make dev        # Run all checks (format, lint, test, build)
make doc        # Generate documentation
```

## Support & Resources

- [Project README](../README.md)
- [AnkiConnect Setup Guide](ANKICONNECT_SETUP.md)
- [Project Plan](PROJECT_PLAN.md)
- [Progress Summary](PROGRESS_SUMMARY.md)

## Future Enhancements

Planned features for future releases:

- **Expand word lists:** Full 100+ words per POS
- **External data sources:** Leipzig Corpora, hrWaC integration
- **Audio pronunciation:** Forvo API integration
- **Example sentences:** Context for each word
- **Images:** Visual aids for nouns
- **Bidirectional cards:** Spanish→Croatian option
- **More language pairs:** Beyond Croatian-Spanish
- **DeepL integration:** Higher quality translations
- **Custom word lists:** Upload your own vocabulary

## Contributing

Found a bug or want to contribute? The project is well-structured and ready for extensions!

Key areas for contribution:
- Additional language frequency data
- External data source integrations
- Translation service providers
- UI/UX improvements
- Performance optimizations

---

**Happy learning! 🎓📚**
