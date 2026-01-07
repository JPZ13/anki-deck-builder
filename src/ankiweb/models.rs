use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub deck_name: String,
    pub model_name: String,
    pub fields: HashMap<String, String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteField {
    pub front: String,
    pub back: String,
}

impl Note {
    pub fn new(deck_name: String, front: String, back: String) -> Self {
        let mut fields = HashMap::new();
        fields.insert("Front".to_string(), front);
        fields.insert("Back".to_string(), back);

        Self {
            deck_name,
            model_name: "Basic".to_string(),
            fields,
            tags: vec![
                "auto-generated".to_string(),
                "language-learning".to_string(),
            ],
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}
