use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::i18n::Language;
use crate::story::EndingType;

/// A single entry in the message log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Who sent the message: Elara or Player
    pub sender: Sender,
    /// The displayed text (already resolved to the correct language)
    pub text: String,
    /// When this message was displayed
    pub timestamp: DateTime<Utc>,
}

/// Who sent a message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sender {
    Elara,
    Player,
    System,
}

/// Tracked stats that affect story gates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub trust_level: i32,
    pub health: i32,
    pub supplies: i32,
    pub morale: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            trust_level: 3,
            health: 10,
            supplies: 5,
            morale: 5,
        }
    }
}

impl Stats {
    /// Get a stat value by name
    pub fn get(&self, name: &str) -> Option<i32> {
        match name {
            "trust_level" => Some(self.trust_level),
            "health" => Some(self.health),
            "supplies" => Some(self.supplies),
            "morale" => Some(self.morale),
            _ => None,
        }
    }

    /// Modify a stat by name with a delta
    pub fn modify(&mut self, name: &str, delta: i32) {
        match name {
            "trust_level" => self.trust_level = (self.trust_level + delta).max(0).min(10),
            "health" => self.health = (self.health + delta).max(0).min(10),
            "supplies" => self.supplies = (self.supplies + delta).max(0).min(10),
            "morale" => self.morale = (self.morale + delta).max(0).min(10),
            _ => {}
        }
    }
}

/// The full game state, serialized to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Current story node id
    pub current_node: String,
    /// Story flags set by choices
    pub flags: HashMap<String, bool>,
    /// Selected language
    pub language: Language,
    /// If set, Elara is busy until this timestamp
    pub waiting_until: Option<DateTime<Utc>>,
    /// History of displayed messages
    pub message_log: Vec<LogEntry>,
    /// Tracked gameplay stats
    pub stats: Stats,
    /// Which ending was reached, if any
    pub ending: Option<EndingType>,
    /// The game day (narrative day tracker)
    pub day: u32,
}

impl GameState {
    /// Create a new game state for a fresh game
    pub fn new(language: Language) -> Self {
        Self {
            current_node: "intro".to_string(),
            flags: HashMap::new(),
            language,
            waiting_until: None,
            message_log: Vec::new(),
            stats: Stats::default(),
            ending: None,
            day: 1,
        }
    }

    /// Check if a flag is set
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.get(flag).copied().unwrap_or(false)
    }

    /// Set a flag
    pub fn set_flag(&mut self, flag: &str) {
        self.flags.insert(flag.to_string(), true);
    }

    /// Remove a flag
    pub fn remove_flag(&mut self, flag: &str) {
        self.flags.remove(flag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_state() {
        let state = GameState::new(Language::En);
        assert_eq!(state.current_node, "intro");
        assert_eq!(state.language, Language::En);
        assert!(state.flags.is_empty());
        assert!(state.waiting_until.is_none());
        assert!(state.message_log.is_empty());
        assert_eq!(state.stats.trust_level, 3);
        assert_eq!(state.day, 1);
    }

    #[test]
    fn test_flags() {
        let mut state = GameState::new(Language::Fr);
        assert!(!state.has_flag("test_flag"));
        state.set_flag("test_flag");
        assert!(state.has_flag("test_flag"));
        state.remove_flag("test_flag");
        assert!(!state.has_flag("test_flag"));
    }

    #[test]
    fn test_stats_modify() {
        let mut stats = Stats::default();
        stats.modify("trust_level", 2);
        assert_eq!(stats.trust_level, 5);
        stats.modify("trust_level", -10);
        assert_eq!(stats.trust_level, 0); // Clamped to 0
        stats.modify("health", 100);
        assert_eq!(stats.health, 10); // Clamped to 10
    }

    #[test]
    fn test_game_state_serialization() {
        let state = GameState::new(Language::En);
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: GameState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.current_node, "intro");
        assert_eq!(deserialized.language, Language::En);
    }
}
