use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{Attribute, Color, Stylize},
    terminal, ExecutableCommand,
};

use crate::i18n::{sys_msg, Language, Msg};

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

/// Check if a key has been pressed (non-blocking)
fn key_pressed() -> bool {
    if event::poll(Duration::from_millis(0)).unwrap_or(false) {
        if let Ok(Event::Key(_)) = event::read() {
            return true;
        }
    }
    false
}

/// Show the animated "Elara is typing..." indicator
/// The dots cycle: . .. ... and back
/// Can be skipped by pressing any key
pub fn show_typing_indicator(lang: Language) -> io::Result<()> {
    let mut stdout = io::stdout();
    let base_text = sys_msg(Msg::ElaraTyping, lang);

    // Enter raw mode so we can detect keypresses without blocking
    terminal::enable_raw_mode()?;

    let total_ms = TYPING_INDICATOR_MS;
    let frame_ms: u64 = 400;
    let frames = total_ms / frame_ms;

    for i in 0..frames {
        // Check for keypress to skip
        if key_pressed() {
            // Clear the typing indicator line
            write!(stdout, "\r{}\r", " ".repeat(base_text.len() + 10))?;
            stdout.flush()?;
            terminal::disable_raw_mode()?;
            return Ok(());
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
    Ok(())
}

/// Print Elara's message with typewriter effect: characters appear one by one
/// Can be skipped by pressing any key (remaining text appears instantly)
pub fn print_elara_message_animated(text: &str) -> io::Result<()> {
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
    let mut col = prefix.len();

    for ch in text.chars() {
        if !skipped && key_pressed() {
            skipped = true;
        }

        if ch == '\n' {
            writeln!(stdout)?;
            write!(stdout, "         ")?; // indent continuation
            col = 9;
        } else {
            write!(stdout, "{}", ch.to_string().with(Color::Cyan))?;
            col += 1;
        }
        stdout.flush()?;

        if !skipped {
            thread::sleep(Duration::from_millis(DEFAULT_CHAR_DELAY_MS));
        }
    }
    writeln!(stdout)?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;

    let _ = col; // suppress unused warning
    Ok(())
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

/// Show typing indicator then print message with typewriter effect
pub fn elara_says(text: &str, lang: Language) -> io::Result<()> {
    show_typing_indicator(lang)?;
    print_elara_message_animated(text)?;
    // Small pause after message for readability
    thread::sleep(Duration::from_millis(300));
    Ok(())
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
            if !skipped && key_pressed() {
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
/// Up/Down (or k/j) to move, Enter to confirm.
/// Returns the 0-based index of the chosen option.
pub fn prompt_choice(choices: &[String]) -> io::Result<usize> {
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
                        break Ok(selected);
                    }
                    KeyCode::Esc => {
                        // Treat Esc like an interrupt
                        break Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled"));
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
