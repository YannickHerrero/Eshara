mod game;
mod i18n;
mod story;
mod time;
mod tui;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

use game::{delete_save, load_game, parse_cli_args, save_exists, GameState};
use i18n::{sys_msg, Language, Msg};
use story::nodes::build_story_tree;
use tui::{App, Screen};

/// Global flag set by the Ctrl+C handler
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Check if Ctrl+C was pressed (used by tui::run)
pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::Relaxed)
}

fn main() {
    // Install Ctrl+C handler
    let _ = ctrlc::set_handler(move || {
        INTERRUPTED.store(true, Ordering::Relaxed);
    });

    if let Err(e) = run() {
        if e.kind() != io::ErrorKind::Interrupted {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
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

    // Determine starting state and screen
    let (game_state, start_screen, prompt_options) = if save_exists() {
        if let Some(existing) = load_game()? {
            let lang = args.language.unwrap_or(existing.language);
            let mut state = existing;
            state.language = lang;

            if time::is_waiting(&state) {
                // Elara is still busy — show wait screen
                let until = state.waiting_until.unwrap();
                let remaining = time::remaining_time_str(until, lang);
                let _wait_msg = format!(
                    "{}\n\n{} (~{})",
                    sys_msg(Msg::ElaraUnavailable, lang),
                    sys_msg(Msg::ElaraBackAround, lang),
                    remaining,
                );
                let opts = vec![
                    sys_msg(Msg::WaitOption, lang).to_string(),
                    sys_msg(Msg::QuitOption, lang).to_string(),
                ];
                (state, Screen::Waiting, opts)
            } else {
                // Clear completed wait if any
                if state.waiting_until.is_some() {
                    state.waiting_until = None;
                    let _ = game::save_game(&state);
                }

                // Show continue or new game prompt
                let opts = vec![
                    sys_msg(Msg::ContinueOption, lang).to_string(),
                    sys_msg(Msg::NewGameOption, lang).to_string(),
                ];
                (state, Screen::ContinueOrNew, opts)
            }
        } else {
            // Corrupted save — start fresh
            let lang = args.language.unwrap_or(Language::En);
            let state = GameState::new(lang);
            let opts = vec![
                sys_msg(Msg::LanguageOption1, Language::En).to_string(),
                sys_msg(Msg::LanguageOption2, Language::En).to_string(),
            ];
            (state, Screen::LanguageSelect, opts)
        }
    } else {
        // No save — new game
        let lang = args.language.unwrap_or(Language::En);
        let state = GameState::new(lang);
        let opts = vec![
            sys_msg(Msg::LanguageOption1, Language::En).to_string(),
            sys_msg(Msg::LanguageOption2, Language::En).to_string(),
        ];
        (state, Screen::LanguageSelect, opts)
    };

    // Build the App
    let mut app = App::new(game_state, story);
    app.screen = start_screen.clone();
    app.prompt_options = prompt_options;

    // If resuming, load backlog into chat
    if start_screen == Screen::ContinueOrNew || start_screen == Screen::Waiting {
        app.load_backlog();
    }

    // Set up wait message if on the waiting screen
    if start_screen == Screen::Waiting {
        if let Some(until) = app.game_state.waiting_until {
            let lang = app.lang();
            let remaining = time::remaining_time_str(until, lang);
            app.wait_message = Some(format!(
                "{}\n\n{} (~{})",
                sys_msg(Msg::ElaraUnavailable, lang),
                sys_msg(Msg::ElaraBackAround, lang),
                remaining,
            ));
        }
    }

    // Initialize ratatui terminal and run
    let mut terminal = ratatui::init();
    let result = tui::run(app, &mut terminal);
    ratatui::restore();

    result
}
