use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use ratatui::crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Attribute, Color, Stylize},
    terminal, ExecutableCommand,
};

use crate::i18n::{sys_msg, Language, Msg};

/// Result of a choice prompt — either a selection or a request to open the menu.
pub enum ChoiceResult {
    /// Player selected a choice (0-based index)
    Selected(usize),
    /// Player pressed Esc — open the pause menu
    OpenMenu,
}

/// Result of displaying messages — either completed normally or Esc was pressed.
pub enum MessageResult {
    /// All messages displayed normally
    Done,
    /// Player pressed Esc during animation — caller should open the pause menu
    OpenMenu,
}

/// Actions available in the pause menu.
pub enum PauseAction {
    Resume,
    ChangeLanguage,
    SaveQuit,
}

/// Default typewriter delay per character in milliseconds
const DEFAULT_CHAR_DELAY_MS: u64 = 60;

/// Duration to show the "typing..." indicator before each message
const TYPING_INDICATOR_MS: u64 = 1500;

/// Terminal width to use for alignment calculations
fn term_width() -> u16 {
    terminal::size().map(|(w, _)| w).unwrap_or(80)
}

/// Clear the terminal screen and move cursor to top-left
pub fn clear_screen() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    Ok(())
}

/// What happened when we checked for a keypress during animation
enum AnimKeypress {
    /// No key was pressed
    None,
    /// A non-Esc key was pressed (skip animation)
    Skip,
    /// Esc was pressed (open pause menu)
    Esc,
}

/// Check if a key has been pressed (non-blocking), distinguishing Esc from other keys.
fn check_keypress() -> AnimKeypress {
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            return match key.code {
                KeyCode::Esc => AnimKeypress::Esc,
                _ => AnimKeypress::Skip,
            };
        }
    }
    AnimKeypress::None
}

/// Show the animated "Elara is typing..." indicator
/// The dots cycle: . .. ... and back
/// Can be skipped by pressing any key. Esc returns OpenMenu.
pub fn show_typing_indicator(lang: Language) -> io::Result<MessageResult> {
    let mut stdout = io::stdout();
    let base_text = sys_msg(Msg::ElaraTyping, lang);

    // Enter raw mode so we can detect keypresses without blocking
    terminal::enable_raw_mode()?;

    let total_ms = TYPING_INDICATOR_MS;
    let frame_ms: u64 = 400;
    let frames = total_ms / frame_ms;

    for i in 0..frames {
        match check_keypress() {
            AnimKeypress::Esc => {
                write!(stdout, "\r{}\r", " ".repeat(base_text.len() + 10))?;
                stdout.flush()?;
                terminal::disable_raw_mode()?;
                return Ok(MessageResult::OpenMenu);
            }
            AnimKeypress::Skip => {
                write!(stdout, "\r{}\r", " ".repeat(base_text.len() + 10))?;
                stdout.flush()?;
                terminal::disable_raw_mode()?;
                return Ok(MessageResult::Done);
            }
            AnimKeypress::None => {}
        }

        let dots = ".".repeat((i as usize % 3) + 1);
        let padding = " ".repeat(3 - dots.len());
        write!(
            stdout,
            "\r  {}{}{}",
            base_text.with(Color::DarkGrey).attribute(Attribute::Italic),
            dots.with(Color::DarkGrey),
            padding
        )?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(frame_ms));
    }

    // Clear the typing indicator line
    write!(stdout, "\r{}\r", " ".repeat(base_text.len() + 10))?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;
    Ok(MessageResult::Done)
}

/// Print Elara's message with typewriter effect: characters appear one by one.
/// Can be skipped by pressing any key. Esc returns OpenMenu.
pub fn print_elara_message_animated(text: &str) -> io::Result<MessageResult> {
    let mut stdout = io::stdout();
    let prefix = "  Elara: ";

    // Enter raw mode for keypress detection
    terminal::enable_raw_mode()?;

    write!(
        stdout,
        "{}",
        prefix.with(Color::Cyan).attribute(Attribute::Bold)
    )?;
    stdout.flush()?;

    let mut skipped = false;
    let mut esc_pressed = false;

    for ch in text.chars() {
        if !skipped {
            match check_keypress() {
                AnimKeypress::Esc => {
                    skipped = true;
                    esc_pressed = true;
                }
                AnimKeypress::Skip => {
                    skipped = true;
                }
                AnimKeypress::None => {}
            }
        }

        if ch == '\n' {
            writeln!(stdout)?;
            write!(stdout, "         ")?; // indent continuation
        } else {
            write!(stdout, "{}", ch.to_string().with(Color::Cyan))?;
        }
        stdout.flush()?;

        if !skipped {
            thread::sleep(Duration::from_millis(DEFAULT_CHAR_DELAY_MS));
        }
    }
    writeln!(stdout)?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;

    if esc_pressed {
        Ok(MessageResult::OpenMenu)
    } else {
        Ok(MessageResult::Done)
    }
}

/// Print Elara's message without animation (for backlog replay)
pub fn print_elara_message(text: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let prefix = "  Elara: ".with(Color::Cyan).attribute(Attribute::Bold);
    write!(stdout, "{}", prefix)?;

    let lines: Vec<&str> = text.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            write!(stdout, "         ")?;
        }
        writeln!(stdout, "{}", line.with(Color::Cyan))?;
    }
    stdout.flush()?;
    Ok(())
}

/// Show typing indicator then print message with typewriter effect.
/// Returns `MessageResult::OpenMenu` if Esc was pressed at any point.
pub fn elara_says(text: &str, lang: Language) -> io::Result<MessageResult> {
    if matches!(show_typing_indicator(lang)?, MessageResult::OpenMenu) {
        // Esc during typing indicator — still print the full message instantly,
        // then signal the menu
        print_elara_message(text)?;
        return Ok(MessageResult::OpenMenu);
    }
    let result = print_elara_message_animated(text)?;
    // Small pause after message for readability
    thread::sleep(Duration::from_millis(300));
    Ok(result)
}

/// Print a player choice (after selection): right-aligned, green
pub fn print_player_choice(text: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let width = term_width() as usize;
    let display_text = format!("  {} >", text);
    let padding = if display_text.len() < width {
        width - display_text.len()
    } else {
        0
    };
    writeln!(
        stdout,
        "{}{}",
        " ".repeat(padding),
        display_text.with(Color::Green).attribute(Attribute::Bold)
    )?;
    stdout.flush()?;
    Ok(())
}

/// Print a system message: centered, dim gray
pub fn print_system_message(text: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let width = term_width() as usize;
    for line in text.lines() {
        let padding = if line.len() < width {
            (width - line.len()) / 2
        } else {
            0
        };
        writeln!(
            stdout,
            "{}{}",
            " ".repeat(padding),
            line.with(Color::DarkGrey)
        )?;
    }
    stdout.flush()?;
    Ok(())
}

/// Print a system message with typewriter effect (for atmospheric intro)
pub fn print_system_message_animated(text: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let width = term_width() as usize;

    terminal::enable_raw_mode()?;
    let mut skipped = false;

    for line in text.lines() {
        let padding = if line.len() < width {
            (width - line.len()) / 2
        } else {
            0
        };
        write!(stdout, "{}", " ".repeat(padding))?;

        for ch in line.chars() {
            if !skipped && !matches!(check_keypress(), AnimKeypress::None) {
                skipped = true;
            }
            write!(stdout, "{}", ch.to_string().with(Color::DarkGrey))?;
            stdout.flush()?;
            if !skipped {
                thread::sleep(Duration::from_millis(40));
            }
        }
        writeln!(stdout)?;
    }
    stdout.flush()?;

    terminal::disable_raw_mode()?;
    Ok(())
}

/// Print a horizontal separator line with optional timestamp
pub fn print_separator(timestamp: Option<&str>) -> io::Result<()> {
    let mut stdout = io::stdout();
    let width = term_width() as usize;

    match timestamp {
        Some(ts) => {
            let label = format!(" {} ", ts);
            let side_len = if label.len() < width {
                (width - label.len()) / 2
            } else {
                0
            };
            let line = format!(
                "{}{}{}",
                "\u{2500}".repeat(side_len),
                label,
                "\u{2500}".repeat(side_len)
            );
            writeln!(stdout, "{}", line.with(Color::DarkGrey))?;
        }
        None => {
            writeln!(
                stdout,
                "{}",
                "\u{2500}".repeat(width.min(80)).with(Color::DarkGrey)
            )?;
        }
    }
    stdout.flush()?;
    Ok(())
}

/// Display choices as an interactive menu navigated with arrow keys.
/// Up/Down (or k/j) to move, Enter to confirm, Esc to open the pause menu.
/// Returns `ChoiceResult::Selected(index)` or `ChoiceResult::OpenMenu`.
pub fn prompt_choice(choices: &[String]) -> io::Result<ChoiceResult> {
    let mut stdout = io::stdout();
    let count = choices.len();
    let mut selected: usize = 0;

    writeln!(stdout)?;

    // Draw the initial menu
    draw_choices(&mut stdout, choices, selected)?;

    // Enter raw mode for key-by-key input
    terminal::enable_raw_mode()?;
    // Hide cursor for cleaner look
    stdout.execute(cursor::Hide)?;

    let result = loop {
        // Poll for events (with a timeout so we can check for Ctrl+C flag)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = if selected == 0 {
                            count - 1
                        } else {
                            selected - 1
                        };
                        redraw_choices(&mut stdout, choices, selected)?;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        selected = (selected + 1) % count;
                        redraw_choices(&mut stdout, choices, selected)?;
                    }
                    KeyCode::Enter => {
                        break Ok(ChoiceResult::Selected(selected));
                    }
                    KeyCode::Esc => {
                        break Ok(ChoiceResult::OpenMenu);
                    }
                    _ => {}
                }
            }
        }

        // Check for Ctrl+C via the atomic flag
        if crate::is_interrupted() {
            break Err(io::Error::new(io::ErrorKind::Interrupted, "interrupted"));
        }
    };

    // Restore terminal state
    stdout.execute(cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}

/// Like `prompt_choice`, but ignores Esc (keeps looping until a selection is made).
/// Used for system menus where the pause menu doesn't apply.
pub fn prompt_choice_simple(choices: &[String]) -> io::Result<usize> {
    loop {
        match prompt_choice(choices)? {
            ChoiceResult::Selected(idx) => return Ok(idx),
            ChoiceResult::OpenMenu => {
                // Esc has no effect in system menus — just redisplay.
                // The choices are still on screen; we need to erase and redraw.
                let mut stdout = io::stdout();
                // Move up past the choice lines to redraw
                let count = choices.len() as u16;
                for _ in 0..count {
                    stdout.execute(cursor::MoveUp(1))?;
                    write!(stdout, "\r")?;
                    stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                }
                // Also erase the blank line before choices
                stdout.execute(cursor::MoveUp(1))?;
                write!(stdout, "\r")?;
                stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                stdout.flush()?;
                continue;
            }
        }
    }
}

/// Draw the choice menu (initial render). Each line: `  > choice` or `    choice`.
fn draw_choices(stdout: &mut io::Stdout, choices: &[String], selected: usize) -> io::Result<()> {
    for (i, choice) in choices.iter().enumerate() {
        if i == selected {
            writeln!(
                stdout,
                "  {} {}",
                ">".with(Color::Yellow).attribute(Attribute::Bold),
                choice
                    .as_str()
                    .with(Color::Yellow)
                    .attribute(Attribute::Bold),
            )?;
        } else {
            writeln!(
                stdout,
                "    {}",
                choice
                    .as_str()
                    .with(Color::Yellow)
                    .attribute(Attribute::Dim),
            )?;
        }
    }
    stdout.flush()?;
    Ok(())
}

/// Redraw the choice menu in-place by moving the cursor up and overwriting.
fn redraw_choices(stdout: &mut io::Stdout, choices: &[String], selected: usize) -> io::Result<()> {
    let count = choices.len() as u16;
    // Move cursor up to the first choice line
    stdout.execute(cursor::MoveUp(count))?;

    for (i, choice) in choices.iter().enumerate() {
        // Clear the line and rewrite
        write!(stdout, "\r")?;
        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;

        if i == selected {
            writeln!(
                stdout,
                "  {} {}",
                ">".with(Color::Yellow).attribute(Attribute::Bold),
                choice
                    .as_str()
                    .with(Color::Yellow)
                    .attribute(Attribute::Bold),
            )?;
        } else {
            writeln!(
                stdout,
                "    {}",
                choice
                    .as_str()
                    .with(Color::Yellow)
                    .attribute(Attribute::Dim),
            )?;
        }
    }
    stdout.flush()?;
    Ok(())
}

/// Show the pause menu overlay. Returns the chosen action.
/// The menu is displayed inline, then erased when the player picks an option.
pub fn show_pause_menu(lang: Language) -> io::Result<PauseAction> {
    let mut stdout = io::stdout();

    // Menu items
    let items = vec![
        sys_msg(Msg::MenuResume, lang).to_string(),
        sys_msg(Msg::MenuChangeLanguage, lang).to_string(),
        sys_msg(Msg::MenuSaveQuit, lang).to_string(),
    ];

    print_blank()?;
    print_separator(None)?;
    print_system_message(sys_msg(Msg::PauseMenuTitle, lang))?;
    print_blank()?;

    // We need to track how many lines the menu occupies so we can erase it later.
    // Title area: blank + separator + title + blank = 4 lines
    // Choices: items.len() lines
    // Trailing blank: 1 line
    // Separator: 1 line
    let menu_lines = 4 + items.len() as u16 + 2;

    draw_choices(&mut stdout, &items, 0)?;
    print_blank()?;
    print_separator(None)?;

    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    let mut selected: usize = 0;
    let count = items.len();

    let result = loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = if selected == 0 {
                            count - 1
                        } else {
                            selected - 1
                        };
                        // Move up past trailing blank + separator (2 lines) + choices
                        stdout.execute(cursor::MoveUp(count as u16 + 2))?;
                        for (i, item) in items.iter().enumerate() {
                            write!(stdout, "\r")?;
                            stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                            if i == selected {
                                writeln!(
                                    stdout,
                                    "  {} {}",
                                    ">".with(Color::Yellow).attribute(Attribute::Bold),
                                    item.as_str().with(Color::Yellow).attribute(Attribute::Bold),
                                )?;
                            } else {
                                writeln!(
                                    stdout,
                                    "    {}",
                                    item.as_str().with(Color::Yellow).attribute(Attribute::Dim),
                                )?;
                            }
                        }
                        // Rewrite trailing blank + separator
                        write!(stdout, "\r")?;
                        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                        writeln!(stdout)?;
                        write!(stdout, "\r")?;
                        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                        let width = term_width() as usize;
                        writeln!(
                            stdout,
                            "{}",
                            "\u{2500}".repeat(width.min(80)).with(Color::DarkGrey)
                        )?;
                        stdout.flush()?;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        selected = (selected + 1) % count;
                        stdout.execute(cursor::MoveUp(count as u16 + 2))?;
                        for (i, item) in items.iter().enumerate() {
                            write!(stdout, "\r")?;
                            stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                            if i == selected {
                                writeln!(
                                    stdout,
                                    "  {} {}",
                                    ">".with(Color::Yellow).attribute(Attribute::Bold),
                                    item.as_str().with(Color::Yellow).attribute(Attribute::Bold),
                                )?;
                            } else {
                                writeln!(
                                    stdout,
                                    "    {}",
                                    item.as_str().with(Color::Yellow).attribute(Attribute::Dim),
                                )?;
                            }
                        }
                        write!(stdout, "\r")?;
                        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                        writeln!(stdout)?;
                        write!(stdout, "\r")?;
                        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
                        let width = term_width() as usize;
                        writeln!(
                            stdout,
                            "{}",
                            "\u{2500}".repeat(width.min(80)).with(Color::DarkGrey)
                        )?;
                        stdout.flush()?;
                    }
                    KeyCode::Enter => {
                        break match selected {
                            0 => PauseAction::Resume,
                            1 => PauseAction::ChangeLanguage,
                            2 => PauseAction::SaveQuit,
                            _ => PauseAction::Resume,
                        };
                    }
                    KeyCode::Esc => {
                        // Esc again = resume
                        break PauseAction::Resume;
                    }
                    _ => {}
                }
            }
        }

        if crate::is_interrupted() {
            break PauseAction::SaveQuit;
        }
    };

    stdout.execute(cursor::Show)?;
    terminal::disable_raw_mode()?;

    // Erase the menu by moving up and clearing each line
    for _ in 0..menu_lines {
        stdout.execute(cursor::MoveUp(1))?;
        write!(stdout, "\r")?;
        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
    }
    stdout.flush()?;

    Ok(result)
}

/// Print a blank line
pub fn print_blank() -> io::Result<()> {
    writeln!(io::stdout())?;
    Ok(())
}

/// Print the game title/banner
pub fn print_banner() -> io::Result<()> {
    let mut stdout = io::stdout();
    let width = term_width() as usize;

    let title = "E S H A R A";
    let padding = if title.len() < width {
        (width - title.len()) / 2
    } else {
        0
    };

    writeln!(stdout)?;
    writeln!(
        stdout,
        "{}{}",
        " ".repeat(padding),
        title.with(Color::White).attribute(Attribute::Bold)
    )?;
    writeln!(stdout)?;
    print_separator(None)?;
    writeln!(stdout)?;

    Ok(())
}

/// Replay the message backlog (non-animated) when resuming a saved game.
/// Inserts session separators at LogEntry items with Sender::System whose text
/// starts with "SESSION:".
pub fn replay_backlog(log: &[crate::game::LogEntry], lang: Language) -> io::Result<()> {
    if log.is_empty() {
        return Ok(());
    }

    print_system_message(sys_msg(Msg::BacklogHeader, lang))?;
    print_blank()?;

    for entry in log {
        match entry.sender {
            crate::game::Sender::Elara => {
                print_elara_message(&entry.text)?;
            }
            crate::game::Sender::Player => {
                print_player_choice(&entry.text)?;
                print_blank()?;
            }
            crate::game::Sender::System => {
                if entry.text.starts_with("SESSION:") {
                    // Session separator — extract the timestamp label
                    let label = entry.text.trim_start_matches("SESSION:");
                    print_blank()?;
                    print_separator(Some(label))?;
                    print_blank()?;
                } else {
                    print_system_message(&entry.text)?;
                }
            }
        }
    }

    print_blank()?;
    print_separator(None)?;
    print_blank()?;

    Ok(())
}
