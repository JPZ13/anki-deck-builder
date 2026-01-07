use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Pronoun,
    Conjunction,
    Interjection,
}

impl PartOfSpeech {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Noun,
            Self::Verb,
            Self::Adjective,
            Self::Adverb,
            Self::Preposition,
            Self::Pronoun,
            Self::Conjunction,
            Self::Interjection,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub text: String,
    pub pos: PartOfSpeech,
    pub frequency: usize,
    pub rank: usize,
}

impl Word {
    pub fn new(text: String, pos: PartOfSpeech, rank: usize) -> Self {
        Self {
            text,
            pos,
            frequency: 0,
            rank,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyData {
    pub language: String,
    pub words: HashMap<PartOfSpeech, Vec<Word>>,
}

impl FrequencyData {
    pub fn new(language: String) -> Self {
        Self {
            language,
            words: HashMap::new(),
        }
    }

    pub fn add_word(&mut self, word: Word) {
        self.words.entry(word.pos.clone()).or_default().push(word);
    }

    pub fn get_top_words(&self, pos: &PartOfSpeech, count: usize) -> Vec<Word> {
        self.words
            .get(pos)
            .map(|words| words.iter().take(count).cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_all_top_words(&self, count_per_pos: usize) -> Vec<Word> {
        let mut all_words = Vec::new();
        for pos in PartOfSpeech::all() {
            all_words.extend(self.get_top_words(&pos, count_per_pos));
        }
        all_words
    }
}
