use std::collections::HashMap;

use crate::i18n::LocalizedString;
use crate::story::{Choice, StoryNode};

/// Build the complete story tree as a HashMap from node id -> StoryNode
pub fn build_story_tree() -> HashMap<String, StoryNode> {
    let mut nodes = HashMap::new();

    build_act1(&mut nodes);
    // Acts 2-5 will be added in subsequent commits

    // Placeholder end node (will be removed once all acts are connected)
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

// ── ACT 1 — Contact (Day 1) ─────────────────────────────────

fn build_act1(nodes: &mut HashMap<String, StoryNode>) {
    // === INTRO: First contact ===
    add_node(
        nodes,
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
                    "My name is Elara. I'm alone in what's left of the Helios Research Facility. Somewhere in the mountains, I think.",
                    "Je m'appelle Elara. J'suis seule dans ce qu'il reste du Centre de Recherche Helios. Quelque part dans les montagnes, je crois.",
                ),
                LocalizedString::new(
                    "I found this device buried under rubble in one of the labs. I've been trying to get it to work for days.",
                    "J'ai trouv\u{00e9} cet appareil sous les d\u{00e9}combres dans un des labos. \u{00c7}a fait des jours que j'essaie de le faire marcher.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Are you okay? What happened to you?",
                        "Est-ce que \u{00e7}a va ? Qu'est-ce qui t'est arriv\u{00e9} ?",
                    ),
                    next_node: "a1_response_calm".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Where exactly are you? What's this facility?",
                        "T'es o\u{00f9} exactement ? C'est quoi ce centre ?",
                    ),
                    next_node: "a1_response_direct".to_string(),
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

    // === Branch: Calm/empathetic response ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_response_calm".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I... I think so. Physically, at least. I've got some bruises, a cut on my arm that's not healing great, but I'm alive.",
                    "Je... je crois que oui. Physiquement, en tout cas. J'ai des bleus, une coupure au bras qui gu\u{00e9}rit pas super bien, mais j'suis vivante.",
                ),
                LocalizedString::new(
                    "Mentally? I honestly don't know anymore. It's been three months since everything went sideways.",
                    "Mentalement ? Honn\u{00ea}tement, j'sais plus trop. \u{00c7}a fait trois mois que tout a bascul\u{00e9}.",
                ),
                LocalizedString::new(
                    "They're calling it the Eshara. I don't even know who \"they\" are anymore. I just heard it on a broken broadcast a few weeks ago.",
                    "Ils appellent \u{00e7}a l'Eshara. J'sais m\u{00ea}me plus qui \"ils\" sont. J'ai entendu \u{00e7}a sur une fr\u{00e9}quence cass\u{00e9}e y'a quelques semaines.",
                ),
            ],
            choices: vec![],
            next_node: Some("a1_eshara_explain".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Branch: Direct/pragmatic response ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_response_direct".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Right, yeah. Helios Research Facility. It was some kind of government-funded lab. Biotech, physics, weird stuff I never fully understood.",
                    "Ouais, le Centre de Recherche Helios. C'\u{00e9}tait un genre de labo financ\u{00e9} par le gouvernement. Biotech, physique, des trucs bizarres que j'ai jamais vraiment compris.",
                ),
                LocalizedString::new(
                    "I was an engineer here. Mechanical systems, infrastructure. Not the secret science stuff.",
                    "Moi j'\u{00e9}tais ing\u{00e9}nieure ici. Syst\u{00e8}mes m\u{00e9}caniques, infrastructure. Pas les trucs scientifiques secrets.",
                ),
                LocalizedString::new(
                    "Then about three months ago... the Eshara happened. That's what people called it before the broadcasts stopped.",
                    "Et puis y'a environ trois mois... l'Eshara s'est produit. C'est comme \u{00e7}a que les gens l'appelaient avant que les \u{00e9}missions s'arr\u{00ea}tent.",
                ),
            ],
            choices: vec![],
            next_node: Some("a1_eshara_explain".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Convergence: What is the Eshara ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_eshara_explain".to_string(),
            messages: vec![
                LocalizedString::new(
                    "One day the sky just... flickered. Like a screen glitching. And then everything electronic died. All at once.",
                    "Un jour le ciel a juste... scintill\u{00e9}. Comme un \u{00e9}cran qui bug. Et puis tout ce qui \u{00e9}tait \u{00e9}lectronique est mort. D'un coup.",
                ),
                LocalizedString::new(
                    "Most people vanished. Not dead — just gone. Empty clothes left behind. Cars still running but no drivers.",
                    "La plupart des gens ont disparu. Pas morts \u{2014} juste partis. Des v\u{00ea}tements vides laiss\u{00e9}s derri\u{00e8}re. Des voitures qui tournent encore mais sans conducteurs.",
                ),
                LocalizedString::new(
                    "The ones who stayed... some are okay. Others are... different. Aggressive. Like they're not really there anymore.",
                    "Ceux qui sont rest\u{00e9}s... certains vont bien. D'autres sont... diff\u{00e9}rents. Agressifs. Comme s'ils \u{00e9}taient plus vraiment l\u{00e0}.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "That's terrifying. How have you survived this long?",
                        "C'est terrifiant. Comment t'as surv\u{00e9}cu aussi longtemps ?",
                    ),
                    next_node: "a1_survival_talk".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1), ("morale".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "What do you mean 'different'? What are they like?",
                        "Comment \u{00e7}a 'diff\u{00e9}rents' ? Ils sont comment ?",
                    ),
                    next_node: "a1_changed_ones".to_string(),
                    flags_set: vec!["asked_about_changed".to_string()],
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

    // === Branch: How she survived ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_survival_talk".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Honestly? Luck. The facility has a bunker level with emergency rations. Enough food and water for maybe another week.",
                    "Honn\u{00ea}tement ? La chance. Le centre a un niveau bunker avec des rations d'urgence. Assez de bouffe et d'eau pour peut-\u{00ea}tre encore une semaine.",
                ),
                LocalizedString::new(
                    "I've been careful. Barricaded the main doors. Only go out when I have to.",
                    "J'ai fait gaffe. J'ai barricad\u{00e9} les portes principales. Je sors que quand j'ai pas le choix.",
                ),
                LocalizedString::new(
                    "But I can't stay here forever. The rations are running low, and... there are noises at night. Things moving in the lower levels.",
                    "Mais je peux pas rester l\u{00e0} \u{00e9}ternellement. Les rations baissent, et... y'a des bruits la nuit. Des trucs qui bougent dans les niveaux inf\u{00e9}rieurs.",
                ),
            ],
            choices: vec![],
            next_node: Some("a1_decision_point".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Branch: The changed ones ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_changed_ones".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I saw a group of them about a month ago. They were just... standing in a field. Staring at the sky. Completely still.",
                    "J'ai vu un groupe y'a environ un mois. Ils \u{00e9}taient juste... debout dans un champ. \u{00c0} fixer le ciel. Compl\u{00e8}tement immobiles.",
                ),
                LocalizedString::new(
                    "When I got closer, one of them turned. Her eyes were wrong. Like static. And she made this sound... I still hear it sometimes when I try to sleep.",
                    "Quand je me suis approch\u{00e9}e, une d'entre eux s'est retourn\u{00e9}e. Ses yeux \u{00e9}taient pas normaux. Comme du gr\u{00e9}sillement. Et elle a fait ce bruit... je l'entends encore parfois quand j'essaie de dormir.",
                ),
                LocalizedString::new(
                    "I ran. I don't know if they're dangerous exactly, but something is very, very wrong with them.",
                    "J'ai couru. J'sais pas s'ils sont dangereux exactement, mais y'a un truc qui va vraiment, vraiment pas chez eux.",
                ),
            ],
            choices: vec![],
            next_node: Some("a1_decision_point".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === DECISION POINT: Explore facility or head north ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_decision_point".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Look, I need to make a decision and I... I could really use someone to think this through with.",
                    "Ecoute, faut que je prenne une d\u{00e9}cision et je... j'aurais vraiment besoin de quelqu'un pour r\u{00e9}fl\u{00e9}chir avec moi.",
                ),
                LocalizedString::new(
                    "There are two options. The facility has lower levels I haven't explored. Labs, storage rooms, maybe a server room. Could have supplies, maybe even answers about what happened. But I've heard things down there. It's risky.",
                    "Y'a deux options. Le centre a des niveaux inf\u{00e9}rieurs que j'ai pas explor\u{00e9}s. Des labos, des salles de stockage, peut-\u{00ea}tre une salle serveur. Y'a peut-\u{00ea}tre des provisions, voire des r\u{00e9}ponses sur ce qui s'est pass\u{00e9}. Mais j'entends des trucs en bas. C'est risqu\u{00e9}.",
                ),
                LocalizedString::new(
                    "Or... I picked up a faint signal from the north two days ago. It could be other survivors. A safe zone. Or it could be nothing. It's at least a day's hike through open terrain.",
                    "Ou alors... j'ai capt\u{00e9} un faible signal venant du nord y'a deux jours. \u{00c7}a pourrait \u{00ea}tre d'autres survivants. Une zone s\u{00fb}re. Ou \u{00e7}a pourrait \u{00ea}tre rien. C'est au moins une journ\u{00e9}e de marche en terrain d\u{00e9}couvert.",
                ),
                LocalizedString::new(
                    "What do you think I should do?",
                    "T'en penses quoi ?",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Explore the lower levels. You need supplies and answers.",
                        "Explore les niveaux inf\u{00e9}rieurs. T'as besoin de provisions et de r\u{00e9}ponses.",
                    ),
                    next_node: "a1_explore_facility".to_string(),
                    flags_set: vec!["explored_facility".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Head north toward the signal. People are more important than supplies.",
                        "Dirige-toi vers le nord, vers le signal. Les gens c'est plus important que les provisions.",
                    ),
                    next_node: "a1_head_north".to_string(),
                    flags_set: vec!["headed_north".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Can you do a quick search of the facility first, then head north?",
                        "Tu pourrais fouiller vite fait le centre d'abord, puis aller vers le nord ?",
                    ),
                    next_node: "a1_both_attempt".to_string(),
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

    // === BRANCH A: Explore the facility ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_explore_facility".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Yeah. Yeah, you're right. If there are answers here, I need to find them. And I need more supplies before going anywhere.",
                    "Ouais. Ouais, t'as raison. S'il y a des r\u{00e9}ponses ici, faut que je les trouve. Et j'ai besoin de plus de provisions avant d'aller o\u{00f9} que ce soit.",
                ),
                LocalizedString::new(
                    "I found a stairwell leading down. It's dark as hell. I've got a flashlight but the batteries won't last forever.",
                    "J'ai trouv\u{00e9} une cage d'escalier qui descend. C'est noir comme dans un four. J'ai une lampe torche mais les piles dureront pas \u{00e9}ternellement.",
                ),
                LocalizedString::new(
                    "Okay, I'm going down. I'll message you when I find something. Or if something finds me.",
                    "Bon, j'y vais. Je te message quand je trouve quelque chose. Ou si quelque chose me trouve.",
                ),
                LocalizedString::new(
                    "Wish me luck.",
                    "Souhaite-moi bonne chance.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Be careful down there. I'll be right here.",
                        "Fais attention l\u{00e0}-dessous. Je bouge pas d'ici.",
                    ),
                    next_node: "a1_facility_search".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Good luck. Try to find medicine if you can.",
                        "Bonne chance. Essaie de trouver des m\u{00e9}dicaments si tu peux.",
                    ),
                    next_node: "a1_facility_search".to_string(),
                    flags_set: vec!["asked_for_medicine".to_string()],
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

    // Facility search — real-time delay (5 min = 300s, 2 min for debug)
    add_node(
        nodes,
        StoryNode {
            id: "a1_facility_search".to_string(),
            messages: vec![LocalizedString::new(
                "Going dark for a bit. Signal might cut out underground.",
                "Je coupe pour un moment. Le signal va s\u{00fb}rement sauter sous terre.",
            )],
            choices: vec![],
            next_node: Some("a1_facility_result".to_string()),
            delay: Some(180), // 3 minutes
            ending: None,
            trust_refusal: None,
        },
    );

    // Facility search results
    add_node(
        nodes,
        StoryNode {
            id: "a1_facility_result".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Okay, I'm back. I'm okay. Mostly.",
                    "Bon, j'suis revenue. \u{00c7}a va. \u{00c0} peu pr\u{00e8}s.",
                ),
                LocalizedString::new(
                    "The lower level is... it's been trashed. Someone or something tore through the labs. Equipment smashed, papers everywhere.",
                    "Le niveau inf\u{00e9}rieur est... saccag\u{00e9}. Quelqu'un ou quelque chose a ravag\u{00e9} les labos. \u{00c9}quipements fracass\u{00e9}s, des papiers partout.",
                ),
                LocalizedString::new(
                    "But I found a storage room that was still sealed. Canned food, water purification tablets, and a first aid kit. Score.",
                    "Mais j'ai trouv\u{00e9} une salle de stockage encore scell\u{00e9}e. Des conserves, des pastilles de purification d'eau, et une trousse de premiers secours. Jackpot.",
                ),
            ],
            choices: vec![],
            next_node: Some("a1_facility_discovery".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a1_facility_discovery".to_string(),
            messages: vec![
                LocalizedString::new(
                    "There's something else. I found a door with a keycard reader. Still has power somehow — there must be a backup generator somewhere.",
                    "Y'a autre chose. J'ai trouv\u{00e9} une porte avec un lecteur de badge. Y'a encore du courant \u{2014} y doit y avoir un g\u{00e9}n\u{00e9}rateur de secours quelque part.",
                ),
                LocalizedString::new(
                    "The sign on the door says 'Project MIROIR'. Whatever that is. I don't have the keycard though.",
                    "Le panneau sur la porte dit \u{00ab} Projet MIROIR \u{00bb}. Quoi que ce soit. J'ai pas le badge par contre.",
                ),
                LocalizedString::new(
                    "I also found this weird thing. A notebook, handwritten. The last entry is dated the day of the Eshara. It says: 'The resonance cascade is accelerating. God help us all.'",
                    "J'ai aussi trouv\u{00e9} un truc bizarre. Un carnet, \u{00e9}crit \u{00e0} la main. La derni\u{00e8}re entr\u{00e9}e date du jour de l'Eshara. \u{00c7}a dit : \u{00ab} La cascade de r\u{00e9}sonance s'acc\u{00e9}l\u{00e8}re. Que Dieu nous aide. \u{00bb}",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Resonance cascade? This might have started right there, in that facility.",
                        "Cascade de r\u{00e9}sonance ? \u{00c7}a a peut-\u{00ea}tre commenc\u{00e9} l\u{00e0}, dans ce centre.",
                    ),
                    next_node: "a1_facility_reflect".to_string(),
                    flags_set: vec!["suspects_facility".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Don't touch anything else. Get back to safety with what you found.",
                        "Touche plus \u{00e0} rien. Remonte en s\u{00e9}curit\u{00e9} avec ce que t'as trouv\u{00e9}.",
                    ),
                    next_node: "a1_facility_reflect".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
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
        nodes,
        StoryNode {
            id: "a1_facility_reflect".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Yeah. I'm coming back up. Got enough supplies to last maybe two more weeks now. That's something.",
                    "Ouais. Je remonte. J'ai assez de provisions pour tenir peut-\u{00ea}tre encore deux semaines maintenant. C'est d\u{00e9}j\u{00e0} \u{00e7}a.",
                ),
                LocalizedString::new(
                    "I keep thinking about that notebook. 'Resonance cascade.' If they were doing something here that caused all this...",
                    "J'arr\u{00ea}te pas de penser \u{00e0} ce carnet. \u{00ab} Cascade de r\u{00e9}sonance. \u{00bb} Si c'est quelque chose qu'ils faisaient ici qui a caus\u{00e9} tout \u{00e7}a...",
                ),
                LocalizedString::new(
                    "Anyway. I need to rest. My arm hurts and I'm exhausted. I'll set up camp upstairs and message you in the morning.",
                    "Bref. Faut que je me repose. Mon bras me fait mal et j'suis crev\u{00e9}e. Je vais m'installer en haut et je te message demain matin.",
                ),
                LocalizedString::new(
                    "Thank you. For being here. Seriously.",
                    "Merci. D'\u{00ea}tre l\u{00e0}. Vraiment.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Get some rest. You've earned it. I'll be here when you wake up.",
                        "Repose-toi. Tu l'as bien m\u{00e9}rit\u{00e9}. J'serai l\u{00e0} quand tu te r\u{00e9}veilleras.",
                    ),
                    next_node: "a1_night".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1), ("morale".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Sleep well, Elara. Tomorrow we figure out the next move.",
                        "Dors bien, Elara. Demain on d\u{00e9}cide de la suite.",
                    ),
                    next_node: "a1_night".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), 1)],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === BRANCH B: Head north ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_head_north".to_string(),
            messages: vec![
                LocalizedString::new(
                    "The signal. Yeah. If there are people out there... that's worth more than anything I'd find in these ruins.",
                    "Le signal. Ouais. S'il y a des gens l\u{00e0}-bas... \u{00e7}a vaut plus que tout ce que je pourrais trouver dans ces ruines.",
                ),
                LocalizedString::new(
                    "I'll pack what I have. Some food, water, the flashlight. My jacket. It's not much.",
                    "J'vais prendre ce que j'ai. Un peu de bouffe, de l'eau, la lampe. Mon blouson. C'est pas grand-chose.",
                ),
                LocalizedString::new(
                    "The terrain north of here is mostly open. Hills, dead forests. I should be able to see anyone coming from far away. That's good and bad.",
                    "Le terrain au nord d'ici est surtout d\u{00e9}couvert. Des collines, des for\u{00ea}ts mortes. Je devrais pouvoir voir quelqu'un arriver de loin. C'est bien et pas bien \u{00e0} la fois.",
                ),
                LocalizedString::new(
                    "Okay. I'm heading out. This is either the smartest or dumbest thing I've done.",
                    "Bon. J'y vais. C'est soit le truc le plus intelligent soit le plus stupide que j'ai fait.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Stay on the high ground when you can. Easier to spot threats.",
                        "Reste en hauteur quand tu peux. Plus facile de rep\u{00e9}rer les dangers.",
                    ),
                    next_node: "a1_north_travel".to_string(),
                    flags_set: vec!["tactical_advice".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Be brave. I believe in you.",
                        "Sois courageuse. Je crois en toi.",
                    ),
                    next_node: "a1_north_travel".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), 2)],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // North travel — real-time delay
    add_node(
        nodes,
        StoryNode {
            id: "a1_north_travel".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'll check in when I can. The signal might be spotty once I'm in the open.",
                    "Je te recontacte quand je peux. Le signal sera peut-\u{00ea}tre instable une fois en terrain d\u{00e9}couvert.",
                ),
                LocalizedString::new(
                    "Here goes nothing.",
                    "C'est parti pour rien. Ou pour tout.",
                ),
            ],
            choices: vec![],
            next_node: Some("a1_north_result".to_string()),
            delay: Some(300), // 5 minutes
            ending: None,
            trust_refusal: None,
        },
    );

    // North travel result
    add_node(
        nodes,
        StoryNode {
            id: "a1_north_result".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Hey. Still here. Still walking.",
                    "H\u{00e9}. Toujours l\u{00e0}. Toujours en marche.",
                ),
                LocalizedString::new(
                    "The landscape is... it's eerie. Everything's grey. The trees are all dead but still standing, like skeletons. No birds, no insects. Just wind.",
                    "Le paysage est... c'est flippant. Tout est gris. Les arbres sont tous morts mais encore debout, comme des squelettes. Pas d'oiseaux, pas d'insectes. Juste du vent.",
                ),
                LocalizedString::new(
                    "I found a road. There's an abandoned car with a map in the glove box. The map shows a town about 15 kilometers north. That might be where the signal's coming from.",
                    "J'ai trouv\u{00e9} une route. Y'a une voiture abandonn\u{00e9}e avec une carte dans la bo\u{00ee}te \u{00e0} gants. La carte montre un village \u{00e0} environ 15 kilom\u{00e8}tres au nord. C'est peut-\u{00ea}tre de l\u{00e0} que vient le signal.",
                ),
                LocalizedString::new(
                    "It's getting dark though. I need to find shelter for the night.",
                    "Mais il commence \u{00e0} faire nuit. Faut que je trouve un abri pour la nuit.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "The car could work as shelter. Lock the doors and stay low.",
                        "La voiture pourrait servir d'abri. Verrouille les portes et reste discr\u{00e8}te.",
                    ),
                    next_node: "a1_north_shelter".to_string(),
                    flags_set: vec!["slept_in_car".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Look for a building nearby. A car isn't safe if those 'changed' people show up.",
                        "Cherche un b\u{00e2}timent \u{00e0} proximit\u{00e9}. Une voiture c'est pas s\u{00fb}r si les 'chang\u{00e9}s' d\u{00e9}barquent.",
                    ),
                    next_node: "a1_north_shelter".to_string(),
                    flags_set: vec!["found_gas_station".to_string()],
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
        nodes,
        StoryNode {
            id: "a1_north_shelter".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Okay, settled in for the night. Not exactly the Ritz, but it'll do.",
                    "Bon, install\u{00e9}e pour la nuit. C'est pas le Ritz, mais \u{00e7}a fera.",
                ),
                LocalizedString::new(
                    "I need to sleep. My legs are killing me and I haven't walked that much in... well, three months.",
                    "Faut que je dorme. J'ai les jambes en compote et j'ai pas autant march\u{00e9} depuis... bah, trois mois.",
                ),
                LocalizedString::new(
                    "Thanks for keeping me company today. It made a difference. Really.",
                    "Merci de m'avoir tenu compagnie aujourd'hui. \u{00c7}a a fait une diff\u{00e9}rence. Vraiment.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Rest up. Tomorrow we find those people.",
                        "Repose-toi. Demain on trouve ces gens.",
                    ),
                    next_node: "a1_night".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Good night, Elara. Stay safe.",
                        "Bonne nuit, Elara. Fais attention \u{00e0} toi.",
                    ),
                    next_node: "a1_night".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === BRANCH C: Try both (Elara pushes back) ===
    add_node(
        nodes,
        StoryNode {
            id: "a1_both_attempt".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I... I don't think that's realistic. If I go into the lower levels AND try to trek north the same day, I'll be exhausted and exposed at nightfall.",
                    "Je... je crois pas que ce soit r\u{00e9}aliste. Si j'explore les niveaux inf\u{00e9}rieurs ET j'essaie de marcher vers le nord le m\u{00ea}me jour, j'serai \u{00e9}puis\u{00e9}e et expos\u{00e9}e \u{00e0} la tomb\u{00e9}e de la nuit.",
                ),
                LocalizedString::new(
                    "I need to pick one. What's more important right now — information, or people?",
                    "Faut que je choisisse. Qu'est-ce qui est plus important l\u{00e0} \u{2014} les infos, ou les gens ?",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "You're right. Explore the facility — knowledge is power.",
                        "T'as raison. Explore le centre \u{2014} le savoir c'est le pouvoir.",
                    ),
                    next_node: "a1_explore_facility".to_string(),
                    flags_set: vec!["explored_facility".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "You're right. Head north — people first.",
                        "T'as raison. Va vers le nord \u{2014} les gens d'abord.",
                    ),
                    next_node: "a1_head_north".to_string(),
                    flags_set: vec!["headed_north".to_string()],
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

    // === NIGHT — End of Day 1 (converges both paths) ===
    // Overnight delay: 6 hours in real-time (debug: 5 seconds)
    add_node(
        nodes,
        StoryNode {
            id: "a1_night".to_string(),
            messages: vec![LocalizedString::new(
                "Good night. I'll message you in the morning.",
                "Bonne nuit. Je te message demain matin.",
            )],
            choices: vec![],
            next_node: Some("placeholder_end".to_string()), // Will connect to Act 2
            delay: Some(600), // 10 minutes (will be 6h in final; kept shorter for now)
            ending: None,
            trust_refusal: None,
        },
    );
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

    #[test]
    fn test_act1_node_count() {
        let tree = build_story_tree();
        // Act 1 should have at least 10 story nodes
        // (minus 1 for placeholder_end)
        assert!(
            tree.len() >= 10,
            "Expected at least 10 nodes, got {}",
            tree.len()
        );
    }

    #[test]
    fn test_act1_branch_paths() {
        let tree = build_story_tree();
        // Verify both main branches exist
        assert!(tree.contains_key("a1_explore_facility"));
        assert!(tree.contains_key("a1_head_north"));
        // Verify convergence point
        assert!(tree.contains_key("a1_night"));
    }
}
