{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer

    # Build dependencies
    pkg-config
    
    # Anki desktop (required for AnkiConnect)
    anki
    
    # Optional: if you need to debug HTTP requests
    # curl
    # jq
  ];

  shellHook = ''
    echo "🦀 Anki Deck Builder development environment"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
    echo "📝 Next steps:"
    echo "  1. Run Anki and install AnkiConnect add-on (code: 2055492159)"
    echo "  2. Build: cargo build"
    echo "  3. Run: cargo run -- config --show"
    echo ""
    echo "💡 Tip: Set environment variables for API keys:"
    echo "  export DEEPL_API_KEY='your-key'"
  '';
}
