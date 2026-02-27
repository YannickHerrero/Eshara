use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::i18n::LocalizedString;

/// The default story JSON, embedded at compile time from data/story.json.
const EMBEDDED_STORY: &str = include_str!("../../data/story.json");

// ── Top-level story data ─────────────────────────────────────

/// Story metadata (title, version, configuration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryMeta {
    pub title: String,
    pub version: String,
    pub start_node: String,
    #[serde(default = "default_typing_delay")]
    pub default_typing_delay_ms: u64,
    #[serde(default = "default_debug_delay")]
    pub debug_delay_override_seconds: u64,
}

fn default_typing_delay() -> u64 {
    60
}
fn default_debug_delay() -> u64 {
    5
}

/// Definition of a tracked stat (initial value, bounds)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatDef {
    pub initial: i32,
    pub min: i32,
    pub max: i32,
    #[serde(default)]
    pub description: String,
}

/// Ending condition hints (stored in JSON for documentation; evaluated at runtime via branch)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndingConditions {
    #[serde(default)]
    pub min_trust: Option<i32>,
    #[serde(default)]
    pub max_trust: Option<i32>,
    #[serde(default)]
    pub min_health: Option<i32>,
    #[serde(default)]
    pub health_equals: Option<i32>,
    #[serde(default)]
    pub flags_required: Vec<String>,
}

/// Localized ending metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndingInfo {
    pub title: LocalizedString,
    #[serde(rename = "type", default)]
    pub ending_type: String,
    #[serde(default)]
    pub conditions: Option<EndingConditions>,
}

/// Global death check rule: if health reaches 0, route to a specific ending
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathCheck {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub trigger: String,
    #[serde(default)]
    pub condition: Option<serde_json::Value>,
    pub override_next_node: String,
}

/// Top-level story data loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryData {
    pub meta: StoryMeta,
    /// Stat definitions keyed by stat name (e.g. "trust", "health", "supplies")
    #[serde(default)]
    pub stats: HashMap<String, StatDef>,
    /// Flag documentation keyed by flag name
    #[serde(default)]
    pub flags: HashMap<String, String>,
    /// Ending metadata keyed by ending key (e.g. "still_here", "gone_dark")
    #[serde(default)]
    pub endings: HashMap<String, EndingInfo>,
    /// All story nodes keyed by their unique id
    pub nodes: HashMap<String, StoryNode>,
    /// Global death check rule
    #[serde(default)]
    pub death_check: Option<DeathCheck>,
}

impl StoryData {
    /// Look up ending info by string key
    pub fn ending_info(&self, key: &str) -> Option<&EndingInfo> {
        self.endings.get(key)
    }
}

// ── Node types ───────────────────────────────────────────────

/// Effects applied when entering a node or choosing an option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Effects {
    #[serde(default)]
    pub trust_change: Option<i32>,
    #[serde(default)]
    pub health_change: Option<i32>,
    #[serde(default)]
    pub supplies_change: Option<i32>,
    #[serde(default)]
    pub flags_set: Vec<String>,
    #[serde(default)]
    pub flags_remove: Vec<String>,
    /// Conditional medicine (ignored in gameplay — handled by flags)
    #[serde(default)]
    pub has_medicine_conditional: Option<bool>,
}

impl Effects {
    /// Apply stat changes and flag modifications to the game state.
    /// Returns true if health was changed (for death check).
    pub fn apply(&self, state: &mut crate::game::GameState) -> bool {
        let mut health_changed = false;
        if let Some(delta) = self.trust_change {
            state.stats.modify("trust", delta);
        }
        if let Some(delta) = self.health_change {
            state.stats.modify("health", delta);
            health_changed = true;
        }
        if let Some(delta) = self.supplies_change {
            state.stats.modify("supplies", delta);
        }
        for flag in &self.flags_set {
            state.set_flag(flag);
        }
        for flag in &self.flags_remove {
            state.remove_flag(flag);
        }
        health_changed
    }
}

/// Real-time delay with a localized waiting message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayInfo {
    pub seconds: u64,
    pub message: LocalizedString,
}

/// A condition for conditional branching
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchCondition {
    #[serde(default)]
    pub flags_required: Vec<String>,
    #[serde(default)]
    pub min_trust: Option<i32>,
    #[serde(default)]
    pub max_trust: Option<i32>,
    #[serde(default)]
    pub min_health: Option<i32>,
    #[serde(default)]
    pub max_health: Option<i32>,
    /// If true, this is the fallback/default branch
    #[serde(default)]
    pub default: bool,
}

impl BranchCondition {
    /// Evaluate whether this branch condition is met
    pub fn evaluate(&self, state: &crate::game::GameState) -> bool {
        if self.default {
            return true;
        }

        // Check required flags
        for flag in &self.flags_required {
            if !state.has_flag(flag) {
                return false;
            }
        }

        // Check stat thresholds
        if let Some(min) = self.min_trust {
            if state.stats.trust < min {
                return false;
            }
        }
        if let Some(max) = self.max_trust {
            if state.stats.trust > max {
                return false;
            }
        }
        if let Some(min) = self.min_health {
            if state.stats.health < min {
                return false;
            }
        }
        if let Some(max) = self.max_health {
            if state.stats.health > max {
                return false;
            }
        }

        true
    }
}

/// A conditional branch entry (evaluated in order; first match wins)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub condition: BranchCondition,
    pub next_node: String,
}

/// A player choice within a story node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// Localized display text for this choice
    pub label: LocalizedString,
    /// The node id to jump to when this choice is selected
    pub next_node: String,
    /// Effects applied when this choice is made
    #[serde(default)]
    pub on_choose: Option<Effects>,
}

/// A single story node in the narrative tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNode {
    /// Unique identifier for this node
    pub id: String,
    /// Act number (informational)
    #[serde(default)]
    pub act: Option<u32>,
    /// Human-readable title (informational)
    #[serde(default)]
    pub title: Option<String>,
    /// Ordered list of messages at this node
    #[serde(default)]
    pub messages: Vec<LocalizedString>,
    /// Player choices (null/absent = no choices)
    pub choices: Option<Vec<Choice>>,
    /// For linear nodes: the next node to auto-advance to
    pub next_node: Option<String>,
    /// Optional real-time delay before the next node triggers
    pub delay: Option<DelayInfo>,
    /// If this node is an ending, the ending key (e.g. "still_here", "gone_dark")
    pub ending: Option<String>,
    /// Effects applied when entering this node
    #[serde(default)]
    pub on_enter: Option<Effects>,
    /// Conditional branching (evaluated in order; first match wins)
    #[serde(default)]
    pub branch: Option<Vec<Branch>>,
}

// ── Story loading ────────────────────────────────────────────

/// Load the story data.
///
/// 1. If `data/story.json` exists on disk (next to the working directory), load it.
/// 2. Otherwise, fall back to the compile-time embedded copy.
///
/// Panics if the JSON is malformed or the story graph is invalid.
pub fn load_story() -> StoryData {
    let story_data: StoryData = {
        let external = Path::new("data/story.json");
        if external.exists() {
            let json = std::fs::read_to_string(external).expect("Failed to read data/story.json");
            serde_json::from_str(&json).expect("Failed to parse data/story.json")
        } else {
            serde_json::from_str(EMBEDDED_STORY).expect("Failed to parse embedded story data")
        }
    };

    let errors = story_data.validate();
    if !errors.is_empty() {
        eprintln!("Story validation errors:");
        for e in &errors {
            eprintln!("  - {}", e);
        }
        panic!(
            "Story data has {} validation error(s). Fix data/story.json and try again.",
            errors.len()
        );
    }

    story_data
}

// ── Validation ───────────────────────────────────────────────

impl StoryData {
    /// Validate the story graph for structural integrity.
    /// Returns a list of errors (empty = valid).
    pub fn validate(&self) -> Vec<String> {
        use std::collections::{HashSet, VecDeque};

        let mut errors = Vec::new();
        let start = &self.meta.start_node;

        // 1. Must have the start node
        if !self.nodes.contains_key(start) {
            errors.push(format!("Missing required start node '{}'", start));
            return errors;
        }

        // 2. All referenced nodes must exist
        for (id, node) in &self.nodes {
            if let Some(ref next) = node.next_node {
                if !self.nodes.contains_key(next) {
                    errors.push(format!(
                        "Node '{}' references next_node '{}' which doesn't exist",
                        id, next
                    ));
                }
            }
            if let Some(ref choices) = node.choices {
                for choice in choices {
                    if !self.nodes.contains_key(&choice.next_node) {
                        errors.push(format!(
                            "Node '{}' has choice pointing to '{}' which doesn't exist",
                            id, choice.next_node
                        ));
                    }
                }
            }
            if let Some(ref branches) = node.branch {
                for branch in branches {
                    if !self.nodes.contains_key(&branch.next_node) {
                        errors.push(format!(
                            "Node '{}' has branch pointing to '{}' which doesn't exist",
                            id, branch.next_node
                        ));
                    }
                }
            }
        }

        // 3. No dead ends
        for (id, node) in &self.nodes {
            let has_next = node.next_node.is_some();
            let has_choices = node.choices.as_ref().is_some_and(|c| !c.is_empty());
            let has_ending = node.ending.is_some();
            let has_branch = node.branch.as_ref().is_some_and(|b| !b.is_empty());

            if !has_next && !has_choices && !has_ending && !has_branch {
                errors.push(format!(
                    "Dead-end node '{}': no choices, no next_node, no ending, no branch",
                    id
                ));
            }
        }

        // 4. All nodes reachable from start
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());

        while let Some(id) = queue.pop_front() {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id.clone());

            if let Some(node) = self.nodes.get(&id) {
                if let Some(ref next) = node.next_node {
                    queue.push_back(next.clone());
                }
                if let Some(ref choices) = node.choices {
                    for choice in choices {
                        queue.push_back(choice.next_node.clone());
                    }
                }
                if let Some(ref branches) = node.branch {
                    for branch in branches {
                        queue.push_back(branch.next_node.clone());
                    }
                }
            }
        }

        // Also add the death check target as reachable
        if let Some(ref dc) = self.death_check {
            visited.insert(dc.override_next_node.clone());
        }

        let unreachable: Vec<_> = self
            .nodes
            .keys()
            .filter(|k| !visited.contains(*k))
            .collect();
        if !unreachable.is_empty() {
            errors.push(format!("Unreachable nodes: {:?}", unreachable));
        }

        // 5. At least one ending node exists
        let ending_count = self.nodes.values().filter(|n| n.ending.is_some()).count();
        if ending_count == 0 {
            errors.push("No ending nodes found in the story".to_string());
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_json_parses() {
        let story_data: StoryData =
            serde_json::from_str(EMBEDDED_STORY).expect("Embedded JSON should parse");
        assert!(!story_data.nodes.is_empty());
        assert!(!story_data.endings.is_empty());
    }

    #[test]
    fn test_embedded_json_validates() {
        let story_data: StoryData = serde_json::from_str(EMBEDDED_STORY).unwrap();
        let errors = story_data.validate();
        assert!(
            errors.is_empty(),
            "Embedded story has validation errors: {:?}",
            errors
        );
    }

    #[test]
    fn test_embedded_json_has_all_endings() {
        let story_data: StoryData = serde_json::from_str(EMBEDDED_STORY).unwrap();
        assert_eq!(story_data.endings.len(), 5, "Expected 5 endings");
        assert!(story_data.ending_info("still_here").is_some());
        assert!(story_data.ending_info("let_go").is_some());
        assert!(story_data.ending_info("static").is_some());
        assert!(story_data.ending_info("gone_dark").is_some());
        assert!(story_data.ending_info("echo").is_some());
    }

    #[test]
    fn test_embedded_json_meta() {
        let story_data: StoryData = serde_json::from_str(EMBEDDED_STORY).unwrap();
        assert_eq!(story_data.meta.start_node, "a1_first_contact");
        assert_eq!(story_data.meta.title, "Eshara");
    }

    #[test]
    fn test_embedded_json_stats() {
        let story_data: StoryData = serde_json::from_str(EMBEDDED_STORY).unwrap();
        assert!(story_data.stats.contains_key("trust"));
        assert!(story_data.stats.contains_key("health"));
        assert!(story_data.stats.contains_key("supplies"));
        assert_eq!(story_data.stats["trust"].initial, 3);
        assert_eq!(story_data.stats["health"].initial, 10);
    }

    #[test]
    fn test_embedded_json_has_death_check() {
        let story_data: StoryData = serde_json::from_str(EMBEDDED_STORY).unwrap();
        assert!(story_data.death_check.is_some());
        assert_eq!(
            story_data.death_check.unwrap().override_next_node,
            "ending_gone_dark"
        );
    }

    #[test]
    fn test_branch_condition_default() {
        let cond = BranchCondition {
            default: true,
            ..Default::default()
        };
        let state = crate::game::GameState::new(crate::i18n::Language::En, "test", 3, 10, 3);
        assert!(cond.evaluate(&state));
    }

    #[test]
    fn test_branch_condition_flags() {
        let cond = BranchCondition {
            flags_required: vec!["has_shielding".to_string()],
            ..Default::default()
        };
        let mut state = crate::game::GameState::new(crate::i18n::Language::En, "test", 3, 10, 3);
        assert!(!cond.evaluate(&state));
        state.set_flag("has_shielding");
        assert!(cond.evaluate(&state));
    }

    #[test]
    fn test_branch_condition_trust() {
        let cond = BranchCondition {
            min_trust: Some(7),
            ..Default::default()
        };
        let mut state = crate::game::GameState::new(crate::i18n::Language::En, "test", 3, 10, 3);
        assert!(!cond.evaluate(&state)); // trust is 3
        state.stats.trust = 8;
        assert!(cond.evaluate(&state)); // trust is 8
    }

    #[test]
    fn test_effects_apply() {
        let effects = Effects {
            trust_change: Some(2),
            health_change: Some(-1),
            supplies_change: None,
            flags_set: vec!["test_flag".to_string()],
            flags_remove: vec![],
            has_medicine_conditional: None,
        };
        let mut state = crate::game::GameState::new(crate::i18n::Language::En, "test", 3, 10, 3);
        let health_changed = effects.apply(&mut state);
        assert!(health_changed);
        assert_eq!(state.stats.trust, 5);
        assert_eq!(state.stats.health, 9);
        assert!(state.has_flag("test_flag"));
    }
}
