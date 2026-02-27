use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::i18n::Language;

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
    pub trust: i32,
    pub health: i32,
    pub supplies: i32,
}

impl Stats {
    /// Create stats with specified initial values
    pub fn new(trust: i32, health: i32, supplies: i32) -> Self {
        Self {
            trust,
            health,
            supplies,
        }
    }

    /// Get a stat value by name
    pub fn get(&self, name: &str) -> Option<i32> {
        match name {
            "trust" => Some(self.trust),
            "health" => Some(self.health),
            "supplies" => Some(self.supplies),
            _ => None,
        }
    }

    /// Modify a stat by name with a delta (clamped to 0..=10)
    pub fn modify(&mut self, name: &str, delta: i32) {
        match name {
            "trust" => self.trust = (self.trust + delta).max(0).min(10),
            "health" => self.health = (self.health + delta).max(0).min(10),
            "supplies" => self.supplies = (self.supplies + delta).max(0).min(10),
            _ => {}
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            trust: 3,
            health: 10,
            supplies: 3,
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
    /// Which ending was reached, if any (string key e.g. "still_here", "gone_dark")
    pub ending: Option<String>,
    /// The game day (narrative day tracker)
    pub day: u32,
}

impl GameState {
    /// Create a new game state for a fresh game
    pub fn new(
        language: Language,
        start_node: &str,
        trust: i32,
        health: i32,
        supplies: i32,
    ) -> Self {
        Self {
            current_node: start_node.to_string(),
            flags: HashMap::new(),
            language,
            waiting_until: None,
            message_log: Vec::new(),
            stats: Stats::new(trust, health, supplies),
            ending: None,
            day: 1,
        }
    }

    /// Create a new game state initialized from StoryData
    pub fn from_story(language: Language, story: &crate::story::StoryData) -> Self {
        let trust = story.stats.get("trust").map(|s| s.initial).unwrap_or(3);
        let health = story.stats.get("health").map(|s| s.initial).unwrap_or(10);
        let supplies = story.stats.get("supplies").map(|s| s.initial).unwrap_or(3);
        Self::new(language, &story.meta.start_node, trust, health, supplies)
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

// ── Save / Load ──────────────────────────────────────────────

/// Get the path to the save directory (~/.eshara/)
pub fn save_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".eshara")
}

/// Get the path to the save file (~/.eshara/save.json)
pub fn save_path() -> PathBuf {
    save_dir().join("save.json")
}

/// Save the game state to disk
pub fn save_game(state: &GameState) -> io::Result<()> {
    let dir = save_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    let json =
        serde_json::to_string_pretty(state).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(save_path(), json)
}

/// Load the game state from disk, if a save file exists
pub fn load_game() -> io::Result<Option<GameState>> {
    let path = save_path();
    if !path.exists() {
        return Ok(None);
    }
    let json = fs::read_to_string(path)?;
    let state: GameState =
        serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Some(state))
}

/// Delete the save file
pub fn delete_save() -> io::Result<()> {
    let path = save_path();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Check if a save file exists
pub fn save_exists() -> bool {
    save_path().exists()
}

// ── CLI argument parsing ─────────────────────────────────────

/// Parsed command-line arguments
pub struct CliArgs {
    /// If true, delete save and exit
    pub reset: bool,
    /// Optional language override
    pub language: Option<Language>,
}

/// Parse command-line arguments (minimal, no dependency)
pub fn parse_cli_args() -> CliArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut reset = false;
    let mut language = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--reset" => reset = true,
            "--lang" => {
                if i + 1 < args.len() {
                    language = crate::i18n::parse_language(&args[i + 1]);
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    CliArgs { reset, language }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_new_game_state() {
        let state = GameState::new(Language::En, "a1_first_contact", 3, 10, 3);
        assert_eq!(state.current_node, "a1_first_contact");
        assert_eq!(state.language, Language::En);
        assert!(state.flags.is_empty());
        assert!(state.waiting_until.is_none());
        assert!(state.message_log.is_empty());
        assert_eq!(state.stats.trust, 3);
        assert_eq!(state.stats.health, 10);
        assert_eq!(state.stats.supplies, 3);
        assert_eq!(state.day, 1);
    }

    #[test]
    fn test_flags() {
        let mut state = GameState::new(Language::Fr, "test", 3, 10, 3);
        assert!(!state.has_flag("test_flag"));
        state.set_flag("test_flag");
        assert!(state.has_flag("test_flag"));
        state.remove_flag("test_flag");
        assert!(!state.has_flag("test_flag"));
    }

    #[test]
    fn test_stats_modify() {
        let mut stats = Stats::default();
        stats.modify("trust", 2);
        assert_eq!(stats.trust, 5);
        stats.modify("trust", -10);
        assert_eq!(stats.trust, 0); // Clamped to 0
        stats.modify("health", 100);
        assert_eq!(stats.health, 10); // Clamped to 10
    }

    #[test]
    fn test_game_state_serialization() {
        let state = GameState::new(Language::En, "a1_first_contact", 3, 10, 3);
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: GameState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.current_node, "a1_first_contact");
        assert_eq!(deserialized.language, Language::En);
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let tmp = std::env::temp_dir().join("eshara_test_save");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        let save_file = tmp.join("save.json");

        let state = GameState::new(Language::Fr, "a1_first_contact", 3, 10, 3);
        let json = serde_json::to_string_pretty(&state).unwrap();
        fs::write(&save_file, &json).unwrap();

        let loaded_json = fs::read_to_string(&save_file).unwrap();
        let loaded: GameState = serde_json::from_str(&loaded_json).unwrap();
        assert_eq!(loaded.current_node, "a1_first_contact");
        assert_eq!(loaded.language, Language::Fr);

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_save_dir_path() {
        let dir = save_dir();
        assert!(dir.to_string_lossy().contains(".eshara"));
    }
}
