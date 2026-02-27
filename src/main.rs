mod game;
mod i18n;
mod story;
mod time;
mod ui;

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Utc;

use game::{
    delete_save, load_game, parse_cli_args, save_exists, save_game, GameState, LogEntry, Sender,
};
use i18n::{sys_msg, Language, Msg};
use story::nodes::build_story_tree;
use story::StoryNode;

/// Global flag set by the Ctrl+C handler
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Check if Ctrl+C was pressed
fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::Relaxed)
}

fn main() {
    // Install Ctrl+C handler
    let _ = ctrlc::set_handler(move || {
        INTERRUPTED.store(true, Ordering::Relaxed);
    });

    if let Err(e) = run() {
        // Don't show error for intentional interrupts
        if e.kind() != io::ErrorKind::Interrupted {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    // Ensure terminal is restored on exit
    let _ = crossterm::terminal::disable_raw_mode();
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

            // Check if Elara is still waiting
            if time::is_waiting(&existing) {
                let mut s = existing;

                // Show backlog before handling the wait
                ui::replay_backlog(&s.message_log, lang)?;

                let should_continue = time::handle_waiting(&mut s)?;
                if !should_continue {
                    // Player chose to quit and come back later
                    save_game(&s)?;
                    return Ok(());
                }

                // Add a session separator
                let now = Utc::now();
                let session_label = now.format("%Y-%m-%d %H:%M").to_string();
                s.message_log.push(LogEntry {
                    sender: Sender::System,
                    text: format!("SESSION:{}", session_label),
                    timestamp: now,
                });
                save_game(&s)?;
                s
            } else {
                // If there was a wait that's now complete, clear it
                let mut s = existing;
                if s.waiting_until.is_some() {
                    s.waiting_until = None;
                    // Bell notification — Elara is back
                    print!("\x07");
                    save_game(&s)?;
                }

                // Show backlog before the continue/new prompt
                ui::replay_backlog(&s.message_log, lang)?;

                ui::print_system_message(sys_msg(Msg::ContinueOrNew, lang))?;
                ui::print_blank()?;

                let choices = vec![
                    sys_msg(Msg::ContinueOption, lang).to_string(),
                    sys_msg(Msg::NewGameOption, lang).to_string(),
                ];
                let choice = ui::prompt_choice(&choices)?;

                if choice == 0 {
                    // Replay the backlog so the player remembers where they left off
                    ui::replay_backlog(&s.message_log, lang)?;

                    // Add a session separator to the log
                    let now = Utc::now();
                    let session_label = now.format("%Y-%m-%d %H:%M").to_string();
                    s.message_log.push(LogEntry {
                        sender: Sender::System,
                        text: format!("SESSION:{}", session_label),
                        timestamp: now,
                    });
                    save_game(&s)?;

                    s
                } else {
                    let lang = select_language(args.language)?;
                    start_new_game(lang)?
                }
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

    let mut state = GameState::new(lang);

    // Log the first session start
    let now = Utc::now();
    let session_label = now.format("%Y-%m-%d %H:%M").to_string();
    state.message_log.push(LogEntry {
        sender: Sender::System,
        text: format!("SESSION:{}", session_label),
        timestamp: now,
    });

    Ok(state)
}

/// The core game loop: process nodes, display messages, handle choices
fn game_loop(state: &mut GameState, story: &HashMap<String, StoryNode>) -> io::Result<()> {
    loop {
        // Check for Ctrl+C
        if is_interrupted() {
            handle_graceful_exit(state)?;
            break;
        }

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

        // Handle real-time delay: if this node has a delay, schedule it and break
        if let Some(delay_secs) = node.delay {
            // Determine the next node after the delay
            let next = if !node.choices.is_empty() {
                // If there are choices AND a delay, the choices were already shown above.
                // This case shouldn't normally occur — delay is for linear auto-advance nodes.
                // But handle it: just use the first choice's next_node.
                node.choices[0].next_node.clone()
            } else if let Some(ref next) = node.next_node {
                next.clone()
            } else {
                break; // dead end
            };

            state.current_node = next;
            time::schedule_wait(state, delay_secs);
            save_game(state)?;

            // Now handle the wait (show message, let player wait or quit)
            let should_continue = time::handle_waiting(state)?;
            if !should_continue {
                save_game(state)?;
                break;
            }
            save_game(state)?;
            continue;
        }

        // If there are choices, present them (with condition filtering)
        if !node.choices.is_empty() {
            // Check trust-based refusal first
            if node.should_refuse(state) {
                let refusal = node.trust_refusal.as_ref().unwrap();
                let refusal_text = refusal.refusal_message.get(lang);

                // Show Elara's refusal
                ui::elara_says(refusal_text, lang)?;
                state.message_log.push(LogEntry {
                    sender: Sender::Elara,
                    text: refusal_text.to_string(),
                    timestamp: Utc::now(),
                });

                // Go to the refusal node instead
                state.current_node = refusal.refusal_node.clone();
                save_game(state)?;
                continue;
            }

            // Filter choices by conditions
            let available: Vec<(usize, &story::Choice)> = node.available_choices(state);

            if available.is_empty() {
                // All choices are gated — fall through to next_node if available
                if let Some(ref next) = node.next_node {
                    state.current_node = next.clone();
                    save_game(state)?;
                    continue;
                } else {
                    break; // dead end
                }
            }

            let choice_labels: Vec<String> = available
                .iter()
                .map(|(_, c)| c.label.get(lang).to_string())
                .collect();

            // Auto-route: if all available choices have the label "...", this is
            // a conditional routing node (not a real player decision). Auto-select
            // the first available choice silently.
            let is_auto_route = choice_labels.iter().all(|l| l == "...");

            let (chosen_display_idx, chosen) = if is_auto_route {
                (0, available[0].1)
            } else {
                let idx = ui::prompt_choice(&choice_labels)?;
                (idx, available[idx].1)
            };

            if !is_auto_route {
                // Show the player's choice in the chat
                ui::print_player_choice(&choice_labels[chosen_display_idx])?;
                ui::print_blank()?;

                // Log the player's choice
                state.message_log.push(LogEntry {
                    sender: Sender::Player,
                    text: choice_labels[chosen_display_idx].clone(),
                    timestamp: Utc::now(),
                });
            }

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

/// Handle graceful exit on Ctrl+C: save, show message, restore terminal
fn handle_graceful_exit(state: &mut GameState) -> io::Result<()> {
    // Restore terminal state (in case we were in raw mode)
    let _ = crossterm::terminal::disable_raw_mode();

    let lang = state.language;

    // Auto-save
    save_game(state)?;

    // Show atmospheric "signal lost" message
    let mut stdout = io::stdout();
    writeln!(stdout)?;
    ui::print_blank()?;
    ui::print_separator(None)?;
    ui::print_blank()?;
    ui::print_system_message(sys_msg(Msg::SignalLost, lang))?;
    ui::print_blank()?;

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
