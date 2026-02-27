pub mod endings;
pub mod nodes;

use serde::{Deserialize, Serialize};

use crate::i18n::LocalizedString;

/// A condition that must be met for a choice to be available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// A flag must be set
    FlagSet(String),
    /// A flag must NOT be set
    FlagUnset(String),
    /// A stat must be >= a threshold
    StatAtLeast(String, i32),
    /// A stat must be < a threshold
    StatBelow(String, i32),
}

/// A player choice within a story node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// Localized display text for this choice
    pub label: LocalizedString,
    /// The node id to jump to when this choice is selected
    pub next_node: String,
    /// Optional flags to set when this choice is made
    #[serde(default)]
    pub flags_set: Vec<String>,
    /// Optional flags to remove when this choice is made
    #[serde(default)]
    pub flags_remove: Vec<String>,
    /// Optional stat changes (stat_name, delta) applied when chosen
    #[serde(default)]
    pub stat_changes: Vec<(String, i32)>,
    /// Optional conditions required for this choice to be visible
    #[serde(default)]
    pub conditions: Vec<Condition>,
}

/// Descriptor for a trust-based refusal: if trust is too low, Elara ignores the choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRefusal {
    /// Minimum trust_level required to obey the player's choice
    pub min_trust: i32,
    /// The node Elara goes to instead if she refuses
    pub refusal_node: String,
    /// Localized refusal message from Elara
    pub refusal_message: LocalizedString,
}

/// A single story node in the narrative tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNode {
    /// Unique identifier for this node
    pub id: String,
    /// Ordered list of Elara's messages at this node
    pub messages: Vec<LocalizedString>,
    /// Optional player choices (if empty, this is a linear node)
    #[serde(default)]
    pub choices: Vec<Choice>,
    /// For linear nodes: the next node to auto-advance to
    pub next_node: Option<String>,
    /// Optional real-time delay in seconds before the next node triggers
    pub delay: Option<u64>,
    /// If this node is an ending, describes which ending
    pub ending: Option<EndingType>,
    /// Optional trust-based refusal configuration for this node
    pub trust_refusal: Option<TrustRefusal>,
}

/// The type of ending reached
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndingType {
    NewDawn,
    TheSignal,
    Static,
    GoneDark,
    TheEsharaWins,
}

// ── Condition evaluation ─────────────────────────────────────

use crate::game::GameState;

impl Condition {
    /// Evaluate whether this condition is met given the current game state
    pub fn evaluate(&self, state: &GameState) -> bool {
        match self {
            Condition::FlagSet(flag) => state.has_flag(flag),
            Condition::FlagUnset(flag) => !state.has_flag(flag),
            Condition::StatAtLeast(stat, threshold) => {
                state.stats.get(stat).unwrap_or(0) >= *threshold
            }
            Condition::StatBelow(stat, threshold) => {
                state.stats.get(stat).unwrap_or(0) < *threshold
            }
        }
    }
}

impl Choice {
    /// Check if all conditions for this choice are met
    pub fn is_available(&self, state: &GameState) -> bool {
        self.conditions.iter().all(|c| c.evaluate(state))
    }
}

impl StoryNode {
    /// Get only the choices that are available given the current game state
    pub fn available_choices(&self, state: &GameState) -> Vec<(usize, &Choice)> {
        self.choices
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_available(state))
            .collect()
    }

    /// Check if this node has a trust refusal that should trigger
    pub fn should_refuse(&self, state: &GameState) -> bool {
        if let Some(ref refusal) = self.trust_refusal {
            state.stats.trust_level < refusal.min_trust
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_node_creation() {
        let node = StoryNode {
            id: "test_node".to_string(),
            messages: vec![LocalizedString::new("Hello", "Bonjour")],
            choices: vec![],
            next_node: Some("next".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        };
        assert_eq!(node.id, "test_node");
        assert_eq!(node.messages.len(), 1);
        assert!(node.choices.is_empty());
    }

    #[test]
    fn test_choice_with_conditions() {
        let choice = Choice {
            label: LocalizedString::new("Go left", "Aller \u{00e0} gauche"),
            next_node: "left_path".to_string(),
            flags_set: vec!["went_left".to_string()],
            flags_remove: vec![],
            stat_changes: vec![("trust_level".to_string(), 1)],
            conditions: vec![Condition::FlagSet("has_map".to_string())],
        };
        assert_eq!(choice.next_node, "left_path");
        assert_eq!(choice.flags_set.len(), 1);
        assert_eq!(choice.conditions.len(), 1);
    }

    #[test]
    fn test_ending_type_serialization() {
        let ending = EndingType::NewDawn;
        let json = serde_json::to_string(&ending).unwrap();
        let deserialized: EndingType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, EndingType::NewDawn);
    }

    #[test]
    fn test_condition_flag_set() {
        use crate::i18n::Language;
        let mut state = GameState::new(Language::En);
        let cond = Condition::FlagSet("test_flag".to_string());
        assert!(!cond.evaluate(&state));
        state.set_flag("test_flag");
        assert!(cond.evaluate(&state));
    }

    #[test]
    fn test_condition_flag_unset() {
        use crate::i18n::Language;
        let mut state = GameState::new(Language::En);
        let cond = Condition::FlagUnset("test_flag".to_string());
        assert!(cond.evaluate(&state));
        state.set_flag("test_flag");
        assert!(!cond.evaluate(&state));
    }

    #[test]
    fn test_condition_stat_at_least() {
        use crate::i18n::Language;
        let state = GameState::new(Language::En);
        // Default trust_level is 3
        let cond = Condition::StatAtLeast("trust_level".to_string(), 3);
        assert!(cond.evaluate(&state));
        let cond_high = Condition::StatAtLeast("trust_level".to_string(), 5);
        assert!(!cond_high.evaluate(&state));
    }

    #[test]
    fn test_choice_availability() {
        use crate::i18n::Language;
        let state = GameState::new(Language::En);
        let choice = Choice {
            label: LocalizedString::new("Gated option", "Option conditionn\u{00e9}e"),
            next_node: "next".to_string(),
            flags_set: vec![],
            flags_remove: vec![],
            stat_changes: vec![],
            conditions: vec![Condition::FlagSet("required_flag".to_string())],
        };
        assert!(!choice.is_available(&state));

        let mut state2 = state.clone();
        state2.set_flag("required_flag");
        assert!(choice.is_available(&state2));
    }

    #[test]
    fn test_trust_refusal() {
        use crate::i18n::Language;
        let state = GameState::new(Language::En); // trust_level = 3
        let node = StoryNode {
            id: "test".to_string(),
            messages: vec![],
            choices: vec![],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: Some(TrustRefusal {
                min_trust: 5,
                refusal_node: "refusal".to_string(),
                refusal_message: LocalizedString::new(
                    "Sorry, I can't do that.",
                    "D\u{00e9}sol\u{00e9}e, je peux pas faire \u{00e7}a.",
                ),
            }),
        };
        assert!(node.should_refuse(&state)); // trust is 3, min is 5

        let mut state2 = state.clone();
        state2.stats.trust_level = 6;
        assert!(!node.should_refuse(&state2)); // trust is 6, min is 5
    }

    #[test]
    fn test_available_choices_filters_correctly() {
        use crate::i18n::Language;
        let mut state = GameState::new(Language::En);
        let node = StoryNode {
            id: "test".to_string(),
            messages: vec![],
            choices: vec![
                Choice {
                    label: LocalizedString::new("Always visible", "Toujours visible"),
                    next_node: "a".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new("Needs flag", "Besoin du flag"),
                    next_node: "b".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![Condition::FlagSet("special".to_string())],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        };

        let available = node.available_choices(&state);
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].0, 0);

        state.set_flag("special");
        let available = node.available_choices(&state);
        assert_eq!(available.len(), 2);
    }
}
