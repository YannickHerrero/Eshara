use std::env;
use std::io;
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Duration as ChronoDuration, Local, Utc};

use crate::game::GameState;
use crate::i18n::{sys_msg, Language, Msg};
use crate::ui;

/// Check if debug mode is enabled (ESHARA_DEBUG=1)
/// In debug mode, all delays are reduced to 5 seconds
pub fn is_debug_mode() -> bool {
    env::var("ESHARA_DEBUG")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

/// Get the effective delay in seconds (respects debug mode)
pub fn effective_delay(seconds: u64) -> u64 {
    if is_debug_mode() {
        5 // All delays become 5 seconds in debug mode
    } else {
        seconds
    }
}

/// Schedule Elara to be "busy" for the given number of seconds
/// Sets `waiting_until` on the game state
pub fn schedule_wait(state: &mut GameState, seconds: u64) {
    let delay = effective_delay(seconds);
    let until = Utc::now() + ChronoDuration::seconds(delay as i64);
    state.waiting_until = Some(until);
}

/// Check if Elara is currently busy (waiting_until is in the future)
pub fn is_waiting(state: &GameState) -> bool {
    if let Some(until) = state.waiting_until {
        Utc::now() < until
    } else {
        false
    }
}

/// Get the remaining wait time as a human-readable string
pub fn remaining_time_str(until: DateTime<Utc>, lang: Language) -> String {
    let now = Utc::now();
    if now >= until {
        return match lang {
            Language::En => "any moment now".to_string(),
            Language::Fr => "d'un moment \u{00e0} l'autre".to_string(),
        };
    }

    let diff = until - now;
    let hours = diff.num_hours();
    let minutes = diff.num_minutes() % 60;

    if hours > 0 {
        match lang {
            Language::En => format!("{}h {}min", hours, minutes),
            Language::Fr => format!("{}h {}min", hours, minutes),
        }
    } else if minutes > 0 {
        match lang {
            Language::En => format!("{} minute{}", minutes, if minutes > 1 { "s" } else { "" }),
            Language::Fr => format!("{} minute{}", minutes, if minutes > 1 { "s" } else { "" }),
        }
    } else {
        match lang {
            Language::En => "less than a minute".to_string(),
            Language::Fr => "moins d'une minute".to_string(),
        }
    }
}

/// Format a DateTime as a local time string for display (e.g., "14:30")
pub fn format_local_time(dt: DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.into();
    local.format("%H:%M").to_string()
}

/// Handle the waiting state when the player launches the game while Elara is busy
/// Returns true if the player chose to wait (and the wait completed),
/// false if they chose to quit
pub fn handle_waiting(state: &mut GameState) -> io::Result<bool> {
    let lang = state.language;

    if let Some(until) = state.waiting_until {
        if Utc::now() >= until {
            // Wait is over — clear it and continue
            state.waiting_until = None;
            // Bell notification
            print!("\x07");
            return Ok(true);
        }

        // Elara is still busy
        ui::print_blank()?;
        ui::print_system_message(sys_msg(Msg::ElaraUnavailable, lang))?;
        ui::print_blank()?;

        let back_time = format_local_time(until);
        let remaining = remaining_time_str(until, lang);
        ui::print_system_message(&format!(
            "{} {} (~{})",
            sys_msg(Msg::ElaraBackAround, lang),
            back_time,
            remaining
        ))?;
        ui::print_blank()?;

        ui::print_system_message(sys_msg(Msg::WaitOrQuit, lang))?;
        let choices = vec![
            sys_msg(Msg::WaitOption, lang).to_string(),
            sys_msg(Msg::QuitOption, lang).to_string(),
        ];
        let choice = ui::prompt_choice_simple(&choices)?;

        if choice == 0 {
            // Wait: poll until the time is reached
            wait_until(until, lang)?;
            state.waiting_until = None;
            // Bell notification
            print!("\x07");
            return Ok(true);
        } else {
            // Quit
            return Ok(false);
        }
    }

    // Not waiting
    Ok(true)
}

/// Actively wait until the given time, showing a countdown
fn wait_until(until: DateTime<Utc>, lang: Language) -> io::Result<()> {
    ui::print_blank()?;

    loop {
        let now = Utc::now();
        if now >= until {
            break;
        }

        let remaining = remaining_time_str(until, lang);
        let msg = match lang {
            Language::En => format!("Waiting... ({})", remaining),
            Language::Fr => format!("En attente... ({})", remaining),
        };
        ui::print_system_message(&msg)?;

        // Sleep for a short interval, then re-check
        let sleep_secs = if is_debug_mode() { 1 } else { 30 };
        thread::sleep(Duration::from_secs(sleep_secs));
    }

    ui::print_blank()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_delay_normal() {
        // Without ESHARA_DEBUG set, should return the original value
        // (This test assumes ESHARA_DEBUG is not set in the test env)
        if !is_debug_mode() {
            assert_eq!(effective_delay(300), 300);
        }
    }

    #[test]
    fn test_remaining_time_str_past() {
        let past = Utc::now() - ChronoDuration::hours(1);
        assert_eq!(remaining_time_str(past, Language::En), "any moment now");
        assert_eq!(
            remaining_time_str(past, Language::Fr),
            "d'un moment \u{00e0} l'autre"
        );
    }

    #[test]
    fn test_remaining_time_str_future() {
        let future = Utc::now() + ChronoDuration::hours(2) + ChronoDuration::minutes(15);
        let result = remaining_time_str(future, Language::En);
        assert!(result.contains("h"));
    }

    #[test]
    fn test_schedule_wait() {
        let mut state = GameState::new(Language::En);
        assert!(state.waiting_until.is_none());
        schedule_wait(&mut state, 60);
        assert!(state.waiting_until.is_some());
    }

    #[test]
    fn test_is_waiting() {
        let mut state = GameState::new(Language::En);
        assert!(!is_waiting(&state));
        schedule_wait(&mut state, 3600); // 1 hour from now
        assert!(is_waiting(&state));
    }
}
