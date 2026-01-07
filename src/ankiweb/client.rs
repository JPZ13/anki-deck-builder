use crate::error::{AnkiDeckBuilderError, Result};
use crate::ankiweb::models::Note;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct AnkiClient {
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct AnkiRequest<T> {
    action: String,
    version: u32,
    params: T,
}

#[derive(Deserialize, Debug)]
struct AnkiResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

impl AnkiClient {
    pub fn new(base_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        Ok(Self { base_url, client })
    }

    /// Verify that AnkiConnect is running and accessible
    pub async fn verify_connection(&self) -> Result<()> {
        debug!("Verifying connection to AnkiConnect at {}", self.base_url);
        
        let request = AnkiRequest {
            action: "version".to_string(),
            version: 6,
            params: json!({}),
        };

        let response = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await
            .map_err(|_| AnkiDeckBuilderError::AnkiConnectNotRunning {
                url: self.base_url.clone(),
            })?;

        let anki_response: AnkiResponse<u32> = response
            .json()
            .await
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        if let Some(error) = anki_response.error {
            return Err(AnkiDeckBuilderError::AnkiConnectError(error));
        }

        info!("Successfully connected to AnkiConnect (version: {:?})", anki_response.result);
        Ok(())
    }

    /// Create a new deck
    pub async fn create_deck(&self, name: &str) -> Result<i64> {
        debug!("Creating deck: {}", name);
        
        let request = AnkiRequest {
            action: "createDeck".to_string(),
            version: 6,
            params: json!({ "deck": name }),
        };

        let response = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        let anki_response: AnkiResponse<i64> = response
            .json()
            .await
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        if let Some(error) = anki_response.error {
            return Err(AnkiDeckBuilderError::AnkiConnectError(error));
        }

        let deck_id = anki_response.result.ok_or_else(|| {
            AnkiDeckBuilderError::AnkiConnectError("No deck ID returned".to_string())
        })?;

        info!("Created deck '{}' with ID: {}", name, deck_id);
        Ok(deck_id)
    }

    /// Add a note to a deck
    pub async fn add_note(&self, note: &Note) -> Result<i64> {
        debug!("Adding note to deck: {}", note.deck_name);
        
        let request = AnkiRequest {
            action: "addNote".to_string(),
            version: 6,
            params: json!({
                "note": {
                    "deckName": note.deck_name,
                    "modelName": note.model_name,
                    "fields": note.fields,
                    "tags": note.tags,
                }
            }),
        };

        let response = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        let anki_response: AnkiResponse<i64> = response
            .json()
            .await
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        if let Some(error) = anki_response.error {
            return Err(AnkiDeckBuilderError::AnkiConnectError(error));
        }

        let note_id = anki_response.result.ok_or_else(|| {
            AnkiDeckBuilderError::AnkiConnectError("No note ID returned".to_string())
        })?;

        debug!("Added note with ID: {}", note_id);
        Ok(note_id)
    }

    /// Get list of all deck names
    pub async fn get_decks(&self) -> Result<Vec<String>> {
        debug!("Fetching deck names");
        
        let request = AnkiRequest {
            action: "deckNames".to_string(),
            version: 6,
            params: json!({}),
        };

        let response = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        let anki_response: AnkiResponse<Vec<String>> = response
            .json()
            .await
            .map_err(|e| AnkiDeckBuilderError::HttpError(e))?;

        if let Some(error) = anki_response.error {
            return Err(AnkiDeckBuilderError::AnkiConnectError(error));
        }

        anki_response.result.ok_or_else(|| {
            AnkiDeckBuilderError::AnkiConnectError("No deck names returned".to_string())
        })
    }
}
