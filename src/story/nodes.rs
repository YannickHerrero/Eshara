use std::collections::HashMap;

use crate::i18n::LocalizedString;
use crate::story::{Choice, StoryNode};

/// Build the complete story tree as a HashMap from node id -> StoryNode
pub fn build_story_tree() -> HashMap<String, StoryNode> {
    let mut nodes = HashMap::new();

    // Placeholder intro node for engine testing — will be replaced by Act 1
    add_node(
        &mut nodes,
        StoryNode {
            id: "intro".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Hello? Is... is anyone there?",
                    "All\u{00f4} ? Est-ce que... y'a quelqu'un ?",
                ),
                LocalizedString::new(
                    "Oh god, it works. The radio actually works.",
                    "Oh mon dieu, \u{00e7}a marche. La radio marche vraiment.",
                ),
                LocalizedString::new(
                    "My name is Elara. I'm... I'm alone. I found this device in the ruins of some research facility.",
                    "Je m'appelle Elara. Je suis... je suis seule. J'ai trouv\u{00e9} cet appareil dans les ruines d'un labo de recherche.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Are you okay? What happened?",
                        "Est-ce que \u{00e7}a va ? Qu'est-ce qui s'est pass\u{00e9} ?",
                    ),
                    next_node: "intro_response_calm".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Who are you? Where are you?",
                        "T'es qui ? T'es o\u{00f9} ?",
                    ),
                    next_node: "intro_response_direct".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        &mut nodes,
        StoryNode {
            id: "intro_response_calm".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I... I think so. Physically, at least.",
                    "Je... je crois que oui. Physiquement, en tout cas.",
                ),
                LocalizedString::new(
                    "I don't really know what happened. Three months ago, everything just... stopped. They're calling it the Eshara.",
                    "J'sais pas trop ce qui s'est pass\u{00e9}. Y'a trois mois, tout s'est juste... arr\u{00ea}t\u{00e9}. Ils appellent \u{00e7}a l'Eshara.",
                ),
            ],
            choices: vec![],
            next_node: Some("intro_continue".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        &mut nodes,
        StoryNode {
            id: "intro_response_direct".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Right, sorry. I'm Elara. I'm in what's left of the Helios Research Facility, somewhere in the mountains.",
                    "Ouais, pardon. Moi c'est Elara. J'suis dans ce qu'il reste du Centre de Recherche Helios, quelque part dans les montagnes.",
                ),
                LocalizedString::new(
                    "Something happened three months ago. They call it the Eshara. Most people are... gone.",
                    "Y'a un truc qui s'est pass\u{00e9} y'a trois mois. Ils appellent \u{00e7}a l'Eshara. La plupart des gens ont... disparu.",
                ),
            ],
            choices: vec![],
            next_node: Some("intro_continue".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        &mut nodes,
        StoryNode {
            id: "intro_continue".to_string(),
            messages: vec![LocalizedString::new(
                "Look, I know this is a lot. But you're the first person I've been able to reach. Please don't go.",
                "Je sais que c'est beaucoup d'un coup. Mais t'es la premi\u{00e8}re personne que j'arrive \u{00e0} joindre. S'il te pla\u{00ee}t, pars pas.",
            )],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "I'm not going anywhere. What do you need?",
                        "Je vais nulle part. De quoi t'as besoin ?",
                    ),
                    next_node: "placeholder_end".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "I'll try to help. Tell me what's going on.",
                        "Je vais essayer de t'aider. Dis-moi ce qui se passe.",
                    ),
                    next_node: "placeholder_end".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // Temporary placeholder ending node — will be replaced by real story
    add_node(
        &mut nodes,
        StoryNode {
            id: "placeholder_end".to_string(),
            messages: vec![LocalizedString::new(
                "[Story continues in future updates...]",
                "[La suite arrive bient\u{00f4}t...]",
            )],
            choices: vec![],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    nodes
}

fn add_node(nodes: &mut HashMap<String, StoryNode>, node: StoryNode) {
    nodes.insert(node.id.clone(), node);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_tree_has_intro() {
        let tree = build_story_tree();
        assert!(tree.contains_key("intro"));
    }

    #[test]
    fn test_all_next_nodes_exist() {
        let tree = build_story_tree();
        for (id, node) in &tree {
            if let Some(ref next) = node.next_node {
                assert!(
                    tree.contains_key(next),
                    "Node '{}' references next_node '{}' which doesn't exist",
                    id,
                    next
                );
            }
            for choice in &node.choices {
                assert!(
                    tree.contains_key(&choice.next_node),
                    "Node '{}' has choice pointing to '{}' which doesn't exist",
                    id,
                    choice.next_node
                );
            }
        }
    }
}
