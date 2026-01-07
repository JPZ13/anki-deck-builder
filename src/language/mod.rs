pub mod frequency;
pub mod languages;
pub mod translator;

pub use frequency::{FrequencyData, PartOfSpeech, Word};
pub use languages::{Language, get_language, get_prioritized_languages, is_supported};
pub use translator::Translator;
