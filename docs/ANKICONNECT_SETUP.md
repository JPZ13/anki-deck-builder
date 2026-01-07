# AnkiConnect Setup Guide

AnkiConnect is a browser add-on for Anki that enables external applications to communicate with Anki via a JSON-RPC API over HTTP.

## Installation

### 1. Install Anki Desktop

**NixOS:**
```bash
# Anki is included in the shell.nix
nix-shell
```

**Other Linux:**
```bash
# Download from https://apps.ankiweb.net
# Or use your package manager
sudo apt install anki  # Debian/Ubuntu
```

### 2. Install AnkiConnect Add-on

1. Open Anki
2. Navigate to: **Tools → Add-ons → Get Add-ons...**
3. Enter the code: `2055492159`
4. Click **OK**
5. Restart Anki

### 3. Verify Installation

#### Using the CLI

```bash
# Build and run the test command
cargo run -- test

# Or with make
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

#### Manual Verification

Send a test request:
```bash
curl http://localhost:8765 -X POST -d '{"action": "version", "version": 6}'
```

Expected response:
```json
{"result": 6, "error": null}
```

## Configuration

### Default Settings

AnkiConnect listens on `http://localhost:8765` by default.

### Custom URL

Set the `ANKICONNECT_URL` environment variable:

```bash
export ANKICONNECT_URL="http://localhost:8765"
cargo run -- config --show
```

### CORS Settings (Optional)

If you need to access AnkiConnect from a web browser or remote machine, you may need to configure CORS settings in AnkiConnect.

1. In Anki: **Tools → Add-ons → AnkiConnect → Config**
2. Modify the `webCorsOriginList` as needed

Example config:
```json
{
    "apiKey": null,
    "apiLogPath": null,
    "webBindAddress": "127.0.0.1",
    "webBindPort": 8765,
    "webCorsOriginList": [
        "http://localhost"
    ]
}
```

## Troubleshooting

### "Failed to connect to AnkiConnect"

**Possible causes:**

1. **Anki is not running**
   - Start Anki desktop application
   - Anki must remain open for AnkiConnect to work

2. **AnkiConnect add-on not installed**
   - Follow installation steps above
   - Code: `2055492159`

3. **Anki was just started**
   - Wait a few seconds for AnkiConnect to initialize
   - Try the test command again

4. **Port conflict**
   - Another application might be using port 8765
   - Check with: `lsof -i :8765` (Linux/macOS) or `netstat -ano | findstr :8765` (Windows)

5. **Firewall blocking localhost**
   - Unlikely but possible
   - Check firewall settings

### "Could not retrieve decks"

- AnkiConnect is running but may have permission issues
- Try restarting Anki
- Check Anki add-on logs: **Tools → Add-ons → AnkiConnect → View Files**

### Testing with Integration Tests

Run the full integration test suite (requires Anki to be running):

```bash
# Run ignored tests (these require AnkiConnect)
cargo test -- --ignored

# Or with make
make test ARGS="-- --ignored"
```

**Warning:** Integration tests will create a test deck in your Anki collection. You may want to delete it manually after testing.

## API Reference

Common AnkiConnect actions used by this project:

### version
Get AnkiConnect version
```json
{"action": "version", "version": 6}
```

### deckNames
List all deck names
```json
{"action": "deckNames", "version": 6}
```

### createDeck
Create a new deck
```json
{
    "action": "createDeck",
    "version": 6,
    "params": {"deck": "My Deck"}
}
```

### addNote
Add a note (flashcard) to a deck
```json
{
    "action": "addNote",
    "version": 6,
    "params": {
        "note": {
            "deckName": "My Deck",
            "modelName": "Basic",
            "fields": {
                "Front": "Croatian word",
                "Back": "Spanish translation"
            },
            "tags": ["auto-generated", "language-learning"]
        }
    }
}
```

## Resources

- [AnkiConnect GitHub](https://github.com/FooSoft/anki-connect)
- [AnkiConnect API Documentation](https://foosoft.net/projects/anki-connect/)
- [Anki Manual](https://docs.ankiweb.net/)
