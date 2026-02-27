use serde::{Deserialize, Serialize};

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    En,
    Fr,
}

/// A string localized in both English and French
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedString {
    pub en: String,
    pub fr: String,
}

impl LocalizedString {
    pub fn new(en: &str, fr: &str) -> Self {
        Self {
            en: en.to_string(),
            fr: fr.to_string(),
        }
    }

    /// Get the string for the given language
    pub fn get(&self, lang: Language) -> &str {
        match lang {
            Language::En => &self.en,
            Language::Fr => &self.fr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localized_string_get() {
        let s = LocalizedString::new("Hello", "Bonjour");
        assert_eq!(s.get(Language::En), "Hello");
        assert_eq!(s.get(Language::Fr), "Bonjour");
    }

    #[test]
    fn test_language_serialization() {
        let lang = Language::En;
        let json = serde_json::to_string(&lang).unwrap();
        assert_eq!(json, "\"En\"");
        let deserialized: Language = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Language::En);
    }
}
