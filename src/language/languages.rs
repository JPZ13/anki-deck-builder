use std::collections::HashMap;

/// Language information with ISO 639-1 code and full name
#[derive(Debug, Clone)]
pub struct Language {
    pub code: String,
    pub name: String,
}

impl Language {
    pub fn new(code: &str, name: &str) -> Self {
        Self {
            code: code.to_string(),
            name: name.to_string(),
        }
    }
}

/// Get a language by code or name (case-insensitive)
pub fn get_language(input: &str) -> Option<Language> {
    let input_lower = input.to_lowercase();

    // Try as code first
    if let Some(name) = get_language_name(&input_lower) {
        return Some(Language::new(&input_lower, name));
    }

    // Try as name
    if let Some(code) = get_language_code(&input_lower) {
        return Some(Language::new(code, input));
    }

    None
}

/// Get language name from code
fn get_language_name(code: &str) -> Option<&'static str> {
    let languages = get_supported_languages_map();
    languages.get(code).copied()
}

/// Get language code from name (case-insensitive)
fn get_language_code(name: &str) -> Option<&'static str> {
    let languages = get_supported_languages_map();
    let name_lower = name.to_lowercase();

    languages
        .iter()
        .find(|(_, &lang_name)| lang_name.to_lowercase() == name_lower)
        .map(|(code, _)| *code)
}

/// Supported languages for MVP (focused on Croatian and Spanish)
fn get_supported_languages_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // MVP languages
    map.insert("hr", "Croatian");
    map.insert("es", "Spanish");

    // Additional common languages for future expansion
    map.insert("en", "English");
    map.insert("fr", "French");
    map.insert("de", "German");
    map.insert("it", "Italian");
    map.insert("pt", "Portuguese");
    map.insert("ru", "Russian");
    map.insert("ja", "Japanese");
    map.insert("ko", "Korean");
    map.insert("zh", "Chinese");
    map.insert("ar", "Arabic");
    map.insert("hi", "Hindi");
    map.insert("nl", "Dutch");
    map.insert("pl", "Polish");
    map.insert("sv", "Swedish");
    map.insert("no", "Norwegian");
    map.insert("da", "Danish");
    map.insert("fi", "Finnish");
    map.insert("el", "Greek");
    map.insert("tr", "Turkish");

    map
}

/// Get list of supported languages for selection
pub fn get_supported_languages() -> Vec<Language> {
    let mut languages: Vec<Language> = get_supported_languages_map()
        .iter()
        .map(|(&code, &name)| Language::new(code, name))
        .collect();

    // Sort by name for better UX
    languages.sort_by(|a, b| a.name.cmp(&b.name));

    languages
}

/// Get prioritized languages for selection (MVP languages first)
pub fn get_prioritized_languages() -> Vec<Language> {
    let mut languages = Vec::new();

    // MVP languages first
    languages.push(Language::new("hr", "Croatian"));
    languages.push(Language::new("es", "Spanish"));

    // Then common languages
    languages.push(Language::new("en", "English"));
    languages.push(Language::new("fr", "French"));
    languages.push(Language::new("de", "German"));
    languages.push(Language::new("it", "Italian"));
    languages.push(Language::new("pt", "Portuguese"));

    // Then rest alphabetically
    let mut others = get_supported_languages();
    others.retain(|lang| !["hr", "es", "en", "fr", "de", "it", "pt"].contains(&lang.code.as_str()));
    languages.extend(others);

    languages
}

/// Validate if a language is supported
pub fn is_supported(code_or_name: &str) -> bool {
    get_language(code_or_name).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_language_by_code() {
        let lang = get_language("hr").unwrap();
        assert_eq!(lang.code, "hr");
        assert_eq!(lang.name, "Croatian");
    }

    #[test]
    fn test_get_language_by_name() {
        let lang = get_language("Croatian").unwrap();
        assert_eq!(lang.code, "hr");
        assert_eq!(lang.name, "Croatian");
    }

    #[test]
    fn test_case_insensitive() {
        assert!(get_language("CROATIAN").is_some());
        assert!(get_language("croatian").is_some());
        assert!(get_language("HR").is_some());
    }

    #[test]
    fn test_unsupported_language() {
        assert!(get_language("xyz").is_none());
        assert!(get_language("Klingon").is_none());
    }

    #[test]
    fn test_is_supported() {
        assert!(is_supported("hr"));
        assert!(is_supported("Croatian"));
        assert!(is_supported("es"));
        assert!(is_supported("Spanish"));
        assert!(!is_supported("xyz"));
    }
}
