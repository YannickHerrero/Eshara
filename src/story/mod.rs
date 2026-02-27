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
}
