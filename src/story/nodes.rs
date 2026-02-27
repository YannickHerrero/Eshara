use std::collections::HashMap;

use crate::i18n::LocalizedString;
use crate::story::{Choice, Condition, StoryNode, TrustRefusal};

/// Build the complete story tree as a HashMap from node id -> StoryNode
pub fn build_story_tree() -> HashMap<String, StoryNode> {
    let mut nodes = HashMap::new();

    build_act1(&mut nodes);
    build_act2(&mut nodes);
    // Acts 3-5 will be added in subsequent commits

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
    // Overnight delay
    add_node(
        nodes,
        StoryNode {
            id: "a1_night".to_string(),
            messages: vec![LocalizedString::new(
                "Good night. I'll message you in the morning.",
                "Bonne nuit. Je te message demain matin.",
            )],
            choices: vec![],
            next_node: Some("a2_morning".to_string()),
            delay: Some(600), // 10 minutes
            ending: None,
            trust_refusal: None,
        },
    );
}

// ── ACT 2 — The Journey (Days 2-4) ───────────────────────────

fn build_act2(nodes: &mut HashMap<String, StoryNode>) {
    // === DAY 2 MORNING ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_morning".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Morning. I think. Hard to tell when the sky always looks like a bruise.",
                    "Bonjour. Je crois. Dur \u{00e0} dire quand le ciel a toujours l'air d'un bleu.",
                ),
                LocalizedString::new(
                    "Slept badly. Kept hearing this low hum, like something vibrating underground. Might just be my imagination.",
                    "J'ai mal dormi. J'arr\u{00ea}tais pas d'entendre ce bourdonnement sourd, comme un truc qui vibre sous terre. C'est peut-\u{00ea}tre mon imagination.",
                ),
                LocalizedString::new(
                    "Anyway. New day. Time to move.",
                    "Bref. Nouveau jour. Faut bouger.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_morning_choice".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_morning_choice".to_string(),
            messages: vec![LocalizedString::new(
                "I need to figure out my next move. I can see smoke in the distance to the northwest. Someone's got a fire going. And that signal from the north is still faintly pulsing.",
                "Faut que je d\u{00e9}cide de la suite. Je vois de la fum\u{00e9}e au loin vers le nord-ouest. Quelqu'un a allum\u{00e9} un feu. Et le signal du nord pulse toujours faiblement.",
            )],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Head toward the smoke. Where there's fire, there are people.",
                        "Va vers la fum\u{00e9}e. L\u{00e0} o\u{00f9} y'a du feu, y'a des gens.",
                    ),
                    next_node: "a2_toward_smoke".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Keep following the signal north. Stay focused on the original plan.",
                        "Continue \u{00e0} suivre le signal vers le nord. Reste concentr\u{00e9}e sur le plan initial.",
                    ),
                    next_node: "a2_continue_north".to_string(),
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

    // === BRANCH: Toward the smoke (meet Kai) ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_toward_smoke".to_string(),
            messages: vec![
                LocalizedString::new(
                    "The smoke's coming from behind a ridge. Walking toward it now.",
                    "La fum\u{00e9}e vient de derri\u{00e8}re une cr\u{00ea}te. J'avance vers elle.",
                ),
                LocalizedString::new(
                    "I can see a small camp. One tent, a fire. And... there's someone there. A man. He's seen me too.",
                    "Je vois un petit campement. Une tente, un feu. Et... y'a quelqu'un. Un homme. Il m'a vue aussi.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_meet_kai".to_string()),
            delay: Some(120), // 2 min travel
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_meet_kai".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Okay, I talked to him. His name is Kai. He's maybe early thirties, looks like he hasn't slept in weeks. Says he was a teacher before the Eshara.",
                    "Bon, je lui ai parl\u{00e9}. Il s'appelle Ka\u{00ef}. La trentaine, on dirait qu'il a pas dormi depuis des semaines. Il dit qu'il \u{00e9}tait prof avant l'Eshara.",
                ),
                LocalizedString::new(
                    "He's been surviving alone too. Moving camp every few days to avoid the changed ones. He says they're getting more... organized.",
                    "Lui aussi survit seul. Il d\u{00e9}place son camp tous les quelques jours pour \u{00e9}viter les chang\u{00e9}s. Il dit qu'ils deviennent plus... organis\u{00e9}s.",
                ),
                LocalizedString::new(
                    "He's heard the northern signal too. Says he was heading that way but got spooked by something he saw near the river.",
                    "Il a entendu le signal du nord aussi. Il dit qu'il allait dans cette direction mais qu'un truc qu'il a vu pr\u{00e8}s de la rivi\u{00e8}re l'a fait flipper.",
                ),
                LocalizedString::new(
                    "He's asking if we should travel together. What do you think?",
                    "Il demande si on devrait voyager ensemble. T'en penses quoi ?",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Yes, there's safety in numbers. Travel together.",
                        "Oui, \u{00e0} deux c'est plus s\u{00fb}r. Voyagez ensemble.",
                    ),
                    next_node: "a2_kai_joins".to_string(),
                    flags_set: vec!["met_survivor_kai".to_string(), "kai_ally".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Be careful. You don't know him. Keep your distance.",
                        "Fais gaffe. Tu le connais pas. Garde tes distances.",
                    ),
                    next_node: "a2_kai_separate".to_string(),
                    flags_set: vec!["met_survivor_kai".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), -1)],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // Kai joins
    add_node(
        nodes,
        StoryNode {
            id: "a2_kai_joins".to_string(),
            messages: vec![
                LocalizedString::new(
                    "He seems relieved. Like really relieved. I think he's been lonely for a long time.",
                    "Il a l'air soulag\u{00e9}. Genre vraiment soulag\u{00e9}. Je crois qu'il \u{00e9}tait seul depuis longtemps.",
                ),
                LocalizedString::new(
                    "He shared some of his food with me. Dried meat, some nuts. Not bad. My supplies situation just got a lot better.",
                    "Il a partag\u{00e9} un peu de sa nourriture avec moi. De la viande s\u{00e9}ch\u{00e9}e, des noix. Pas mal. Ma situation de provisions s'am\u{00e9}liore d'un coup.",
                ),
                LocalizedString::new(
                    "We're going to head north together. He knows a path through the forest that avoids the open areas.",
                    "On va partir vers le nord ensemble. Il conna\u{00ee}t un chemin \u{00e0} travers la for\u{00ea}t qui \u{00e9}vite les zones d\u{00e9}couvertes.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_travel_day2".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // Kai stays separate
    add_node(
        nodes,
        StoryNode {
            id: "a2_kai_separate".to_string(),
            messages: vec![
                LocalizedString::new(
                    "He looked hurt but he understood. Said 'fair enough' and wished me luck.",
                    "Il avait l'air bless\u{00e9} mais il a compris. Il a dit \u{00ab} je comprends \u{00bb} et m'a souhait\u{00e9} bonne chance.",
                ),
                LocalizedString::new(
                    "I feel kind of bad. But you're right, I can't just trust anyone. Not in this world.",
                    "Je me sens un peu mal. Mais t'as raison, je peux pas faire confiance \u{00e0} n'importe qui. Pas dans ce monde.",
                ),
                LocalizedString::new(
                    "He did tell me about the river path before I left. Said to avoid the bridge — it's where he saw something wrong.",
                    "Il m'a quand m\u{00ea}me parl\u{00e9} du chemin de la rivi\u{00e8}re avant que je parte. Il a dit d'\u{00e9}viter le pont \u{2014} c'est l\u{00e0} qu'il a vu un truc qui allait pas.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_travel_day2".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Continue north (no smoke detour) ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_continue_north".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Yeah, you're right. No distractions. The signal is the priority.",
                    "Ouais, t'as raison. Pas de distractions. Le signal c'est la priorit\u{00e9}.",
                ),
                LocalizedString::new(
                    "I'm pushing through. The terrain is getting rougher. More trees, but they're all dead. It's like walking through a graveyard of giants.",
                    "Je continue. Le terrain devient plus difficile. Plus d'arbres, mais tous morts. C'est comme marcher dans un cimeti\u{00e8}re de g\u{00e9}ants.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_travel_day2".to_string()),
            delay: Some(180), // 3 min
            ending: None,
            trust_refusal: None,
        },
    );

    // === DAY 2 TRAVEL — Strange phenomena ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_travel_day2".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Something weird just happened.",
                    "Un truc bizarre vient de se passer.",
                ),
                LocalizedString::new(
                    "The sky flickered again. Just for a second. Like reality blinked. And my compass is going crazy — spinning in circles.",
                    "Le ciel a scintill\u{00e9} encore. Juste une seconde. Comme si la r\u{00e9}alit\u{00e9} avait clign\u{00e9}. Et ma boussole d\u{00e9}conne \u{2014} elle tourne en boucle.",
                ),
                LocalizedString::new(
                    "And I can hear... whispers? It's faint. Like a crowd murmuring very far away. But there's nobody here.",
                    "Et j'entends... des chuchotements ? C'est faible. Comme une foule qui murmure tr\u{00e8}s loin. Mais y'a personne ici.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Stay calm. Don't follow the whispers. Keep moving north.",
                        "Reste calme. Suis pas les chuchotements. Continue vers le nord.",
                    ),
                    next_node: "a2_ignore_whispers".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Can you tell which direction the whispers are coming from?",
                        "Tu peux dire de quelle direction viennent les chuchotements ?",
                    ),
                    next_node: "a2_investigate_whispers".to_string(),
                    flags_set: vec!["investigated_whispers".to_string()],
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
            id: "a2_ignore_whispers".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Yeah. Yeah, you're right. Nothing good comes from chasing phantom voices in the apocalypse.",
                    "Ouais. Ouais, t'as raison. Rien de bon vient de courir apr\u{00e8}s des voix fant\u{00f4}mes pendant l'apocalypse.",
                ),
                LocalizedString::new(
                    "I'm picking up the pace. The sooner I get to wherever that signal is, the better.",
                    "J'acc\u{00e9}l\u{00e8}re le pas. Plus vite j'arrive o\u{00f9} que ce signal m\u{00e8}ne, mieux c'est.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_resource_check".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_investigate_whispers".to_string(),
            messages: vec![
                LocalizedString::new(
                    "They're coming from... everywhere? No, wait. There's a direction. East. Toward what looks like an old church on a hill.",
                    "Ils viennent de... partout ? Non, attends. Y'a une direction. L'est. Vers ce qui ressemble \u{00e0} une vieille \u{00e9}glise sur une colline.",
                ),
                LocalizedString::new(
                    "The air feels thick there. Electric. Like before a storm but... wrong.",
                    "L'air est \u{00e9}pais l\u{00e0}-bas. \u{00c9}lectrique. Comme avant un orage mais... pas normal.",
                ),
                LocalizedString::new(
                    "I'm not going in there. Just noting it. Something is very off about that place.",
                    "J'y vais pas. Je note juste. Y'a un truc qui va vraiment pas avec cet endroit.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_resource_check".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === RESOURCE CHECK / Day 2 evening ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_resource_check".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Okay, stopping to take stock. It's getting dark and I need to eat something.",
                    "Bon, je m'arr\u{00ea}te pour faire le point. Il fait nuit et faut que je mange un truc.",
                ),
                LocalizedString::new(
                    "Food situation: I've got enough for maybe 3-4 days if I'm careful. Water's the bigger problem — I need to find a clean source.",
                    "Situation nourriture : j'ai assez pour peut-\u{00ea}tre 3-4 jours si je fais gaffe. L'eau c'est le plus gros probl\u{00e8}me \u{2014} faut que je trouve une source propre.",
                ),
                LocalizedString::new(
                    "I found an old gas station nearby. Might have supplies inside, but the door's jammed. I'd have to break in and it would make a lot of noise.",
                    "J'ai trouv\u{00e9} une vieille station-service pas loin. Y'a peut-\u{00ea}tre des provisions dedans, mais la porte est coinc\u{00e9}e. Faudrait que je force l'entr\u{00e9}e et \u{00e7}a ferait beaucoup de bruit.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Break in. You need the supplies. Deal with the noise.",
                        "Force l'entr\u{00e9}e. T'as besoin des provisions. Tant pis pour le bruit.",
                    ),
                    next_node: "a2_break_in".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("supplies".to_string(), 2)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Too risky. Conserve what you have and keep moving at dawn.",
                        "Trop risqu\u{00e9}. Garde ce que t'as et repars \u{00e0} l'aube.",
                    ),
                    next_node: "a2_conserve".to_string(),
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
        nodes,
        StoryNode {
            id: "a2_break_in".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Okay, I found a rock and smashed the window. Loud as hell. My heart is pounding.",
                    "Bon, j'ai trouv\u{00e9} une pierre et j'ai d\u{00e9}fonc\u{00e9} la vitre. Un boucan d'enfer. Mon c\u{0153}ur bat \u{00e0} fond.",
                ),
                LocalizedString::new(
                    "But it paid off. Bottled water, energy bars, and — oh thank god — a box of ibuprofen. My arm's been killing me.",
                    "Mais \u{00e7}a a pay\u{00e9}. De l'eau en bouteille, des barres \u{00e9}nerg\u{00e9}tiques, et \u{2014} oh dieu merci \u{2014} une bo\u{00ee}te d'ibuprof\u{00e8}ne. Mon bras me tuait.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_day3_morning".to_string()),
            delay: Some(120),
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_conserve".to_string(),
            messages: vec![
                LocalizedString::new(
                    "You're probably right. I'll ration what I have. Found a sheltered spot behind the station to sleep.",
                    "T'as s\u{00fb}rement raison. Je vais rationner ce que j'ai. J'ai trouv\u{00e9} un coin abrit\u{00e9} derri\u{00e8}re la station pour dormir.",
                ),
                LocalizedString::new(
                    "I'll try to find water tomorrow. There has to be a stream or something around here.",
                    "J'essaierai de trouver de l'eau demain. Y doit bien y avoir un ruisseau ou un truc du genre dans le coin.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_day3_morning".to_string()),
            delay: Some(120),
            ending: None,
            trust_refusal: None,
        },
    );

    // === DAY 3 — Moral dilemma ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_day3_morning".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Day three. I'm making progress. The signal is getting stronger. I think I'm getting close.",
                    "Troisi\u{00e8}me jour. J'avance. Le signal se renforce. Je crois que je m'approche.",
                ),
                LocalizedString::new(
                    "But... I just found something. There's a woman on the side of the road. She's hurt. Her leg is badly twisted — broken maybe. She's conscious but barely.",
                    "Mais... je viens de trouver un truc. Y'a une femme au bord de la route. Elle est bless\u{00e9}e. Sa jambe est tordue \u{2014} cass\u{00e9}e peut-\u{00ea}tre. Elle est consciente mais \u{00e0} peine.",
                ),
                LocalizedString::new(
                    "She's asking for help. She says her name is Lena, and she was trying to reach the northern signal too.",
                    "Elle demande de l'aide. Elle dit qu'elle s'appelle Lena, et qu'elle essayait d'atteindre le signal du nord aussi.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Help her. Give her medicine and try to splint the leg.",
                        "Aide-la. Donne-lui des m\u{00e9}dicaments et essaie d'\u{00e9}clisser la jambe.",
                    ),
                    next_node: "a2_help_lena".to_string(),
                    flags_set: vec!["helped_stranger".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![
                        ("supplies".to_string(), -1),
                        ("morale".to_string(), 2),
                        ("trust_level".to_string(), 1),
                    ],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "You can't stop. It's too dangerous, and you don't have supplies to spare.",
                        "Tu peux pas t'arr\u{00ea}ter. C'est trop dangereux, et t'as pas de provisions en rab.",
                    ),
                    next_node: "a2_leave_lena".to_string(),
                    flags_set: vec!["left_stranger".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), -2)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Give her water and directions, but you can't carry her.",
                        "Donne-lui de l'eau et des indications, mais tu peux pas la porter.",
                    ),
                    next_node: "a2_partial_help".to_string(),
                    flags_set: vec!["partial_help_stranger".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("supplies".to_string(), -1)],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            // Trust refusal: if trust is low, Elara helps anyway
            trust_refusal: Some(TrustRefusal {
                min_trust: 2,
                refusal_node: "a2_elara_helps_anyway".to_string(),
                refusal_message: LocalizedString::new(
                    "Sorry, but... I can't just leave her. I know you think it's too risky, but I can't walk past someone who's suffering. I'm going to help her.",
                    "D\u{00e9}sol\u{00e9}e, mais... je peux pas juste la laisser. Je sais que tu trouves \u{00e7}a trop risqu\u{00e9}, mais je peux pas passer \u{00e0} c\u{00f4}t\u{00e9} de quelqu'un qui souffre. Je vais l'aider.",
                ),
            }),
        },
    );

    // Trust refusal override — Elara helps regardless
    add_node(
        nodes,
        StoryNode {
            id: "a2_elara_helps_anyway".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I gave her some water and wrapped her leg as best I could. She told me about a shortcut through the valley. Might be useful.",
                    "Je lui ai donn\u{00e9} de l'eau et j'ai band\u{00e9} sa jambe du mieux que j'ai pu. Elle m'a parl\u{00e9} d'un raccourci par la vall\u{00e9}e. \u{00c7}a pourrait \u{00ea}tre utile.",
                ),
                LocalizedString::new(
                    "I know you don't agree. But some things are more important than survival strategy.",
                    "Je sais que t'es pas d'accord. Mais certaines choses sont plus importantes que la strat\u{00e9}gie de survie.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_day3_evening".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_help_lena".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I helped her. Set the leg with a branch and some torn cloth. She's in pain but she can sort of hobble now.",
                    "Je l'ai aid\u{00e9}e. J'ai immobilis\u{00e9} la jambe avec une branche et du tissu d\u{00e9}chir\u{00e9}. Elle souffre mais elle arrive \u{00e0} boitiller maintenant.",
                ),
                LocalizedString::new(
                    "She told me about a shortcut through the valley. Said there's an old bunker entrance near a dried-up lake. Could be important.",
                    "Elle m'a parl\u{00e9} d'un raccourci par la vall\u{00e9}e. Elle dit qu'y a une entr\u{00e9}e de bunker pr\u{00e8}s d'un lac ass\u{00e9}ch\u{00e9}. \u{00c7}a pourrait \u{00ea}tre important.",
                ),
                LocalizedString::new(
                    "She's going to try to make it south. Said there's a farmhouse she knows about. I hope she makes it.",
                    "Elle va essayer d'aller vers le sud. Elle dit qu'il y a une ferme qu'elle conna\u{00ee}t. J'esp\u{00e8}re qu'elle y arrivera.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_day3_evening".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_leave_lena".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I... I kept walking. She called after me. I didn't turn around.",
                    "J'ai... j'ai continu\u{00e9} \u{00e0} marcher. Elle m'a appel\u{00e9}e. Je me suis pas retourn\u{00e9}e.",
                ),
                LocalizedString::new(
                    "You're right. I know you're right. But I feel sick. This world is turning me into something I don't want to be.",
                    "T'as raison. Je sais que t'as raison. Mais j'ai la naus\u{00e9}e. Ce monde est en train de me transformer en quelque chose que je veux pas \u{00ea}tre.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_day3_evening".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_partial_help".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I gave her water and pointed her south. Told her about the gas station I passed. It's not much, but it's something.",
                    "Je lui ai donn\u{00e9} de l'eau et je l'ai orient\u{00e9}e vers le sud. Je lui ai parl\u{00e9} de la station-service que j'ai pass\u{00e9}e. C'est pas grand-chose, mais c'est quelque chose.",
                ),
                LocalizedString::new(
                    "She thanked me. Told me about a bunker entrance she'd seen near a dried-up lake to the north. Said it might have supplies.",
                    "Elle m'a remerci\u{00e9}e. Elle m'a parl\u{00e9} d'une entr\u{00e9}e de bunker qu'elle avait vue pr\u{00e8}s d'un lac ass\u{00e9}ch\u{00e9} au nord. Elle dit qu'y a peut-\u{00ea}tre des provisions.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_day3_evening".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === DAY 3 evening / DAY 4 ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_day3_evening".to_string(),
            messages: vec![
                LocalizedString::new(
                    "The signal is definitely getting closer. I can feel it now — not just hear it. It's like a vibration in my chest.",
                    "Le signal est clairement plus proche. Je le sens maintenant \u{2014} pas juste l'entendre. C'est comme une vibration dans ma poitrine.",
                ),
                LocalizedString::new(
                    "The sky is doing that thing again. Flickering. Colors that shouldn't exist. It's beautiful in a terrifying way.",
                    "Le ciel refait ce truc. Scintillement. Des couleurs qui devraient pas exister. C'est beau d'une fa\u{00e7}on terrifiante.",
                ),
                LocalizedString::new(
                    "I'm going to set up camp and try to sleep. Tomorrow should be the day I reach... whatever's sending that signal.",
                    "Je vais monter le camp et essayer de dormir. Demain devrait \u{00ea}tre le jour o\u{00f9} j'atteins... ce qui envoie ce signal.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Almost there. Stay strong, Elara.",
                        "T'y es presque. Tiens bon, Elara.",
                    ),
                    next_node: "a2_night_day3".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Be ready for anything tomorrow. Whatever's making that signal might not be friendly.",
                        "Sois pr\u{00ea}te \u{00e0} tout demain. Ce qui \u{00e9}met ce signal est pas forc\u{00e9}ment amical.",
                    ),
                    next_node: "a2_night_day3".to_string(),
                    flags_set: vec!["warned_about_signal".to_string()],
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
            id: "a2_night_day3".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Thanks. For everything. I don't think I'd have made it this far without you.",
                    "Merci. Pour tout. Je crois que j'aurais pas tenu aussi loin sans toi.",
                ),
                LocalizedString::new(
                    "Good night. Tomorrow's a big day.",
                    "Bonne nuit. Demain c'est un grand jour.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_day4".to_string()),
            delay: Some(300), // 5 min
            ending: None,
            trust_refusal: None,
        },
    );

    // === DAY 4 — Approaching the signal source ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_day4".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I can see it.",
                    "Je le vois.",
                ),
                LocalizedString::new(
                    "It's a town. Or what used to be one. Buildings are mostly intact but everything's covered in this... grey dust. Like ash but finer.",
                    "C'est un village. Ou ce que c'\u{00e9}tait. Les b\u{00e2}timents sont plut\u{00f4}t intacts mais tout est recouvert de cette... poussi\u{00e8}re grise. Comme de la cendre mais plus fine.",
                ),
                LocalizedString::new(
                    "The signal is coming from a large building at the center. Looks like it was a town hall or a school.",
                    "Le signal vient d'un grand b\u{00e2}timent au centre. On dirait une mairie ou une \u{00e9}cole.",
                ),
                LocalizedString::new(
                    "And there are lights. Actual lights. Someone has power in there.",
                    "Et y'a des lumi\u{00e8}res. De vraies lumi\u{00e8}res. Quelqu'un a de l'\u{00e9}lectricit\u{00e9} l\u{00e0}-dedans.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Approach carefully. Scout the area first before going in.",
                        "Approche-toi prudemment. Observe la zone d'abord avant d'entrer.",
                    ),
                    next_node: "a2_scout_town".to_string(),
                    flags_set: vec!["scouted_town".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Go in. If they have power, they have organization. This could be the safe zone.",
                        "Vas-y. S'ils ont de l'\u{00e9}lectricit\u{00e9}, ils sont organis\u{00e9}s. C'est peut-\u{00ea}tre la zone s\u{00fb}re.",
                    ),
                    next_node: "a2_enter_town".to_string(),
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
        nodes,
        StoryNode {
            id: "a2_scout_town".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Good call. I found a hill overlooking the town. Let me see...",
                    "Bonne id\u{00e9}e. J'ai trouv\u{00e9} une colline qui surplombe le village. Voyons voir...",
                ),
                LocalizedString::new(
                    "I count maybe a dozen people moving around. They seem... normal. Building things, carrying supplies. There's even a garden.",
                    "Je compte peut-\u{00ea}tre une douzaine de personnes qui bougent. Elles ont l'air... normales. Elles construisent des trucs, transportent des provisions. Y'a m\u{00ea}me un jardin.",
                ),
                LocalizedString::new(
                    "But there's also a wall being built around the perimeter. And I see someone on watch with what looks like a rifle.",
                    "Mais y'a aussi un mur en construction autour du p\u{00e9}rim\u{00e8}tre. Et je vois quelqu'un de garde avec ce qui ressemble \u{00e0} un fusil.",
                ),
                LocalizedString::new(
                    "These people are surviving, but they're also defending against something. Okay, I'm going in.",
                    "Ces gens survivent, mais ils se d\u{00e9}fendent aussi contre quelque chose. Bon, j'y vais.",
                ),
            ],
            choices: vec![],
            next_node: Some("a2_enter_town".to_string()),
            delay: Some(120),
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a2_enter_town".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm in. They saw me coming and a group met me at the entrance. They were cautious but not hostile.",
                    "J'y suis. Ils m'ont vue arriver et un groupe m'a accueillie \u{00e0} l'entr\u{00e9}e. M\u{00e9}fiants mais pas hostiles.",
                ),
                LocalizedString::new(
                    "The leader is a woman named Dr. Osei. She's a physicist. She says they've been gathering survivors and trying to understand what the Eshara is.",
                    "La cheffe est une femme appel\u{00e9}e Dr Osei. C'est une physicienne. Elle dit qu'ils rassemblent des survivants et essaient de comprendre ce qu'est l'Eshara.",
                ),
                LocalizedString::new(
                    "She says they have a working generator, food production, and defenses. But she also says the phenomena are getting worse. The flickering is more frequent. The changed ones are moving in patterns now.",
                    "Elle dit qu'ils ont un g\u{00e9}n\u{00e9}rateur, de la production de nourriture, et des d\u{00e9}fenses. Mais elle dit aussi que les ph\u{00e9}nom\u{00e8}nes empirent. Les scintillements sont plus fr\u{00e9}quents. Les chang\u{00e9}s se d\u{00e9}placent en sch\u{00e9}mas maintenant.",
                ),
                LocalizedString::new(
                    "She asked where I came from. When I mentioned the Helios facility, her face changed. She wants to talk more tomorrow.",
                    "Elle a demand\u{00e9} d'o\u{00f9} je venais. Quand j'ai mentionn\u{00e9} le centre Helios, son visage a chang\u{00e9}. Elle veut en reparler demain.",
                ),
                LocalizedString::new(
                    "They gave me a real bed. With blankets. I almost cried.",
                    "Ils m'ont donn\u{00e9} un vrai lit. Avec des couvertures. J'ai failli pleurer.",
                ),
                LocalizedString::new(
                    "I'm safe for now. Going to sleep. Talk tomorrow?",
                    "J'suis en s\u{00e9}curit\u{00e9} pour le moment. Je vais dormir. On se parle demain ?",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "You made it. Sleep well, Elara. You deserve this.",
                        "T'as r\u{00e9}ussi. Dors bien, Elara. Tu le m\u{00e9}rites.",
                    ),
                    next_node: "a2_end".to_string(),
                    flags_set: vec!["reached_settlement".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![
                        ("morale".to_string(), 2),
                        ("health".to_string(), 1),
                        ("trust_level".to_string(), 1),
                    ],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Good work finding them. But be careful — we don't know their full story yet.",
                        "Bien jou\u{00e9} de les avoir trouv\u{00e9}s. Mais fais gaffe \u{2014} on conna\u{00ee}t pas toute leur histoire encore.",
                    ),
                    next_node: "a2_end".to_string(),
                    flags_set: vec!["reached_settlement".to_string(), "cautious_about_settlement".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), 1), ("health".to_string(), 1)],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === END OF ACT 2 — overnight transition ===
    add_node(
        nodes,
        StoryNode {
            id: "a2_end".to_string(),
            messages: vec![LocalizedString::new(
                "Good night. First real good night in three months.",
                "Bonne nuit. La premi\u{00e8}re vraie bonne nuit en trois mois.",
            )],
            choices: vec![],
            next_node: Some("placeholder_end".to_string()), // Will connect to Act 3
            delay: Some(300),                               // 5 min
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
        assert!(tree.contains_key("a1_explore_facility"));
        assert!(tree.contains_key("a1_head_north"));
        assert!(tree.contains_key("a1_night"));
    }

    #[test]
    fn test_act2_nodes_exist() {
        let tree = build_story_tree();
        assert!(tree.contains_key("a2_morning"));
        assert!(tree.contains_key("a2_meet_kai"));
        assert!(tree.contains_key("a2_day3_morning")); // Moral dilemma
        assert!(tree.contains_key("a2_enter_town"));
        assert!(tree.contains_key("a2_end"));
    }

    #[test]
    fn test_act2_has_trust_refusal() {
        let tree = build_story_tree();
        let dilemma = tree.get("a2_day3_morning").unwrap();
        assert!(dilemma.trust_refusal.is_some());
    }

    #[test]
    fn test_total_node_count() {
        let tree = build_story_tree();
        // Acts 1+2 + placeholder should be at least 30 nodes
        assert!(
            tree.len() >= 30,
            "Expected at least 30 nodes, got {}",
            tree.len()
        );
    }
}
