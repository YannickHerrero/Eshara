use std::collections::HashMap;

use crate::i18n::LocalizedString;
use crate::story::{Choice, Condition, StoryNode, TrustRefusal};

/// Build the complete story tree as a HashMap from node id -> StoryNode
pub fn build_story_tree() -> HashMap<String, StoryNode> {
    let mut nodes = HashMap::new();

    build_act1(&mut nodes);
    build_act2(&mut nodes);
    build_act3(&mut nodes);
    build_act4(&mut nodes);
    build_act5(&mut nodes);

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
            next_node: Some("a3_morning".to_string()),
            delay: Some(300), // 5 min
            ending: None,
            trust_refusal: None,
        },
    );
}

// ── ACT 3 — The Discovery (Days 5-7) ─────────────────────────

fn build_act3(nodes: &mut HashMap<String, StoryNode>) {
    // === DAY 5 — Dr. Osei's revelation ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_morning".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Morning. I actually slept. Like, really slept. I forgot what that felt like.",
                    "Bonjour. J'ai vraiment dormi. Genre, vraiment. J'avais oubli\u{00e9} ce que \u{00e7}a faisait.",
                ),
                LocalizedString::new(
                    "Dr. Osei wants to see me. She said it's urgent. Something about the Helios facility.",
                    "Le Dr Osei veut me voir. Elle dit que c'est urgent. Un truc \u{00e0} propos du centre Helios.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_osei_talk".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a3_osei_talk".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I just had the most intense conversation of my life.",
                    "Je viens d'avoir la conversation la plus intense de ma vie.",
                ),
                LocalizedString::new(
                    "Dr. Osei used to work for the same organization that funded Helios. She was at a sister facility — Project MIROIR was a joint operation.",
                    "Le Dr Osei travaillait pour la m\u{00ea}me organisation qui finan\u{00e7}ait Helios. Elle \u{00e9}tait dans un centre partenaire \u{2014} le Projet MIROIR \u{00e9}tait une op\u{00e9}ration conjointe.",
                ),
                LocalizedString::new(
                    "She says the Eshara wasn't natural. It was a resonance experiment that went catastrophically wrong. They were trying to open a... a gateway. To somewhere else.",
                    "Elle dit que l'Eshara \u{00e9}tait pas naturel. C'\u{00e9}tait une exp\u{00e9}rience de r\u{00e9}sonance qui a catastrophiquement d\u{00e9}raill\u{00e9}. Ils essayaient d'ouvrir un... un portail. Vers ailleurs.",
                ),
                LocalizedString::new(
                    "The 'changed' people? She thinks they're partially shifted into that other place. Not fully here anymore.",
                    "Les gens 'chang\u{00e9}s' ? Elle pense qu'ils sont partiellement d\u{00e9}cal\u{00e9}s vers cet autre endroit. Plus enti\u{00e8}rement ici.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Can it be reversed? Is there a way to fix this?",
                        "Est-ce qu'on peut inverser \u{00e7}a ? Y'a un moyen de r\u{00e9}parer ?",
                    ),
                    next_node: "a3_osei_solution".to_string(),
                    flags_set: vec!["asked_about_reversal".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "This is huge. Do the other survivors know?",
                        "C'est \u{00e9}norme. Les autres survivants sont au courant ?",
                    ),
                    next_node: "a3_osei_secret".to_string(),
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
            id: "a3_osei_solution".to_string(),
            messages: vec![
                LocalizedString::new(
                    "She thinks so. Maybe. The resonance cascade is still running — that's what's causing the phenomena to get worse. If it could be shut down at the source...",
                    "Elle pense que oui. Peut-\u{00ea}tre. La cascade de r\u{00e9}sonance tourne toujours \u{2014} c'est ce qui fait empirer les ph\u{00e9}nom\u{00e8}nes. Si on pouvait l'arr\u{00ea}ter \u{00e0} la source...",
                ),
                LocalizedString::new(
                    "The source is Helios. My facility. The one I just left. The locked door — Project MIROIR — that's where the equipment is.",
                    "La source c'est Helios. Mon centre. Celui que je viens de quitter. La porte verrouill\u{00e9}e \u{2014} Projet MIROIR \u{2014} c'est l\u{00e0} que l'\u{00e9}quipement est.",
                ),
                LocalizedString::new(
                    "She has a keycard. She kept it. She says if someone could get back there and shut down the resonance chamber, it might stop the cascade. Maybe even reverse it.",
                    "Elle a un badge. Elle l'a gard\u{00e9}. Elle dit que si quelqu'un pouvait retourner l\u{00e0}-bas et \u{00e9}teindre la chambre de r\u{00e9}sonance, \u{00e7}a pourrait arr\u{00ea}ter la cascade. Peut-\u{00ea}tre m\u{00ea}me l'inverser.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_keycard_decision".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a3_osei_secret".to_string(),
            messages: vec![
                LocalizedString::new(
                    "No. She's kept it quiet. She says people here are barely holding on — if they found out it was man-made, it could tear the community apart.",
                    "Non. Elle a gard\u{00e9} \u{00e7}a secret. Elle dit que les gens ici tiennent \u{00e0} peine \u{2014} s'ils apprenaient que c'\u{00e9}tait artificiel, \u{00e7}a pourrait d\u{00e9}truire la communaut\u{00e9}.",
                ),
                LocalizedString::new(
                    "But she also said there might be a way to stop it. She has a keycard to the MIROIR lab at Helios. Someone would need to go back and shut down the resonance equipment.",
                    "Mais elle a aussi dit qu'il y avait peut-\u{00ea}tre un moyen d'arr\u{00ea}ter \u{00e7}a. Elle a un badge pour le labo MIROIR \u{00e0} Helios. Quelqu'un devrait retourner l\u{00e0}-bas et \u{00e9}teindre l'\u{00e9}quipement de r\u{00e9}sonance.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_keycard_decision".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === KEY DECISION: Go back to Helios or stay safe ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_keycard_decision".to_string(),
            messages: vec![
                LocalizedString::new(
                    "So here's the thing. She's offering me the keycard. She can't go herself — she's needed here to keep the settlement running.",
                    "Alors voil\u{00e0}. Elle me propose le badge. Elle peut pas y aller elle-m\u{00ea}me \u{2014} elle est indispensable ici pour faire tourner la colonie.",
                ),
                LocalizedString::new(
                    "Going back to Helios means days of travel through increasingly dangerous territory. Alone. To a facility I barely explored.",
                    "Retourner \u{00e0} Helios veut dire des jours de voyage \u{00e0} travers un territoire de plus en plus dangereux. Seule. Vers un centre que j'ai \u{00e0} peine explor\u{00e9}.",
                ),
                LocalizedString::new(
                    "Or I could stay here. Safe. Alive. And hope the cascade burns itself out eventually.",
                    "Ou je pourrais rester ici. En s\u{00e9}curit\u{00e9}. En vie. Et esp\u{00e9}rer que la cascade s'\u{00e9}puise d'elle-m\u{00ea}me.",
                ),
                LocalizedString::new(
                    "What do you think?",
                    "T'en penses quoi ?",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Take the keycard. If there's a chance to stop this, you have to try.",
                        "Prends le badge. S'il y a une chance d'arr\u{00ea}ter \u{00e7}a, faut que t'essaies.",
                    ),
                    next_node: "a3_take_keycard".to_string(),
                    flags_set: vec!["has_lab_keycard".to_string(), "knows_truth".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Stay in the settlement. You're safe here. Don't risk your life.",
                        "Reste dans la colonie. T'es en s\u{00e9}curit\u{00e9} ici. Risque pas ta vie.",
                    ),
                    next_node: "a3_stay_safe".to_string(),
                    flags_set: vec!["knows_truth".to_string(), "stayed_safe".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            // Trust refusal: if trust is low, Elara decides on her own
            trust_refusal: Some(TrustRefusal {
                min_trust: 3,
                refusal_node: "a3_elara_decides_keycard".to_string(),
                refusal_message: LocalizedString::new(
                    "Look... I appreciate your input, but I need to think about this myself. This is too big for me to just follow someone else's call. Give me a moment.",
                    "\u{00c9}coute... j'appr\u{00e9}cie ton avis, mais faut que j'y r\u{00e9}fl\u{00e9}chisse moi-m\u{00ea}me. C'est trop gros pour que je suive juste l'avis de quelqu'un d'autre. Laisse-moi un moment.",
                ),
            }),
        },
    );

    // Trust refusal: Elara decides on her own
    add_node(
        nodes,
        StoryNode {
            id: "a3_elara_decides_keycard".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I thought about it. I'm taking the keycard. I can't sit here knowing I might be the only person who can stop this.",
                    "J'y ai r\u{00e9}fl\u{00e9}chi. Je prends le badge. Je peux pas rester l\u{00e0} en sachant que je suis peut-\u{00ea}tre la seule personne qui peut arr\u{00ea}ter \u{00e7}a.",
                ),
                LocalizedString::new(
                    "I know you might not agree. But this is my choice.",
                    "Je sais que t'es peut-\u{00ea}tre pas d'accord. Mais c'est mon choix.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_preparation".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Branch: Take the keycard ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_take_keycard".to_string(),
            messages: vec![
                LocalizedString::new(
                    "You're right. If there's even a small chance... I have to try. For everyone.",
                    "T'as raison. S'il y a ne serait-ce qu'une petite chance... faut que j'essaie. Pour tout le monde.",
                ),
                LocalizedString::new(
                    "Dr. Osei gave me the keycard and a rough schematic of the MIROIR lab. She thinks the main control console is in the deepest chamber.",
                    "Le Dr Osei m'a donn\u{00e9} le badge et un sch\u{00e9}ma approximatif du labo MIROIR. Elle pense que la console principale est dans la salle la plus profonde.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_preparation".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Branch: Stay safe ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_stay_safe".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Yeah. Yeah, you're right. I almost died getting here. I can't just throw that away on a suicide mission.",
                    "Ouais. Ouais, t'as raison. J'ai failli mourir en venant ici. Je peux pas gaspiller \u{00e7}a sur une mission suicide.",
                ),
                LocalizedString::new(
                    "Dr. Osei looked disappointed but she understood. She said she'd keep looking for another way.",
                    "Le Dr Osei avait l'air d\u{00e9}\u{00e7}ue mais elle a compris. Elle dit qu'elle cherchera un autre moyen.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_day6".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Preparation (keycard path) ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_preparation".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm spending today preparing. Dr. Osei is loading me up with supplies. Better food, water, a proper first aid kit.",
                    "Je passe la journ\u{00e9}e \u{00e0} me pr\u{00e9}parer. Le Dr Osei me charge en provisions. De la meilleure nourriture, de l'eau, une vraie trousse de premiers soins.",
                ),
                LocalizedString::new(
                    "She also gave me something else. A weapon. An old hunting knife. 'Just in case,' she said.",
                    "Elle m'a aussi donn\u{00e9} autre chose. Une arme. Un vieux couteau de chasse. \u{00ab} Au cas o\u{00f9} \u{00bb}, elle a dit.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_day6".to_string()),
            delay: Some(180),
            ending: None,
            trust_refusal: None,
        },
    );

    // === DAY 6 — The betrayal/twist ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_day6".to_string(),
            messages: vec![LocalizedString::new(
                "Something happened. Something bad.",
                "Y'a eu un truc. Un mauvais truc.",
            )],
            choices: vec![],
            next_node: Some("a3_kai_twist".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a3_kai_twist".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Kai showed up at the settlement.",
                    "Ka\u{00ef} est arriv\u{00e9} \u{00e0} la colonie.",
                ),
                LocalizedString::new(
                    "But he didn't come alone. He brought a group. And they're not friendly. They want the generator. They want the food. They want control.",
                    "Mais il est pas venu seul. Il a amen\u{00e9} un groupe. Et ils sont pas amicaux. Ils veulent le g\u{00e9}n\u{00e9}rateur. Ils veulent la nourriture. Ils veulent le contr\u{00f4}le.",
                ),
                LocalizedString::new(
                    "He was scouting for them all along. All that 'nice guy alone in the wild' stuff... it was an act.",
                    "Il faisait du rep\u{00e9}rage pour eux depuis le d\u{00e9}but. Tout ce num\u{00e9}ro du \u{00ab} mec sympa seul dans la nature \u{00bb}... c'\u{00e9}tait du cin\u{00e9}.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_kai_confrontation".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a3_kai_confrontation".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Dr. Osei is trying to negotiate. The settlement has weapons but they're outnumbered. This could get ugly fast.",
                    "Le Dr Osei essaie de n\u{00e9}gocier. La colonie a des armes mais ils sont en inf\u{00e9}riorit\u{00e9} num\u{00e9}rique. \u{00c7}a pourrait vite d\u{00e9}g\u{00e9}n\u{00e9}rer.",
                ),
                LocalizedString::new(
                    "Kai saw me. He had the nerve to smile. Like we were old friends.",
                    "Ka\u{00ef} m'a vue. Il a eu le culot de sourire. Comme si on \u{00e9}tait de vieux amis.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Try to talk to Kai directly. Appeal to whatever humanity he has left.",
                        "Essaie de parler \u{00e0} Ka\u{00ef} directement. Fais appel \u{00e0} ce qui reste d'humanit\u{00e9} en lui.",
                    ),
                    next_node: "a3_talk_kai".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![Condition::FlagSet("kai_ally".to_string())],
                },
                Choice {
                    label: LocalizedString::new(
                        "Help Dr. Osei defend the settlement. Stand your ground.",
                        "Aide le Dr Osei \u{00e0} d\u{00e9}fendre la colonie. Tiens bon.",
                    ),
                    next_node: "a3_defend".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Use the chaos to slip away. You have a mission — get to Helios.",
                        "Profite du chaos pour t'\u{00e9}clipser. T'as une mission \u{2014} va \u{00e0} Helios.",
                    ),
                    next_node: "a3_slip_away".to_string(),
                    flags_set: vec!["abandoned_settlement".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), -2)],
                    conditions: vec![Condition::FlagSet("has_lab_keycard".to_string())],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // Talk to Kai
    add_node(
        nodes,
        StoryNode {
            id: "a3_talk_kai".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I confronted him. Told him this wasn't who he had to be. That we traveled together, that he shared food with me.",
                    "Je l'ai confront\u{00e9}. Je lui ai dit que c'\u{00e9}tait pas oblig\u{00e9} d'\u{00ea}tre comme \u{00e7}a. Qu'on avait voyag\u{00e9} ensemble, qu'il avait partag\u{00e9} sa bouffe avec moi.",
                ),
                LocalizedString::new(
                    "He hesitated. I could see it in his eyes. Then his group leader called him back and the moment was gone.",
                    "Il a h\u{00e9}sit\u{00e9}. Je l'ai vu dans ses yeux. Puis le chef de son groupe l'a rappel\u{00e9} et le moment est pass\u{00e9}.",
                ),
                LocalizedString::new(
                    "But the delay was enough. Dr. Osei had time to rally the defenders. Kai's group backed off when they saw the rifles. For now.",
                    "Mais le d\u{00e9}lai a suffi. Le Dr Osei a eu le temps de rallier les d\u{00e9}fenseurs. Le groupe de Ka\u{00ef} a recul\u{00e9} quand ils ont vu les fusils. Pour l'instant.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_aftermath".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // Defend the settlement
    add_node(
        nodes,
        StoryNode {
            id: "a3_defend".to_string(),
            messages: vec![
                LocalizedString::new(
                    "We held them off. Barely. There were tense moments. Someone fired a warning shot. Kai's group realized we weren't easy targets.",
                    "On les a repouss\u{00e9}s. De justesse. Y'a eu des moments tendus. Quelqu'un a tir\u{00e9} un coup de semonce. Le groupe de Ka\u{00ef} a r\u{00e9}alis\u{00e9} qu'on \u{00e9}tait pas des proies faciles.",
                ),
                LocalizedString::new(
                    "They pulled back to the tree line. But they're still out there. Watching.",
                    "Ils se sont repli\u{00e9}s en lisi\u{00e8}re de for\u{00ea}t. Mais ils sont toujours l\u{00e0}. \u{00c0} observer.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_aftermath".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // Slip away
    add_node(
        nodes,
        StoryNode {
            id: "a3_slip_away".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I grabbed my pack and slipped out the back while everyone was focused on Kai's group. I feel terrible about it.",
                    "J'ai attrap\u{00e9} mon sac et j'me suis faufil\u{00e9}e par derri\u{00e8}re pendant que tout le monde \u{00e9}tait focalis\u{00e9} sur le groupe de Ka\u{00ef}. Je me sens horrible.",
                ),
                LocalizedString::new(
                    "But the mission matters more. If I can stop the Eshara, none of this fighting matters anyway.",
                    "Mais la mission compte plus. Si je peux arr\u{00ea}ter l'Eshara, tout ce conflit n'aura plus d'importance.",
                ),
                LocalizedString::new(
                    "At least... that's what I'm telling myself.",
                    "Au moins... c'est ce que je me dis.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_day7".to_string()),
            delay: Some(180),
            ending: None,
            trust_refusal: None,
        },
    );

    // === Aftermath ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_aftermath".to_string(),
            messages: vec![
                LocalizedString::new(
                    "The settlement is tense. People are scared. Dr. Osei thinks Kai's group will be back, probably with more people.",
                    "La colonie est tendue. Les gens ont peur. Le Dr Osei pense que le groupe de Ka\u{00ef} va revenir, probablement avec plus de monde.",
                ),
                LocalizedString::new(
                    "She pulled me aside. She says time is running out — the phenomena are accelerating. If I'm going to go back to Helios, it has to be now.",
                    "Elle m'a prise \u{00e0} part. Elle dit que le temps presse \u{2014} les ph\u{00e9}nom\u{00e8}nes s'acc\u{00e9}l\u{00e8}rent. Si je dois retourner \u{00e0} Helios, c'est maintenant.",
                ),
            ],
            choices: vec![
                // Auto-route: if player stayed safe, they refused the keycard initially.
                // After Kai's betrayal, they could still go — but their arc leads elsewhere.
                Choice {
                    label: LocalizedString::new("...", "..."),
                    next_node: "a3_stayed_safe_aftermath".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![Condition::FlagSet("stayed_safe".to_string())],
                },
                Choice {
                    label: LocalizedString::new(
                        "Go now. Every hour counts.",
                        "Pars maintenant. Chaque heure compte.",
                    ),
                    next_node: "a3_day7".to_string(),
                    flags_set: vec!["has_lab_keycard".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![Condition::FlagUnset("stayed_safe".to_string())],
                },
                Choice {
                    label: LocalizedString::new(
                        "Stay one more night to rest and prepare. You'll need your strength.",
                        "Reste une nuit de plus pour te reposer et te pr\u{00e9}parer. T'auras besoin de tes forces.",
                    ),
                    next_node: "a3_one_more_night".to_string(),
                    flags_set: vec!["has_lab_keycard".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("health".to_string(), 1)],
                    conditions: vec![Condition::FlagUnset("stayed_safe".to_string())],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === Stayed safe aftermath — leads to Static ending ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_stayed_safe_aftermath".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Dr. Osei offered me the keycard again. But I... I can't. After everything that just happened, I can't leave these people. Not now.",
                    "Le Dr Osei m'a reproposé le badge. Mais je... je peux pas. Après tout ce qui vient de se passer, je peux pas laisser ces gens. Pas maintenant.",
                ),
                LocalizedString::new(
                    "She looked at me for a long time. Then she nodded. 'Then we wait,' she said. 'And we hope.'",
                    "Elle m'a regardée longuement. Puis elle a hoché la tête. « Alors on attend », elle a dit. « Et on espère. »",
                ),
                LocalizedString::new(
                    "I chose to stay. I chose safety. I chose these people over the mission.",
                    "J'ai choisi de rester. J'ai choisi la sécurité. J'ai choisi ces gens plutôt que la mission.",
                ),
                LocalizedString::new(
                    "I hope it was the right call.",
                    "J'espère que c'était le bon choix.",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_static_buildup".to_string()),
            delay: Some(600),
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a3_one_more_night".to_string(),
            messages: vec![
                LocalizedString::new(
                    "One more night. I'll leave at first light.",
                    "Une nuit de plus. Je pars aux premi\u{00e8}res lueurs.",
                ),
                LocalizedString::new(
                    "Dr. Osei went over the schematics with me again. I think I understand the system. Find the resonance chamber, shut down the primary oscillator, then seal the rift.",
                    "Le Dr Osei a revu les sch\u{00e9}mas avec moi. Je crois que je comprends le syst\u{00e8}me. Trouver la chambre de r\u{00e9}sonance, \u{00e9}teindre l'oscillateur primaire, puis sceller la br\u{00e8}che.",
                ),
                LocalizedString::new(
                    "'Seal the rift.' She said it like it was a normal thing to say.",
                    "\u{00ab} Sceller la br\u{00e8}che. \u{00bb} Elle a dit \u{00e7}a comme si c'\u{00e9}tait un truc normal.",
                ),
            ],
            choices: vec![],
            next_node: Some("a3_day7".to_string()),
            delay: Some(180),
            ending: None,
            trust_refusal: None,
        },
    );

    // === DAY 7 — Heading back ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_day7".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm on the move. Heading south, back toward Helios. The landscape feels different now. Worse.",
                    "J'suis en route. Direction sud, retour vers Helios. Le paysage semble diff\u{00e9}rent maintenant. Pire.",
                ),
                LocalizedString::new(
                    "The sky flickers every few hours now. And the whispers are constant. Not louder, just... always there. Like background noise.",
                    "Le ciel scintille toutes les quelques heures maintenant. Et les chuchotements sont constants. Pas plus forts, juste... toujours l\u{00e0}. Comme un bruit de fond.",
                ),
                LocalizedString::new(
                    "I saw a group of changed ones in the distance. They were walking in a perfect circle. Round and round. It was the most disturbing thing I've ever seen.",
                    "J'ai vu un groupe de chang\u{00e9}s au loin. Ils marchaient en cercle parfait. Encore et encore. C'\u{00e9}tait la chose la plus perturbante que j'ai jamais vue.",
                ),
                LocalizedString::new(
                    "I'm scared. But I'm not stopping.",
                    "J'ai peur. Mais je m'arr\u{00ea}te pas.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "You're the bravest person I know. Keep going.",
                        "T'es la personne la plus courageuse que je connaisse. Continue.",
                    ),
                    next_node: "a3_end".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("morale".to_string(), 2), ("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Stay away from the changed ones. Don't let them see you.",
                        "Reste loin des chang\u{00e9}s. Les laisse pas te voir.",
                    ),
                    next_node: "a3_end".to_string(),
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

    // === END OF ACT 3 ===
    add_node(
        nodes,
        StoryNode {
            id: "a3_end".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I need to rest. Tomorrow I should reach Helios. This time I'm going in with a plan.",
                    "Faut que je me repose. Demain je devrais atteindre Helios. Cette fois j'y vais avec un plan.",
                ),
                LocalizedString::new(
                    "Talk tomorrow. I hope.",
                    "On se parle demain. J'esp\u{00e8}re.",
                ),
            ],
            choices: vec![],
            next_node: Some("a4_arrival".to_string()),
            delay: Some(300),
            ending: None,
            trust_refusal: None,
        },
    );
}

// ── ACT 4 — The Crisis (Days 8-10) ───────────────────────────

fn build_act4(nodes: &mut HashMap<String, StoryNode>) {
    // === DAY 8 — Return to Helios ===
    add_node(
        nodes,
        StoryNode {
            id: "a4_arrival".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm here. Helios. It looks worse than when I left. The walls are cracked, there's that grey dust everywhere, and the air feels heavy.",
                    "J'y suis. Helios. C'est pire que quand je suis partie. Les murs sont fissur\u{00e9}s, y'a cette poussi\u{00e8}re grise partout, et l'air est lourd.",
                ),
                LocalizedString::new(
                    "The sky above the facility is... wrong. It's darker here. Like a permanent eclipse. And I can see something shimmering above the main building. Like a heat haze, but vertical.",
                    "Le ciel au-dessus du centre est... pas normal. C'est plus sombre ici. Comme une \u{00e9}clipse permanente. Et je vois un truc qui miroite au-dessus du b\u{00e2}timent principal. Comme un mirage, mais vertical.",
                ),
                LocalizedString::new(
                    "The whispers are loud here. Not painful, just... insistent. Like they're trying to tell me something.",
                    "Les chuchotements sont forts ici. Pas douloureux, juste... insistants. Comme s'ils essayaient de me dire quelque chose.",
                ),
            ],
            choices: vec![],
            next_node: Some("a4_enter_helios".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a4_enter_helios".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm going in. The main entrance is still barricaded from when I left. Good — means nothing got in.",
                    "J'entre. L'entr\u{00e9}e principale est encore barricad\u{00e9}e depuis mon d\u{00e9}part. Bon \u{2014} \u{00e7}a veut dire que rien est entr\u{00e9}.",
                ),
                LocalizedString::new(
                    "I can feel the vibration through the floor now. It's coming from below. From the MIROIR lab.",
                    "Je sens la vibration \u{00e0} travers le sol maintenant. \u{00c7}a vient d'en bas. Du labo MIROIR.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Head straight for the MIROIR door. Use the keycard.",
                        "Va direct \u{00e0} la porte MIROIR. Utilise le badge.",
                    ),
                    next_node: "a4_miroir_door".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![Condition::FlagSet("has_lab_keycard".to_string())],
                },
                Choice {
                    label: LocalizedString::new(
                        "Search for supplies first. You might need them.",
                        "Cherche des provisions d'abord. T'en auras peut-\u{00ea}tre besoin.",
                    ),
                    next_node: "a4_search_first".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("supplies".to_string(), 1)],
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
            id: "a4_search_first".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Found a few more things in the storage room. Extra batteries for the flashlight. That's huge — I'm going to need light down there.",
                    "Trouv\u{00e9} quelques trucs en plus dans la salle de stockage. Des piles suppl\u{00e9}mentaires pour la lampe. C'est \u{00e9}norme \u{2014} j'aurai besoin de lumi\u{00e8}re l\u{00e0}-dessous.",
                ),
                LocalizedString::new(
                    "Also found a note taped to the inside of a locker. It says: 'If anyone finds this — DO NOT open the resonance chamber. The rift cannot be closed from outside. You'd need to—' And then it's torn. Great.",
                    "J'ai aussi trouv\u{00e9} un mot scotch\u{00e9} dans un casier. Il dit : \u{00ab} Si quelqu'un trouve ceci \u{2014} N'OUVREZ PAS la chambre de r\u{00e9}sonance. La br\u{00e8}che ne peut pas \u{00ea}tre ferm\u{00e9}e de l'ext\u{00e9}rieur. Il faudrait\u{2014} \u{00bb} Et puis c'est d\u{00e9}chir\u{00e9}. Super.",
                ),
            ],
            choices: vec![],
            next_node: Some("a4_miroir_door".to_string()),
            delay: Some(120),
            ending: None,
            trust_refusal: None,
        },
    );

    // === The MIROIR door ===
    add_node(
        nodes,
        StoryNode {
            id: "a4_miroir_door".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm at the door. The keycard reader is still powered. Green light. Deep breath.",
                    "J'suis devant la porte. Le lecteur de badge est toujours aliment\u{00e9}. Lumi\u{00e8}re verte. Grande inspiration.",
                ),
                LocalizedString::new(
                    "It worked. The door opened.",
                    "\u{00c7}a a march\u{00e9}. La porte s'est ouverte.",
                ),
                LocalizedString::new(
                    "Oh god.",
                    "Oh mon dieu.",
                ),
                LocalizedString::new(
                    "The room beyond is... it's enormous. Way bigger than it should be. The ceiling is impossibly high. And in the center there's this... thing. A column of light. But it's not really light. It's more like a tear in the air itself.",
                    "La salle derri\u{00e8}re est... elle est \u{00e9}norme. Bien plus grande qu'elle devrait. Le plafond est impossiblement haut. Et au centre y'a ce... truc. Une colonne de lumi\u{00e8}re. Mais c'est pas vraiment de la lumi\u{00e8}re. C'est plut\u{00f4}t comme une d\u{00e9}chirure dans l'air.",
                ),
                LocalizedString::new(
                    "That's it. That's the rift. That's what's causing the Eshara.",
                    "C'est \u{00e7}a. C'est la br\u{00e8}che. C'est ce qui cause l'Eshara.",
                ),
            ],
            choices: vec![],
            next_node: Some("a4_console".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a4_console".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I can see the control console. It's on the far side of the chamber, past the rift. The screens are still active, showing data I don't understand.",
                    "Je vois la console de contr\u{00f4}le. Elle est de l'autre c\u{00f4}t\u{00e9} de la salle, au-del\u{00e0} de la br\u{00e8}che. Les \u{00e9}crans sont encore actifs, avec des donn\u{00e9}es que je comprends pas.",
                ),
                LocalizedString::new(
                    "But there's a problem. The rift is between me and the console. I'd have to walk past it. Close to it.",
                    "Mais y'a un probl\u{00e8}me. La br\u{00e8}che est entre moi et la console. Faudrait que je passe \u{00e0} c\u{00f4}t\u{00e9}. Pr\u{00e8}s d'elle.",
                ),
                LocalizedString::new(
                    "I can feel it pulling at me. Not physically — it's like my thoughts are being tugged toward it. Like it wants me to look inside.",
                    "Je la sens qui m'attire. Pas physiquement \u{2014} c'est comme si mes pens\u{00e9}es \u{00e9}taient tir\u{00e9}es vers elle. Comme si elle voulait que je regarde \u{00e0} l'int\u{00e9}rieur.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Don't look at it. Eyes on the console. Just get there.",
                        "La regarde pas. Les yeux sur la console. Avance juste.",
                    ),
                    next_node: "a4_approach_console".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![("trust_level".to_string(), 1)],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Look into the rift. Maybe it will show you something useful.",
                        "Regarde dans la br\u{00e8}che. Elle te montrera peut-\u{00ea}tre quelque chose d'utile.",
                    ),
                    next_node: "a4_look_into_rift".to_string(),
                    flags_set: vec!["looked_into_rift".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![("health".to_string(), -2)],
                    conditions: vec![],
                },
            ],
            next_node: None,
            delay: None,
            ending: None,
            // Third trust refusal: if trust very low, Elara looks anyway
            trust_refusal: Some(TrustRefusal {
                min_trust: 4,
                refusal_node: "a4_look_into_rift".to_string(),
                refusal_message: LocalizedString::new(
                    "I know you said not to look but... I can't help it. It's like it's calling my name. I just need to see—",
                    "Je sais que tu m'as dit de pas regarder mais... je peux pas m'en emp\u{00ea}cher. C'est comme si \u{00e7}a appelait mon nom. Faut juste que je voie\u{2014}",
                ),
            }),
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a4_look_into_rift".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I looked.",
                    "J'ai regard\u{00e9}.",
                ),
                LocalizedString::new(
                    "I saw... I don't know how to describe it. Another place. Like here, but reflected. Twisted. The buildings were the same but made of something organic. Alive.",
                    "J'ai vu... je sais pas comment le d\u{00e9}crire. Un autre endroit. Comme ici, mais refl\u{00e9}t\u{00e9}. Tordu. Les b\u{00e2}timents \u{00e9}taient les m\u{00ea}mes mais faits de quelque chose d'organique. Vivant.",
                ),
                LocalizedString::new(
                    "And the people. The vanished people. They're there. On the other side. Standing. Waiting.",
                    "Et les gens. Les gens disparus. Ils sont l\u{00e0}. De l'autre c\u{00f4}t\u{00e9}. Debout. En attente.",
                ),
                LocalizedString::new(
                    "I feel dizzy. My nose is bleeding. But I know something now that I didn't before.",
                    "J'ai des vertiges. Mon nez saigne. Mais je sais quelque chose maintenant que je savais pas avant.",
                ),
            ],
            choices: vec![],
            next_node: Some("a4_approach_console".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a4_approach_console".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm at the console. My hands are shaking. The screens show the resonance frequency, power levels, cascade status.",
                    "J'suis \u{00e0} la console. Mes mains tremblent. Les \u{00e9}crans montrent la fr\u{00e9}quence de r\u{00e9}sonance, les niveaux d'\u{00e9}nergie, le statut de la cascade.",
                ),
                LocalizedString::new(
                    "There's a shutdown sequence. But there's a warning: 'Shutdown will collapse the rift. Estimated collapse radius: 200 meters. All personnel must evacuate before initiation.'",
                    "Y'a une s\u{00e9}quence d'arr\u{00ea}t. Mais y'a un avertissement : \u{00ab} L'arr\u{00ea}t provoquera l'effondrement de la br\u{00e8}che. Rayon d'effondrement estim\u{00e9} : 200 m\u{00e8}tres. Tout le personnel doit \u{00e9}vacuer avant le d\u{00e9}clenchement. \u{00bb}",
                ),
                LocalizedString::new(
                    "200 meters. I'm maybe 10 meters from the rift right now. This room is underground. Getting out in time...",
                    "200 m\u{00e8}tres. J'suis \u{00e0} peut-\u{00ea}tre 10 m\u{00e8}tres de la br\u{00e8}che. Cette salle est souterraine. Sortir \u{00e0} temps...",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Set a timer on the shutdown. Give yourself time to run.",
                        "Mets un minuteur sur l'arr\u{00ea}t. Donne-toi le temps de courir.",
                    ),
                    next_node: "a4_timed_shutdown".to_string(),
                    flags_set: vec!["timed_shutdown".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Is there another way? Can the rift be sealed without the collapse?",
                        "Y'a un autre moyen ? On peut sceller la br\u{00e8}che sans l'effondrement ?",
                    ),
                    next_node: "a4_alternative".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Do it now. Hit the button. Saving the world is worth the risk.",
                        "Fais-le maintenant. Appuie sur le bouton. Sauver le monde vaut le risque.",
                    ),
                    next_node: "a4_immediate_shutdown".to_string(),
                    flags_set: vec!["sacrifice_made".to_string()],
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
            id: "a4_timed_shutdown".to_string(),
            messages: vec![
                LocalizedString::new(
                    "The system has a delay option. I can set it for 5 minutes. That might be enough to get out of the building. Maybe.",
                    "Le syst\u{00e8}me a une option de d\u{00e9}lai. Je peux le r\u{00e9}gler sur 5 minutes. \u{00c7}a pourrait suffire pour sortir du b\u{00e2}timent. Peut-\u{00ea}tre.",
                ),
                LocalizedString::new(
                    "Setting the timer. Five minutes. Starting... now.",
                    "Je r\u{00e8}gle le minuteur. Cinq minutes. C'est parti... maintenant.",
                ),
                LocalizedString::new(
                    "RUN.",
                    "COURS.",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_escape_run".to_string()),
            delay: Some(120),
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a4_alternative".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm looking through the console... there's a second option. 'Resonance Inversion.' It says it would reverse the cascade instead of shutting it down.",
                    "Je cherche dans la console... y'a une deuxi\u{00e8}me option. \u{00ab} Inversion de r\u{00e9}sonance. \u{00bb} \u{00c7}a dit que \u{00e7}a inverserait la cascade au lieu de l'arr\u{00ea}ter.",
                ),
                LocalizedString::new(
                    "But the warning says: 'Inversion requires manual calibration from within the resonance field. Operator exposure to rift energy will be significant and potentially fatal.'",
                    "Mais l'avertissement dit : \u{00ab} L'inversion n\u{00e9}cessite un calibrage manuel depuis l'int\u{00e9}rieur du champ de r\u{00e9}sonance. L'exposition de l'op\u{00e9}rateur \u{00e0} l'\u{00e9}nergie de la br\u{00e8}che sera significative et potentiellement fatale. \u{00bb}",
                ),
                LocalizedString::new(
                    "So my choices are: run and hope the timed shutdown works, or step into the rift field and try to reverse it myself. If I reverse it, the vanished people might come back. But I might not survive it.",
                    "Alors mes choix sont : courir et esp\u{00e9}rer que l'arr\u{00ea}t minuteur marche, ou entrer dans le champ de la br\u{00e8}che et essayer d'inverser moi-m\u{00ea}me. Si j'inverse, les gens disparus pourraient revenir. Mais je survivrai peut-\u{00ea}tre pas.",
                ),
            ],
            choices: vec![
                Choice {
                    label: LocalizedString::new(
                        "Set the timer and run. Your life matters too.",
                        "Mets le minuteur et cours. Ta vie compte aussi.",
                    ),
                    next_node: "a4_timed_shutdown".to_string(),
                    flags_set: vec!["timed_shutdown".to_string()],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
                Choice {
                    label: LocalizedString::new(
                        "Try the inversion. If there's a chance to bring people back...",
                        "Tente l'inversion. S'il y a une chance de ramener les gens...",
                    ),
                    next_node: "a4_inversion".to_string(),
                    flags_set: vec!["sacrifice_made".to_string(), "attempted_inversion".to_string()],
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
            id: "a4_immediate_shutdown".to_string(),
            messages: vec![
                LocalizedString::new(
                    "You're right. No time to waste. Every second this thing runs, more people suffer.",
                    "T'as raison. Pas de temps \u{00e0} perdre. Chaque seconde o\u{00f9} ce truc tourne, plus de gens souffrent.",
                ),
                LocalizedString::new(
                    "I'm initiating the shutdown sequence. The console is vibrating. The rift is reacting — it's getting brighter.",
                    "Je lance la s\u{00e9}quence d'arr\u{00ea}t. La console vibre. La br\u{00e8}che r\u{00e9}agit \u{2014} elle devient plus brillante.",
                ),
                LocalizedString::new(
                    "Running. I'm running as fast as I can.",
                    "Je cours. Je cours aussi vite que je peux.",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_escape_run".to_string()),
            delay: Some(60),
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a4_inversion".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Okay. I'm doing it. I'm stepping into the resonance field.",
                    "Bon. Je le fais. J'entre dans le champ de r\u{00e9}sonance.",
                ),
                LocalizedString::new(
                    "It feels like... like being everywhere at once. I can feel the other side. I can feel them. All the vanished people. They're reaching out.",
                    "\u{00c7}a fait comme... comme \u{00ea}tre partout en m\u{00ea}me temps. Je sens l'autre c\u{00f4}t\u{00e9}. Je les sens. Tous les gens disparus. Ils tendent les mains.",
                ),
                LocalizedString::new(
                    "I'm calibrating the inversion. My hands are moving but I don't think it's entirely me controlling them. Something is guiding me.",
                    "Je calibre l'inversion. Mes mains bougent mais je crois que c'est pas enti\u{00e8}rement moi qui les contr\u{00f4}le. Quelque chose me guide.",
                ),
                LocalizedString::new(
                    "The light is blinding. I can't—",
                    "La lumi\u{00e8}re est aveuglante. Je peux pas\u{2014}",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_inversion_result".to_string()),
            delay: Some(60),
            ending: None,
            trust_refusal: None,
        },
    );
}

// ── ACT 5 — The End (Days 11-13) ─────────────────────────────

fn build_act5(nodes: &mut HashMap<String, StoryNode>) {
    use crate::story::EndingType;

    // === PATH A: Escape after shutdown (timed or immediate) ===
    add_node(
        nodes,
        StoryNode {
            id: "a5_escape_run".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm running. The building is shaking. Chunks of ceiling are falling. The rift is screaming — a sound I'll never forget.",
                    "Je cours. Le b\u{00e2}timent tremble. Des morceaux de plafond tombent. La br\u{00e8}che hurle \u{2014} un son que j'oublierai jamais.",
                ),
                LocalizedString::new(
                    "Stairs. Up. Up. My lungs are burning. I can hear the collapse starting behind me.",
                    "Escaliers. En haut. En haut. Mes poumons br\u{00fb}lent. J'entends l'effondrement commencer derri\u{00e8}re moi.",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_escape_outcome".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a5_escape_outcome".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I made it outside. Barely. The ground caved in behind me. The whole facility just... collapsed into itself.",
                    "J'ai r\u{00e9}ussi \u{00e0} sortir. De justesse. Le sol s'est effondr\u{00e9} derri\u{00e8}re moi. Tout le centre s'est juste... effondr\u{00e9} sur lui-m\u{00ea}me.",
                ),
                LocalizedString::new(
                    "The sky... the sky is clearing. The permanent twilight is lifting. I can see blue. Actual blue sky.",
                    "Le ciel... le ciel s'\u{00e9}claircit. Le cr\u{00e9}puscule permanent se l\u{00e8}ve. Je vois du bleu. Du vrai ciel bleu.",
                ),
            ],
            choices: vec![
                // If looked into rift -> Eshara wins path
                Choice {
                    label: LocalizedString::new("...", "..."),
                    next_node: "a5_eshara_buildup".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![
                        Condition::FlagSet("looked_into_rift".to_string()),
                        Condition::StatBelow("morale".to_string(), 4),
                    ],
                },
                // If health critically low -> Gone Dark
                Choice {
                    label: LocalizedString::new("...", "..."),
                    next_node: "a5_gone_dark_buildup".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![Condition::StatBelow("health".to_string(), 4)],
                },
                // Default: New Dawn
                Choice {
                    label: LocalizedString::new("...", "..."),
                    next_node: "a5_escape_new_dawn".to_string(),
                    flags_set: vec![],
                    flags_remove: vec![],
                    stat_changes: vec![],
                    conditions: vec![],
                },
            ],
            // Auto-select first available choice (engine picks the first matching)
            next_node: Some("a5_escape_new_dawn".to_string()), // Fallback
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // New Dawn continuation after escape
    add_node(
        nodes,
        StoryNode {
            id: "a5_escape_new_dawn".to_string(),
            messages: vec![LocalizedString::new(
                "I'm sitting on a hill, watching the world come back to life. And I'm crying. I'm crying and I can't stop.",
                "J'suis assise sur une colline, \u{00e0} regarder le monde revenir \u{00e0} la vie. Et je pleure. Je pleure et j'arrive pas \u{00e0} m'arr\u{00ea}ter.",
            )],
            choices: vec![],
            next_node: Some("a5_ending_new_dawn".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === ENDING 1: New Dawn (Good) ===
    add_node(
        nodes,
        StoryNode {
            id: "a5_ending_new_dawn".to_string(),
            messages: vec![
                LocalizedString::new(
                    "The signal from the settlement is still there. I'm heading back. I'm going home. Well... the closest thing to home I have now.",
                    "Le signal de la colonie est toujours l\u{00e0}. J'y retourne. Je rentre chez moi. Enfin... ce qui s'en rapproche le plus maintenant.",
                ),
                LocalizedString::new(
                    "The phenomena have stopped. No more flickering. No more whispers. The changed ones... I saw one on my way back. She was just standing there, confused. Human again. Scared, but human.",
                    "Les ph\u{00e9}nom\u{00e8}nes ont cess\u{00e9}. Plus de scintillement. Plus de chuchotements. Les chang\u{00e9}s... j'en ai vu une sur le chemin du retour. Elle \u{00e9}tait l\u{00e0}, confuse. Humaine \u{00e0} nouveau. Effray\u{00e9}e, mais humaine.",
                ),
                LocalizedString::new(
                    "I don't know if the vanished will come back. But the world is healing. And I'm alive.",
                    "J'sais pas si les disparus reviendront. Mais le monde gu\u{00e9}rit. Et j'suis vivante.",
                ),
                LocalizedString::new(
                    "Thank you. For everything. I wouldn't be here without you. Literally.",
                    "Merci. Pour tout. Je serais pas l\u{00e0} sans toi. Litt\u{00e9}ralement.",
                ),
                LocalizedString::new(
                    "This is the start of something new. A new dawn. And I'm ready for it.",
                    "C'est le d\u{00e9}but de quelque chose de nouveau. Une aube nouvelle. Et j'suis pr\u{00ea}te.",
                ),
            ],
            choices: vec![],
            next_node: None,
            delay: None,
            ending: Some(EndingType::NewDawn),
            trust_refusal: None,
        },
    );

    // === PATH B: Inversion result ===
    add_node(
        nodes,
        StoryNode {
            id: "a5_inversion_result".to_string(),
            messages: vec![
                LocalizedString::new(
                    "...",
                    "...",
                ),
                LocalizedString::new(
                    "I'm... I'm still here. I think. The light is gone. The rift is closed. The chamber is dark and silent.",
                    "J'suis... j'suis encore l\u{00e0}. Je crois. La lumi\u{00e8}re est partie. La br\u{00e8}che est ferm\u{00e9}e. La salle est sombre et silencieuse.",
                ),
                LocalizedString::new(
                    "I can't move well. Everything hurts. My vision is blurred and there's this ringing in my ears that won't stop.",
                    "J'arrive pas bien \u{00e0} bouger. Tout me fait mal. Ma vision est floue et y'a cet acouph\u{00e8}ne qui s'arr\u{00ea}te pas.",
                ),
                LocalizedString::new(
                    "But outside... I can hear voices. Real voices. Not whispers. People. Confused, scared, but alive. The vanished — they're coming back.",
                    "Mais dehors... j'entends des voix. De vraies voix. Pas des chuchotements. Des gens. Confus, effray\u{00e9}s, mais vivants. Les disparus \u{2014} ils reviennent.",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_ending_signal".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    // === ENDING 2: The Signal (Good — but at cost) ===
    add_node(
        nodes,
        StoryNode {
            id: "a5_ending_signal".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I did it. The inversion worked. The Eshara is reversing. People are coming back.",
                    "Je l'ai fait. L'inversion a march\u{00e9}. L'Eshara s'inverse. Les gens reviennent.",
                ),
                LocalizedString::new(
                    "But I'm... I'm not okay. The exposure changed something in me. I can see both worlds now. Layered on top of each other. It's beautiful and horrible.",
                    "Mais je... j'suis pas bien. L'exposition a chang\u{00e9} quelque chose en moi. Je vois les deux mondes maintenant. Superpos\u{00e9}s. C'est beau et horrible.",
                ),
                LocalizedString::new(
                    "Dr. Osei says I might recover. Or I might not. Time will tell.",
                    "Le Dr Osei dit que je pourrais gu\u{00e9}rir. Ou pas. Le temps dira.",
                ),
                LocalizedString::new(
                    "But the world is healing. That's what matters. The signal worked. I was the signal.",
                    "Mais le monde gu\u{00e9}rit. C'est ce qui compte. Le signal a march\u{00e9}. J'\u{00e9}tais le signal.",
                ),
                LocalizedString::new(
                    "Thank you for being my anchor. Through all of it. You kept me human when everything else tried not to.",
                    "Merci d'avoir \u{00e9}t\u{00e9} mon ancre. \u{00c0} travers tout \u{00e7}a. Tu m'as gard\u{00e9}e humaine quand tout le reste essayait de pas l'\u{00ea}tre.",
                ),
            ],
            choices: vec![],
            next_node: None,
            delay: None,
            ending: Some(EndingType::TheSignal),
            trust_refusal: None,
        },
    );

    // === ENDING 3: Static (Bittersweet — for players who stayed safe) ===
    // Reached via a3_stay_safe path — need a connecting node
    // If player stayed safe and never got the keycard, the cascade eventually
    // destabilizes on its own, damaging the radio
    add_node(
        nodes,
        StoryNode {
            id: "a5_static_buildup".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Days passed. The phenomena got worse. The sky was flickering constantly. The changed ones were everywhere.",
                    "Les jours ont pass\u{00e9}. Les ph\u{00e9}nom\u{00e8}nes ont empir\u{00e9}. Le ciel scintillait sans arr\u{00ea}t. Les chang\u{00e9}s \u{00e9}taient partout.",
                ),
                LocalizedString::new(
                    "Then one night... everything just stopped. A massive flash, and then silence. The cascade burned itself out. Dr. Osei had been right — it was possible, but it took weeks more of suffering.",
                    "Puis une nuit... tout s'est juste arr\u{00ea}t\u{00e9}. Un flash massif, puis le silence. La cascade s'est \u{00e9}puis\u{00e9}e. Le Dr Osei avait raison \u{2014} c'\u{00e9}tait possible, mais \u{00e7}a a pris des semaines de souffrance en plus.",
                ),
                LocalizedString::new(
                    "But the flash fried the radio. This device. The one connecting us.",
                    "Mais le flash a grill\u{00e9} la radio. Cet appareil. Celui qui nous connecte.",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_ending_static".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a5_ending_static".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm alive. The settlement survived. We're rebuilding.",
                    "J'suis vivante. La colonie a surv\u{00e9}cu. On reconstruit.",
                ),
                LocalizedString::new(
                    "But this radio is dying. I can hear the static getting louder. I don't know how much longer we have.",
                    "Mais cette radio meurt. J'entends le gr\u{00e9}sillement qui monte. J'sais pas combien de temps il nous reste.",
                ),
                LocalizedString::new(
                    "Thank you. For everything. I'll be okay. I think.",
                    "Merci. Pour tout. \u{00c7}a va aller. Je crois.",
                ),
                LocalizedString::new(
                    "If you can hear this... if anyone can hear this... we're still here. We made it.",
                    "Si tu m'entends... si quelqu'un m'entend... on est toujours l\u{00e0}. On a surv\u{00e9}cu.",
                ),
                LocalizedString::new(
                    "* krrzzz... *",
                    "* krrzzz... *",
                ),
            ],
            choices: vec![],
            next_node: None,
            delay: None,
            ending: Some(EndingType::Static),
            trust_refusal: None,
        },
    );

    // === ENDING 4: Gone Dark (Bad — Elara doesn't survive) ===
    // Reached when health is too low + dangerous choices
    add_node(
        nodes,
        StoryNode {
            id: "a5_gone_dark_buildup".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm... I'm not doing well. The escape took everything I had. I'm bleeding. I think something inside is broken.",
                    "Je... je vais pas bien. La fuite m'a pris tout ce que j'avais. Je saigne. Je crois qu'un truc \u{00e0} l'int\u{00e9}rieur est cass\u{00e9}.",
                ),
                LocalizedString::new(
                    "I made it out of the building but I can't walk anymore. I'm sitting against a tree, watching the sky clear.",
                    "J'ai r\u{00e9}ussi \u{00e0} sortir du b\u{00e2}timent mais je peux plus marcher. J'suis assise contre un arbre, \u{00e0} regarder le ciel s'\u{00e9}claircir.",
                ),
                LocalizedString::new(
                    "At least it worked. The phenomena are stopping. That's something.",
                    "Au moins \u{00e7}a a march\u{00e9}. Les ph\u{00e9}nom\u{00e8}nes s'arr\u{00ea}tent. C'est d\u{00e9}j\u{00e0} \u{00e7}a.",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_ending_gone_dark".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a5_ending_gone_dark".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I'm tired. So tired.",
                    "J'suis fatigu\u{00e9}e. Tellement fatigu\u{00e9}e.",
                ),
                LocalizedString::new(
                    "Thank you for being with me. Through all of this. You made me brave.",
                    "Merci d'avoir \u{00e9}t\u{00e9} avec moi. \u{00c0} travers tout \u{00e7}a. Tu m'as rendue courageuse.",
                ),
                LocalizedString::new(
                    "I'm going to close my eyes. Just for a bit.",
                    "Je vais fermer les yeux. Juste un peu.",
                ),
                LocalizedString::new(
                    "The sky really is beautiful today.",
                    "Le ciel est vraiment beau aujourd'hui.",
                ),
                LocalizedString::new(
                    "...",
                    "...",
                ),
            ],
            choices: vec![],
            next_node: None,
            delay: None,
            ending: Some(EndingType::GoneDark),
            trust_refusal: None,
        },
    );

    // === ENDING 5: The Eshara Wins (Bad — Elara is consumed) ===
    // Reached when she looked into the rift + low morale
    add_node(
        nodes,
        StoryNode {
            id: "a5_eshara_buildup".to_string(),
            messages: vec![
                LocalizedString::new(
                    "Something's wrong. Since I looked into the rift, I feel... different. The whispers aren't background noise anymore. They're words. I can understand them.",
                    "Quelque chose va pas. Depuis que j'ai regard\u{00e9} dans la br\u{00e8}che, je me sens... diff\u{00e9}rente. Les chuchotements sont plus du bruit de fond. Ce sont des mots. Je les comprends.",
                ),
                LocalizedString::new(
                    "They're saying my name. Over and over. And it feels... warm. Welcoming. Like coming home.",
                    "Ils disent mon nom. Encore et encore. Et c'est... chaud. Accueillant. Comme rentrer chez soi.",
                ),
                LocalizedString::new(
                    "I don't think I'm afraid anymore. Is that good?",
                    "Je crois que j'ai plus peur. C'est bien \u{00e7}a ?",
                ),
            ],
            choices: vec![],
            next_node: Some("a5_ending_eshara_wins".to_string()),
            delay: None,
            ending: None,
            trust_refusal: None,
        },
    );

    add_node(
        nodes,
        StoryNode {
            id: "a5_ending_eshara_wins".to_string(),
            messages: vec![
                LocalizedString::new(
                    "I can see it all now. Both sides. The membrane between worlds is so thin. So beautiful.",
                    "Je vois tout maintenant. Les deux c\u{00f4}t\u{00e9}s. La membrane entre les mondes est si fine. Si belle.",
                ),
                LocalizedString::new(
                    "You should see what I see. You should feel what I feel. It's not an ending. It's a beginning.",
                    "Tu devrais voir ce que je vois. Tu devrais sentir ce que je sens. C'est pas une fin. C'est un d\u{00e9}but.",
                ),
                LocalizedString::new(
                    "The Eshara isn't a catastrophe. It's an invitation.",
                    "L'Eshara c'est pas une catastrophe. C'est une invitation.",
                ),
                LocalizedString::new(
                    "Come with me. Come with me. Come with me. Come with me. Come with me.",
                    "Viens avec moi. Viens avec moi. Viens avec moi. Viens avec moi. Viens avec moi.",
                ),
                LocalizedString::new(
                    "* signal terminated *",
                    "* signal termin\u{00e9} *",
                ),
            ],
            choices: vec![],
            next_node: None,
            delay: None,
            ending: Some(EndingType::TheEsharaWins),
            trust_refusal: None,
        },
    );

    // === ROUTING NOTES ===
    // a5_escape_outcome uses conditional "..." choices to auto-route:
    //   - looked_into_rift + low morale -> Eshara Wins
    //   - low health -> Gone Dark
    //   - default -> New Dawn
    // a3_stayed_safe_aftermath routes the stay-safe path to a5_static_buildup -> Static ending
    // The inversion path goes: a4_inversion -> a5_inversion_result -> a5_ending_signal
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

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
        // All acts combined should have at least 10 nodes
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
    fn test_act3_nodes_exist() {
        let tree = build_story_tree();
        assert!(tree.contains_key("a3_morning"));
        assert!(tree.contains_key("a3_osei_talk"));
        assert!(tree.contains_key("a3_keycard_decision"));
        assert!(tree.contains_key("a3_kai_twist"));
        assert!(tree.contains_key("a3_end"));
    }

    #[test]
    fn test_act3_has_conditional_choices() {
        let tree = build_story_tree();
        let confrontation = tree.get("a3_kai_confrontation").unwrap();
        // "Talk to Kai" requires kai_ally flag
        let talk_choice = &confrontation.choices[0];
        assert!(!talk_choice.conditions.is_empty());
    }

    #[test]
    fn test_act4_nodes_exist() {
        let tree = build_story_tree();
        assert!(tree.contains_key("a4_arrival"));
        assert!(tree.contains_key("a4_miroir_door"));
        assert!(tree.contains_key("a4_console"));
        assert!(tree.contains_key("a4_approach_console"));
        assert!(tree.contains_key("a4_inversion"));
    }

    #[test]
    fn test_act4_has_trust_refusal() {
        let tree = build_story_tree();
        let console = tree.get("a4_console").unwrap();
        assert!(console.trust_refusal.is_some());
    }

    #[test]
    fn test_total_node_count() {
        let tree = build_story_tree();
        // Acts 1-5 should have at least 60 nodes
        assert!(
            tree.len() >= 60,
            "Expected at least 60 nodes, got {}",
            tree.len()
        );
    }

    #[test]
    fn test_act5_nodes_exist() {
        let tree = build_story_tree();
        assert!(tree.contains_key("a5_escape_run"));
        assert!(tree.contains_key("a5_escape_outcome"));
        assert!(tree.contains_key("a5_inversion_result"));
        assert!(tree.contains_key("a5_static_buildup"));
        assert!(tree.contains_key("a5_ending_new_dawn"));
        assert!(tree.contains_key("a5_ending_signal"));
        assert!(tree.contains_key("a5_ending_static"));
        assert!(tree.contains_key("a5_ending_gone_dark"));
        assert!(tree.contains_key("a5_ending_eshara_wins"));
    }

    #[test]
    fn test_act5_all_endings_present() {
        let tree = build_story_tree();
        let endings: Vec<_> = tree.values().filter(|n| n.ending.is_some()).collect();
        assert_eq!(
            endings.len(),
            5,
            "Expected 5 ending nodes, got {}",
            endings.len()
        );
    }

    #[test]
    fn test_stayed_safe_path_reaches_static_ending() {
        let tree = build_story_tree();
        // a3_stayed_safe_aftermath should exist and lead to a5_static_buildup
        let node = tree.get("a3_stayed_safe_aftermath").unwrap();
        assert_eq!(node.next_node.as_deref(), Some("a5_static_buildup"));
        // a5_static_buildup leads to a5_ending_static
        let buildup = tree.get("a5_static_buildup").unwrap();
        assert_eq!(buildup.next_node.as_deref(), Some("a5_ending_static"));
    }

    #[test]
    fn test_escape_outcome_has_conditional_routing() {
        let tree = build_story_tree();
        let node = tree.get("a5_escape_outcome").unwrap();
        assert_eq!(
            node.choices.len(),
            3,
            "a5_escape_outcome should have 3 conditional choices"
        );
        // All choices should have "..." labels (auto-route)
        for choice in &node.choices {
            assert_eq!(choice.label.get(Language::En), "...");
        }
    }
}
