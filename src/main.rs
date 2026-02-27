mod game;
mod i18n;
mod story;
mod time;
mod ui;

use std::collections::HashMap;
use std::io;

use chrono::Utc;

use game::{
    delete_save, load_game, parse_cli_args, save_exists, save_game, GameState, LogEntry, Sender,
};
use i18n::{sys_msg, Language, Msg};
use story::nodes::build_story_tree;
use story::StoryNode;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let args = parse_cli_args();

    // Handle --reset
    if args.reset {
        delete_save()?;
        println!("{}", sys_msg(Msg::SaveDeleted, Language::En));
        println!("{}", sys_msg(Msg::SaveDeleted, Language::Fr));
        return Ok(());
    }

    let story = build_story_tree();

    // Check for existing save
    let mut state = if save_exists() {
        if let Some(existing) = load_game()? {
            let lang = existing.language;
            ui::print_banner()?;
            ui::print_system_message(sys_msg(Msg::ContinueOrNew, lang))?;
            ui::print_blank()?;

            let choices = vec![
                sys_msg(Msg::ContinueOption, lang).to_string(),
                sys_msg(Msg::NewGameOption, lang).to_string(),
            ];
            let choice = ui::prompt_choice(&choices)?;

            if choice == 0 {
                existing
            } else {
                let lang = select_language(args.language)?;
                start_new_game(lang)?
            }
        } else {
            let lang = select_language(args.language)?;
            start_new_game(lang)?
        }
    } else {
        let lang = select_language(args.language)?;
        start_new_game(lang)?
    };

    // Main game loop
    game_loop(&mut state, &story)?;

    Ok(())
}

/// Prompt the player to select a language, or use the override
fn select_language(override_lang: Option<Language>) -> io::Result<Language> {
    if let Some(lang) = override_lang {
        return Ok(lang);
    }

    ui::clear_screen()?;
    ui::print_banner()?;
    ui::print_system_message(sys_msg(Msg::LanguagePrompt, Language::En))?;
    ui::print_blank()?;

    let choices = vec![
        sys_msg(Msg::LanguageOption1, Language::En).to_string(),
        sys_msg(Msg::LanguageOption2, Language::En).to_string(),
    ];
    let choice = ui::prompt_choice(&choices)?;

    Ok(if choice == 0 {
        Language::En
    } else {
        Language::Fr
    })
}

/// Start a new game: show intro, create fresh state
fn start_new_game(lang: Language) -> io::Result<GameState> {
    ui::clear_screen()?;
    ui::print_banner()?;
    ui::print_blank()?;

    // Atmospheric intro
    ui::print_system_message_animated(sys_msg(Msg::IntroRadioCrackle, lang))?;
    ui::print_blank()?;
    ui::print_separator(None)?;
    ui::print_blank()?;

    Ok(GameState::new(lang))
}

/// The core game loop: process nodes, display messages, handle choices
fn game_loop(state: &mut GameState, story: &HashMap<String, StoryNode>) -> io::Result<()> {
    loop {
        let node = match story.get(&state.current_node) {
            Some(n) => n.clone(),
            None => {
                ui::print_system_message(&format!(
                    "Error: story node '{}' not found.",
                    state.current_node
                ))?;
                break;
            }
        };

        let lang = state.language;

        // Display all messages for this node
        for msg in &node.messages {
            let text = msg.get(lang);
            ui::elara_says(text, lang)?;

            // Log the message
            state.message_log.push(LogEntry {
                sender: Sender::Elara,
                text: text.to_string(),
                timestamp: Utc::now(),
            });
        }

        // Save after displaying messages
        save_game(state)?;

        // Check if this is an ending node
        if node.ending.is_some() {
            state.ending = node.ending.clone();
            save_game(state)?;
            show_ending_screen(state)?;
            break;
        }

        // If there are choices, present them
        if !node.choices.is_empty() {
            let choice_labels: Vec<String> = node
                .choices
                .iter()
                .map(|c| c.label.get(lang).to_string())
                .collect();

            let chosen_idx = ui::prompt_choice(&choice_labels)?;
            let chosen = &node.choices[chosen_idx];

            // Show the player's choice in the chat
            ui::print_player_choice(&choice_labels[chosen_idx])?;
            ui::print_blank()?;

            // Log the player's choice
            state.message_log.push(LogEntry {
                sender: Sender::Player,
                text: choice_labels[chosen_idx].clone(),
                timestamp: Utc::now(),
            });

            // Apply flags
            for flag in &chosen.flags_set {
                state.set_flag(flag);
            }
            for flag in &chosen.flags_remove {
                state.remove_flag(flag);
            }

            // Apply stat changes
            for (stat, delta) in &chosen.stat_changes {
                state.stats.modify(stat, *delta);
            }

            // Advance to next node
            state.current_node = chosen.next_node.clone();
            save_game(state)?;
        } else if let Some(ref next) = node.next_node {
            // Linear node: auto-advance
            state.current_node = next.clone();
            save_game(state)?;
        } else {
            // Dead end (no choices and no next_node, and not an ending)
            // This shouldn't happen in a well-formed story, but handle gracefully
            break;
        }
    }

    Ok(())
}

/// Display the ending summary screen
fn show_ending_screen(state: &GameState) -> io::Result<()> {
    let lang = state.language;

    ui::print_blank()?;
    ui::print_separator(None)?;
    ui::print_blank()?;

    if let Some(ref ending) = state.ending {
        let title = story::endings::ending_title(ending);
        let desc = story::endings::ending_description(ending);

        ui::print_system_message(&format!("--- {} ---", sys_msg(Msg::EndingReached, lang)))?;
        ui::print_blank()?;
        ui::print_system_message(&format!("\"{}\"", title.get(lang)))?;
        ui::print_blank()?;
        ui::print_system_message(desc.get(lang))?;
    }

    ui::print_blank()?;
    ui::print_system_message(&format!(
        "{} {}",
        sys_msg(Msg::DaysSurvived, lang),
        state.day
    ))?;
    ui::print_blank()?;
    ui::print_separator(None)?;
    ui::print_blank()?;

    // Play again prompt
    ui::print_system_message(sys_msg(Msg::PlayAgain, lang))?;
    let choices = vec![
        sys_msg(Msg::YesOption, lang).to_string(),
        sys_msg(Msg::NoOption, lang).to_string(),
    ];
    let choice = ui::prompt_choice(&choices)?;

    if choice == 0 {
        delete_save()?;
        // Restart
        run_fresh()?;
    } else {
        delete_save()?;
    }

    Ok(())
}

/// Run a completely fresh game (after "play again")
fn run_fresh() -> io::Result<()> {
    let story = build_story_tree();
    let lang = select_language(None)?;
    let mut state = start_new_game(lang)?;
    game_loop(&mut state, &story)?;
    Ok(())
}
