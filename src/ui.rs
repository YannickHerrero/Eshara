use std::io::{self, Write};

use crossterm::{
    cursor,
    style::{self, Attribute, Color, Stylize},
    terminal, ExecutableCommand,
};

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

/// Print Elara's message: left-aligned, cyan, with "Elara:" prefix
pub fn print_elara_message(text: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let prefix = "  Elara: ".with(Color::Cyan).attribute(Attribute::Bold);
    write!(stdout, "{}", prefix)?;

    // Handle multi-line messages with proper indentation
    let lines: Vec<&str> = text.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            write!(stdout, "         ")?; // indent continuation lines
        }
        writeln!(stdout, "{}", line.with(Color::Cyan))?;
    }
    stdout.flush()?;
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

/// Display numbered choices and read the player's selection (1-indexed)
/// Returns the 0-based index of the chosen option
pub fn prompt_choice(choices: &[String]) -> io::Result<usize> {
    let mut stdout = io::stdout();

    writeln!(stdout)?;
    for (i, choice) in choices.iter().enumerate() {
        writeln!(
            stdout,
            "  {}",
            format!("{}. {}", i + 1, choice)
                .with(Color::Yellow)
                .attribute(Attribute::Dim)
        )?;
    }
    writeln!(stdout)?;
    stdout.flush()?;

    loop {
        write!(stdout, "  {} ", ">".with(Color::Yellow))?;
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if let Ok(n) = trimmed.parse::<usize>() {
            if n >= 1 && n <= choices.len() {
                return Ok(n - 1);
            }
        }

        writeln!(
            stdout,
            "  {}",
            "Invalid choice. Please enter a number."
                .with(Color::Red)
                .attribute(Attribute::Dim)
        )?;
    }
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

// Silence unused import warnings for style — it's used via the Stylize trait
const _: () = {
    fn _use_style() {
        let _ = std::mem::size_of::<style::Color>();
    }
};
