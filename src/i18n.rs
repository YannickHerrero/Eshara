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

/// System message keys for all UI/menu text
#[allow(dead_code)]
pub enum Msg {
    LanguagePrompt,
    LanguageOption1,
    LanguageOption2,
    ContinueOrNew,
    ContinueOption,
    NewGameOption,
    ElaraTyping,
    ElaraUnavailable,
    ElaraBackAround,
    WaitOrQuit,
    WaitOption,
    QuitOption,
    SignalLost,
    DaySeparator,
    BacklogHeader,
    SessionStart,
    EndingReached,
    DaysSurvived,
    KeyChoices,
    PlayAgain,
    YesOption,
    NoOption,
    InvalidChoice,
    SaveDeleted,
    IntroRadioCrackle,
}

/// Get a localized system message
pub fn sys_msg(key: Msg, lang: Language) -> &'static str {
    match (key, lang) {
        // Language selection (shown before language is chosen, so both are hardcoded)
        (Msg::LanguagePrompt, _) => "Choose your language / Choisissez votre langue:",
        (Msg::LanguageOption1, _) => "1. English",
        (Msg::LanguageOption2, _) => "2. Fran\u{00e7}ais",

        // Continue or new game
        (Msg::ContinueOrNew, Language::En) => "A save file was found. What would you like to do?",
        (Msg::ContinueOrNew, Language::Fr) => "Une sauvegarde a \u{00e9}t\u{00e9} trouv\u{00e9}e. Que voulez-vous faire ?",
        (Msg::ContinueOption, Language::En) => "1. Continue",
        (Msg::ContinueOption, Language::Fr) => "1. Continuer",
        (Msg::NewGameOption, Language::En) => "2. New Game",
        (Msg::NewGameOption, Language::Fr) => "2. Nouvelle Partie",

        // Typing indicator
        (Msg::ElaraTyping, Language::En) => "Elara is typing",
        (Msg::ElaraTyping, Language::Fr) => "Elara \u{00e9}crit",

        // Waiting
        (Msg::ElaraUnavailable, Language::En) => "Elara is not available right now.",
        (Msg::ElaraUnavailable, Language::Fr) => "Elara n'est pas disponible pour le moment.",
        (Msg::ElaraBackAround, Language::En) => "She said she'd be back around",
        (Msg::ElaraBackAround, Language::Fr) => "Elle a dit qu'elle reviendrait vers",
        (Msg::WaitOrQuit, Language::En) => "What would you like to do?",
        (Msg::WaitOrQuit, Language::Fr) => "Que voulez-vous faire ?",
        (Msg::WaitOption, Language::En) => "1. Wait",
        (Msg::WaitOption, Language::Fr) => "1. Attendre",
        (Msg::QuitOption, Language::En) => "2. Quit and come back later",
        (Msg::QuitOption, Language::Fr) => "2. Quitter et revenir plus tard",

        // Signal lost (Ctrl+C)
        (Msg::SignalLost, Language::En) => "Signal lost...",
        (Msg::SignalLost, Language::Fr) => "Signal perdu...",

        // Day separator
        (Msg::DaySeparator, Language::En) => "Day",
        (Msg::DaySeparator, Language::Fr) => "Jour",

        // Backlog / session
        (Msg::BacklogHeader, Language::En) => "--- Previous messages ---",
        (Msg::BacklogHeader, Language::Fr) => "--- Messages pr\u{00e9}c\u{00e9}dents ---",
        (Msg::SessionStart, Language::En) => "Session",
        (Msg::SessionStart, Language::Fr) => "Session",

        // Ending screen
        (Msg::EndingReached, Language::En) => "ENDING REACHED",
        (Msg::EndingReached, Language::Fr) => "FIN ATTEINTE",
        (Msg::DaysSurvived, Language::En) => "Days survived:",
        (Msg::DaysSurvived, Language::Fr) => "Jours de survie :",
        (Msg::KeyChoices, Language::En) => "Key choices made:",
        (Msg::KeyChoices, Language::Fr) => "Choix d\u{00e9}terminants :",
        (Msg::PlayAgain, Language::En) => "Play again?",
        (Msg::PlayAgain, Language::Fr) => "Rejouer ?",
        (Msg::YesOption, Language::En) => "1. Yes",
        (Msg::YesOption, Language::Fr) => "1. Oui",
        (Msg::NoOption, Language::En) => "2. No",
        (Msg::NoOption, Language::Fr) => "2. Non",

        // Invalid input
        (Msg::InvalidChoice, Language::En) => "Invalid choice. Please try again.",
        (Msg::InvalidChoice, Language::Fr) => "Choix invalide. Veuillez r\u{00e9}essayer.",

        // Save management
        (Msg::SaveDeleted, Language::En) => "Save file deleted. Starting fresh.",
        (Msg::SaveDeleted, Language::Fr) => "Sauvegarde supprim\u{00e9}e. Red\u{00e9}marrage.",

        // Intro
        (Msg::IntroRadioCrackle, Language::En) => {
            "* krrzzz... krrzzz... *\n\nA faint signal cuts through the static.\nSomeone is trying to reach you."
        }
        (Msg::IntroRadioCrackle, Language::Fr) => {
            "* krrzzz... krrzzz... *\n\nUn faible signal perce \u{00e0} travers le gr\u{00e9}sillement.\nQuelqu'un essaie de vous joindre."
        }
    }
}

/// Parse a language from a CLI argument string
pub fn parse_language(s: &str) -> Option<Language> {
    match s.to_lowercase().as_str() {
        "en" | "english" => Some(Language::En),
        "fr" | "french" | "français" | "francais" => Some(Language::Fr),
        _ => None,
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

    #[test]
    fn test_sys_msg_returns_content() {
        // Verify all messages return non-empty strings
        let msg = sys_msg(Msg::ElaraTyping, Language::En);
        assert_eq!(msg, "Elara is typing");
        let msg = sys_msg(Msg::ElaraTyping, Language::Fr);
        assert_eq!(msg, "Elara \u{00e9}crit");
    }

    #[test]
    fn test_parse_language() {
        assert_eq!(parse_language("en"), Some(Language::En));
        assert_eq!(parse_language("EN"), Some(Language::En));
        assert_eq!(parse_language("fr"), Some(Language::Fr));
        assert_eq!(parse_language("français"), Some(Language::Fr));
        assert_eq!(parse_language("invalid"), None);
    }

    #[test]
    fn test_language_prompt_bilingual() {
        // Language prompt should be the same regardless of language passed
        let en = sys_msg(Msg::LanguagePrompt, Language::En);
        let fr = sys_msg(Msg::LanguagePrompt, Language::Fr);
        assert_eq!(en, fr);
        assert!(en.contains("English") || en.contains("language"));
    }
}
