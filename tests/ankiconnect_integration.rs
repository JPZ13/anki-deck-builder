use anki_deck_builder::ankiweb::{AnkiClient, Note};

/// Tests require AnkiConnect to be running on localhost:8765
/// Skip these tests if AnkiConnect is not available
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_ankiconnect_connection() {
    let client = AnkiClient::new("http://localhost:8765".to_string())
        .expect("Failed to create client");
    
    let result = client.verify_connection().await;
    assert!(result.is_ok(), "Failed to connect to AnkiConnect: {:?}", result.err());
}

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_get_decks() {
    let client = AnkiClient::new("http://localhost:8765".to_string())
        .expect("Failed to create client");
    
    client.verify_connection().await.expect("Connection failed");
    
    let decks = client.get_decks().await;
    assert!(decks.is_ok(), "Failed to get decks: {:?}", decks.err());
    
    let deck_list = decks.unwrap();
    assert!(!deck_list.is_empty(), "Deck list should not be empty (at least 'Default' should exist)");
    assert!(deck_list.contains(&"Default".to_string()), "Default deck should exist");
}

#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_create_and_add_note() {
    let client = AnkiClient::new("http://localhost:8765".to_string())
        .expect("Failed to create client");
    
    client.verify_connection().await.expect("Connection failed");
    
    // Create a test deck
    let deck_name = "AnkiDeckBuilder_Test";
    let deck_result = client.create_deck(deck_name).await;
    assert!(deck_result.is_ok(), "Failed to create deck: {:?}", deck_result.err());
    
    // Add a test note
    let note = Note::new(
        deck_name.to_string(),
        "Test Croatian Word".to_string(),
        "Test Spanish Translation".to_string(),
    );
    
    let note_result = client.add_note(&note).await;
    assert!(note_result.is_ok(), "Failed to add note: {:?}", note_result.err());
    
    println!("Successfully created deck and added note!");
    println!("Note: You may want to manually delete the '{}' deck in Anki.", deck_name);
}
