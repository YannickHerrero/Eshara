//! Ratatui-based terminal UI for Eshara.
//!
//! This module implements the full game interface using ratatui's immediate-mode
//! rendering model. The `App` struct holds all UI state; the `run()` function
//! drives the event loop.

use std::time::{Duration, Instant};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::game::{save_game, GameState, LogEntry, Sender};
use crate::i18n::{sys_msg, Language, Msg};
use crate::story::{self, EndingType, StoryData};

// ── Constants ────────────────────────────────────────────────

/// Milliseconds between each character reveal in typewriter mode.
const TYPEWRITER_TICK_MS: u64 = 45;

/// Milliseconds to show the "Elara is typing..." indicator.
const TYPING_INDICATOR_MS: u64 = 1500;

/// Milliseconds between animation frames (dot cycling).
const ANIM_FRAME_MS: u64 = 400;

// ── Chat entries ─────────────────────────────────────────────

/// A single entry in the visible chat log.
#[derive(Clone, Debug)]
pub enum ChatEntry {
    Elara(String),
    Player(String),
    System(String),
    Separator(String),
}

// ── Screen / overlay state ───────────────────────────────────

/// Which screen the app is currently showing.
#[derive(Clone, Debug, PartialEq)]
pub enum Screen {
    /// Language selection (first launch).
    LanguageSelect,
    /// "Continue or new game?" prompt.
    ContinueOrNew,
    /// Atmospheric intro sequence.
    Intro,
    /// Main gameplay (chat + choices).
    Game,
    /// Elara is unavailable (real-time wait).
    Waiting,
    /// Ending summary screen.
    Ending,
}

/// Overlay that renders on top of the current screen.
#[derive(Clone, Debug, PartialEq)]
pub enum Overlay {
    None,
    PauseMenu,
}

// ── Animation state ──────────────────────────────────────────

/// Tracks the typewriter animation for the current message.
#[derive(Clone, Debug)]
pub struct TypewriterState {
    /// Full text to reveal.
    pub full_text: String,
    /// How many characters have been revealed so far.
    pub revealed: usize,
    /// When the last character was revealed.
    pub last_tick: Instant,
    /// If true, show "Elara is typing..." before the message.
    pub show_typing_indicator: bool,
    /// When the typing indicator started.
    pub indicator_start: Instant,
}

impl TypewriterState {
    pub fn new(text: String) -> Self {
        Self {
            full_text: text,
            revealed: 0,
            last_tick: Instant::now(),
            show_typing_indicator: true,
            indicator_start: Instant::now(),
        }
    }

    /// Is the typing indicator phase still active?
    pub fn is_indicating(&self) -> bool {
        self.show_typing_indicator
            && self.indicator_start.elapsed() < Duration::from_millis(TYPING_INDICATOR_MS)
    }

    /// Is the full text revealed?
    pub fn is_done(&self) -> bool {
        !self.show_typing_indicator && self.revealed >= self.full_text.len()
    }

    /// Reset timing after a pause so the animation doesn't fast-forward.
    pub fn resume(&mut self) {
        let now = Instant::now();
        self.last_tick = now;
        if self.show_typing_indicator {
            // Preserve how much indicator time was left
            let elapsed = self.indicator_start.elapsed();
            let indicator_dur = Duration::from_millis(TYPING_INDICATOR_MS);
            if elapsed < indicator_dur {
                // Reset indicator_start so the remaining time is preserved
                self.indicator_start = now - elapsed;
            }
        }
    }

    /// Skip to completion: reveal all text immediately.
    pub fn skip(&mut self) {
        self.show_typing_indicator = false;
        self.revealed = self.full_text.len();
    }

    /// Advance the animation by one tick if enough time has passed.
    pub fn tick(&mut self) {
        if self.show_typing_indicator {
            if self.indicator_start.elapsed() >= Duration::from_millis(TYPING_INDICATOR_MS) {
                self.show_typing_indicator = false;
                self.last_tick = Instant::now();
            }
            return;
        }
        if self.revealed < self.full_text.len()
            && self.last_tick.elapsed() >= Duration::from_millis(TYPEWRITER_TICK_MS)
        {
            // Reveal one character (handle multi-byte)
            let remaining = &self.full_text[self.revealed..];
            if let Some(ch) = remaining.chars().next() {
                self.revealed += ch.len_utf8();
            }
            self.last_tick = Instant::now();
        }
    }

    /// Get the currently visible text slice.
    pub fn visible_text(&self) -> &str {
        &self.full_text[..self.revealed]
    }
}

// ── App state ────────────────────────────────────────────────

/// The main application state that drives the ratatui UI.
pub struct App {
    /// Current screen.
    pub screen: Screen,
    /// Current overlay (None or PauseMenu).
    pub overlay: Overlay,
    /// Visible chat entries.
    pub chat: Vec<ChatEntry>,
    /// Scroll offset for chat (0 = bottom). Reserved for manual scroll support.
    #[allow(dead_code)]
    pub chat_scroll: u16,
    /// Current typewriter animation (if any).
    pub typewriter: Option<TypewriterState>,
    /// Queue of messages still to be displayed for the current node.
    pub message_queue: Vec<String>,
    /// Choices currently being presented to the player.
    pub choices: Vec<String>,
    /// Selection index for the choice menu.
    pub choice_index: usize,
    /// Selection index for the pause menu.
    pub menu_index: usize,
    /// Selection index for generic prompts (language, continue, etc.).
    pub prompt_index: usize,
    /// Prompt options for the current screen.
    pub prompt_options: Vec<String>,
    /// Whether the app should exit.
    pub should_quit: bool,
    /// The game state (borrowed mutably during run).
    /// We'll hold this directly since we own the game loop.
    pub game_state: GameState,
    /// The full story data (nodes + endings), loaded from JSON.
    pub story_data: StoryData,
    /// Whether we need to process the next story node.
    pub advance_story: bool,
    /// Intro animation state.
    pub intro_typewriter: Option<TypewriterState>,
    /// Post-message pause timer (small delay after a message finishes).
    pub post_message_pause: Option<Instant>,
    /// Ending type reached (for the ending screen).
    pub ending_reached: Option<EndingType>,
    /// Wait screen info.
    pub wait_message: Option<String>,
}

impl App {
    /// Create a new App for a fresh or resumed game.
    pub fn new(game_state: GameState, story_data: StoryData) -> Self {
        Self {
            screen: Screen::Game,
            overlay: Overlay::None,
            chat: Vec::new(),
            chat_scroll: 0,
            typewriter: None,
            message_queue: Vec::new(),
            choices: Vec::new(),
            choice_index: 0,
            menu_index: 0,
            prompt_index: 0,
            prompt_options: Vec::new(),
            should_quit: false,
            game_state,
            story_data,
            advance_story: true,
            intro_typewriter: None,
            post_message_pause: None,
            ending_reached: None,
            wait_message: None,
        }
    }

    pub fn lang(&self) -> Language {
        self.game_state.language
    }

    /// Load the backlog from the game state's message log into the chat.
    pub fn load_backlog(&mut self) {
        for entry in &self.game_state.message_log {
            match entry.sender {
                Sender::Elara => self.chat.push(ChatEntry::Elara(entry.text.clone())),
                Sender::Player => self.chat.push(ChatEntry::Player(entry.text.clone())),
                Sender::System => {
                    if entry.text.starts_with("SESSION:") {
                        let label = entry.text.trim_start_matches("SESSION:").to_string();
                        self.chat.push(ChatEntry::Separator(label));
                    } else {
                        self.chat.push(ChatEntry::System(entry.text.clone()));
                    }
                }
            }
        }
    }

    /// Process the current story node: queue its messages and prepare choices.
    pub fn process_current_node(&mut self) {
        self.advance_story = false;

        let node = match self.story_data.nodes.get(&self.game_state.current_node) {
            Some(n) => n.clone(),
            None => {
                self.chat.push(ChatEntry::System(format!(
                    "Error: story node '{}' not found.",
                    self.game_state.current_node
                )));
                self.should_quit = true;
                return;
            }
        };

        let lang = self.lang();

        // Queue all messages for typewriter display
        self.message_queue.clear();
        for msg in &node.messages {
            self.message_queue.push(msg.get(lang).to_string());
        }

        // Start the first message
        self.start_next_message();
    }

    /// Pop the next message from the queue and start its typewriter animation.
    fn start_next_message(&mut self) {
        if self.message_queue.is_empty() {
            // All messages displayed — now handle the node's outcome
            self.handle_node_outcome();
            return;
        }

        let text = self.message_queue.remove(0);
        self.typewriter = Some(TypewriterState::new(text));
    }

    /// Called when all messages for the current node have been displayed.
    fn handle_node_outcome(&mut self) {
        let node = match self.story_data.nodes.get(&self.game_state.current_node) {
            Some(n) => n.clone(),
            None => return,
        };

        let lang = self.lang();

        // Check for ending
        if node.ending.is_some() {
            self.game_state.ending = node.ending.clone();
            let _ = save_game(&self.game_state);
            self.ending_reached = node.ending;
            self.screen = Screen::Ending;
            self.prompt_options = vec![
                sys_msg(Msg::YesOption, lang).to_string(),
                sys_msg(Msg::NoOption, lang).to_string(),
            ];
            self.prompt_index = 0;
            return;
        }

        // Handle real-time delay
        if let Some(delay_secs) = node.delay {
            let next = if !node.choices.is_empty() {
                node.choices[0].next_node.clone()
            } else if let Some(ref next) = node.next_node {
                next.clone()
            } else {
                self.should_quit = true;
                return;
            };

            self.game_state.current_node = next;
            crate::time::schedule_wait(&mut self.game_state, delay_secs);
            let _ = save_game(&self.game_state);

            // Show wait screen
            self.screen = Screen::Waiting;
            self.prompt_options = vec![
                sys_msg(Msg::WaitOption, lang).to_string(),
                sys_msg(Msg::QuitOption, lang).to_string(),
            ];
            self.prompt_index = 0;

            let remaining =
                crate::time::remaining_time_str(self.game_state.waiting_until.unwrap(), lang);
            self.wait_message = Some(format!(
                "{}\n\n{} (~{})",
                sys_msg(Msg::ElaraUnavailable, lang),
                sys_msg(Msg::ElaraBackAround, lang),
                remaining,
            ));
            return;
        }

        // Handle choices
        if !node.choices.is_empty() {
            // Trust-based refusal check
            if node.should_refuse(&self.game_state) {
                let refusal = node.trust_refusal.as_ref().unwrap();
                let refusal_text = refusal.refusal_message.get(lang).to_string();

                self.chat.push(ChatEntry::Elara(refusal_text.clone()));
                self.game_state.message_log.push(LogEntry {
                    sender: Sender::Elara,
                    text: refusal_text,
                    timestamp: chrono::Utc::now(),
                });

                self.game_state.current_node = refusal.refusal_node.clone();
                let _ = save_game(&self.game_state);
                self.advance_story = true;
                return;
            }

            // Filter available choices
            let available: Vec<(usize, &story::Choice)> = node.available_choices(&self.game_state);

            if available.is_empty() {
                if let Some(ref next) = node.next_node {
                    self.game_state.current_node = next.clone();
                    let _ = save_game(&self.game_state);
                    self.advance_story = true;
                } else {
                    self.should_quit = true;
                }
                return;
            }

            let choice_labels: Vec<String> = available
                .iter()
                .map(|(_, c)| c.label.get(lang).to_string())
                .collect();

            // Auto-route: if all choices are "...", pick the first silently
            let is_auto_route = choice_labels.iter().all(|l| l == "...");
            if is_auto_route {
                let (_, chosen) = available[0];
                self.apply_choice(chosen);
                return;
            }

            self.choices = choice_labels;
            self.choice_index = 0;
        } else if let Some(ref next) = node.next_node {
            self.game_state.current_node = next.clone();
            let _ = save_game(&self.game_state);
            self.advance_story = true;
        } else {
            self.should_quit = true;
        }
    }

    /// Apply a chosen choice: set flags, modify stats, advance node.
    fn apply_choice(&mut self, choice: &story::Choice) {
        for flag in &choice.flags_set {
            self.game_state.set_flag(flag);
        }
        for flag in &choice.flags_remove {
            self.game_state.remove_flag(flag);
        }
        for (stat, delta) in &choice.stat_changes {
            self.game_state.stats.modify(stat, *delta);
        }
        self.game_state.current_node = choice.next_node.clone();
        let _ = save_game(&self.game_state);
        self.advance_story = true;
    }

    /// Called when the player selects a choice.
    pub fn select_choice(&mut self) {
        if self.choices.is_empty() {
            return;
        }

        let lang = self.lang();
        let label = self.choices[self.choice_index].clone();

        // Show player's choice in chat
        self.chat.push(ChatEntry::Player(label.clone()));
        self.game_state.message_log.push(LogEntry {
            sender: Sender::Player,
            text: label,
            timestamp: chrono::Utc::now(),
        });

        // Find the original choice by matching the current node's available choices
        let node = self
            .story_data
            .nodes
            .get(&self.game_state.current_node)
            .cloned();
        if let Some(node) = node {
            let available: Vec<(usize, &story::Choice)> = node.available_choices(&self.game_state);
            let non_auto: Vec<_> = available
                .iter()
                .filter(|(_, c)| c.label.get(lang) != "...")
                .collect();

            if self.choice_index < non_auto.len() {
                let (_, chosen) = non_auto[self.choice_index];
                let chosen = (*chosen).clone();
                self.choices.clear();
                self.apply_choice(&chosen);
            }
        }
    }

    /// Called when a typewriter animation finishes for a message.
    pub fn on_message_complete(&mut self) {
        if let Some(tw) = self.typewriter.take() {
            let text = tw.full_text;
            self.chat.push(ChatEntry::Elara(text.clone()));
            self.game_state.message_log.push(LogEntry {
                sender: Sender::Elara,
                text,
                timestamp: chrono::Utc::now(),
            });
            let _ = save_game(&self.game_state);
        }

        // Small pause before next message
        self.post_message_pause = Some(Instant::now());
    }

    /// Close the overlay and reset animation timers so nothing fast-forwards.
    pub fn resume_from_overlay(&mut self) {
        self.overlay = Overlay::None;
        if let Some(ref mut tw) = self.typewriter {
            tw.resume();
        }
        if let Some(ref mut tw) = self.intro_typewriter {
            tw.resume();
        }
        // Reset post-message pause timer so it doesn't expire instantly
        if self.post_message_pause.is_some() {
            self.post_message_pause = Some(Instant::now());
        }
    }
}

// ── Event handling ───────────────────────────────────────────

/// Handle a key event. Returns true if the event was consumed.
pub fn handle_key(app: &mut App, code: KeyCode) {
    // Overlay takes priority
    if app.overlay == Overlay::PauseMenu {
        handle_pause_menu_key(app, code);
        return;
    }

    match app.screen {
        Screen::Game => handle_game_key(app, code),
        Screen::LanguageSelect | Screen::ContinueOrNew => handle_prompt_key(app, code),
        Screen::Intro => handle_intro_key(app, code),
        Screen::Ending => handle_prompt_key(app, code),
        Screen::Waiting => handle_prompt_key(app, code),
    }
}

fn handle_game_key(app: &mut App, code: KeyCode) {
    // If typewriter is active, any key skips (Esc opens menu)
    if let Some(ref mut tw) = app.typewriter {
        if !tw.is_done() {
            match code {
                KeyCode::Esc => {
                    // Open pause menu — typewriter pauses (no skip)
                    app.overlay = Overlay::PauseMenu;
                    app.menu_index = 0;
                }
                _ => tw.skip(),
            }
            return;
        }
    }

    // If we're showing choices
    if !app.choices.is_empty() {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.choice_index > 0 {
                    app.choice_index -= 1;
                } else {
                    app.choice_index = app.choices.len() - 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.choice_index = (app.choice_index + 1) % app.choices.len();
            }
            KeyCode::Enter => {
                app.select_choice();
            }
            KeyCode::Esc => {
                app.overlay = Overlay::PauseMenu;
                app.menu_index = 0;
            }
            _ => {}
        }
        return;
    }

    // Esc always opens menu
    if code == KeyCode::Esc {
        app.overlay = Overlay::PauseMenu;
        app.menu_index = 0;
    }
}

fn handle_pause_menu_key(app: &mut App, code: KeyCode) {
    let items = 3; // Resume, Change Language, Save & Quit
    match code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.menu_index > 0 {
                app.menu_index -= 1;
            } else {
                app.menu_index = items - 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.menu_index = (app.menu_index + 1) % items;
        }
        KeyCode::Enter => {
            match app.menu_index {
                0 => {
                    // Resume
                    app.resume_from_overlay();
                }
                1 => {
                    // Change language
                    let new_lang = match app.game_state.language {
                        Language::En => Language::Fr,
                        Language::Fr => Language::En,
                    };
                    app.game_state.language = new_lang;
                    let _ = save_game(&app.game_state);
                    app.chat.push(ChatEntry::System(
                        sys_msg(Msg::LanguageSwitched, new_lang).to_string(),
                    ));
                    app.resume_from_overlay();
                }
                2 => {
                    // Save & Quit
                    let _ = save_game(&app.game_state);
                    app.chat.push(ChatEntry::System(
                        sys_msg(Msg::SavedAndQuit, app.lang()).to_string(),
                    ));
                    app.should_quit = true;
                    app.overlay = Overlay::None;
                }
                _ => {}
            }
        }
        KeyCode::Esc => {
            app.resume_from_overlay();
        }
        _ => {}
    }
}

fn handle_prompt_key(app: &mut App, code: KeyCode) {
    let count = app.prompt_options.len();
    if count == 0 {
        return;
    }
    match code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.prompt_index > 0 {
                app.prompt_index -= 1;
            } else {
                app.prompt_index = count - 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.prompt_index = (app.prompt_index + 1) % count;
        }
        KeyCode::Enter => {
            match app.screen {
                Screen::LanguageSelect => {
                    let lang = if app.prompt_index == 0 {
                        Language::En
                    } else {
                        Language::Fr
                    };
                    app.game_state.language = lang;
                    // Transition to intro
                    app.screen = Screen::Intro;
                    let intro_text = sys_msg(Msg::IntroRadioCrackle, lang).to_string();
                    app.intro_typewriter = Some(TypewriterState::new(intro_text));
                    // No typing indicator for intro
                    if let Some(ref mut tw) = app.intro_typewriter {
                        tw.show_typing_indicator = false;
                    }
                }
                Screen::ContinueOrNew => {
                    if app.prompt_index == 0 {
                        // Continue — just go to game
                        app.screen = Screen::Game;
                        app.advance_story = true;
                    } else {
                        // New game — go to language select
                        app.screen = Screen::LanguageSelect;
                        app.prompt_options = vec![
                            sys_msg(Msg::LanguageOption1, Language::En).to_string(),
                            sys_msg(Msg::LanguageOption2, Language::En).to_string(),
                        ];
                        app.prompt_index = 0;
                        app.game_state = GameState::new(Language::En);
                        app.chat.clear();
                    }
                }
                Screen::Ending => {
                    if app.prompt_index == 0 {
                        // Play again
                        let _ = crate::game::delete_save();
                        app.game_state = GameState::new(Language::En);
                        app.chat.clear();
                        app.ending_reached = None;
                        app.screen = Screen::LanguageSelect;
                        app.prompt_options = vec![
                            sys_msg(Msg::LanguageOption1, Language::En).to_string(),
                            sys_msg(Msg::LanguageOption2, Language::En).to_string(),
                        ];
                        app.prompt_index = 0;
                    } else {
                        // Quit
                        let _ = crate::game::delete_save();
                        app.should_quit = true;
                    }
                }
                Screen::Waiting => {
                    if app.prompt_index == 0 {
                        // Wait — we can't really block, so just quit and come back
                        app.should_quit = true;
                    } else {
                        // Quit
                        let _ = save_game(&app.game_state);
                        app.should_quit = true;
                    }
                }
                _ => {}
            }
        }
        KeyCode::Esc => {
            // Esc opens pause menu on game-like screens
            if app.screen == Screen::Waiting {
                app.overlay = Overlay::PauseMenu;
                app.menu_index = 0;
            }
        }
        _ => {}
    }
}

fn handle_intro_key(app: &mut App, code: KeyCode) {
    if let Some(ref mut tw) = app.intro_typewriter {
        if !tw.is_done() {
            tw.skip();
            return;
        }
    }
    // Intro is done — any key proceeds to game
    match code {
        _ => {
            app.screen = Screen::Game;
            app.advance_story = true;
            app.intro_typewriter = None;

            // Log session start
            let now = chrono::Utc::now();
            let label = now.format("%Y-%m-%d %H:%M").to_string();
            app.game_state.message_log.push(LogEntry {
                sender: Sender::System,
                text: format!("SESSION:{}", label),
                timestamp: now,
            });
            app.chat.push(ChatEntry::Separator(label));
        }
    }
}

// ── Tick (animation update) ──────────────────────────────────

/// Called on each frame to advance animations.
pub fn tick(app: &mut App) {
    // Don't advance anything while an overlay is open
    if app.overlay != Overlay::None {
        return;
    }

    // Advance typewriter
    if let Some(ref mut tw) = app.typewriter {
        tw.tick();
        if tw.is_done() {
            app.on_message_complete();
        }
    }

    // Post-message pause
    if let Some(start) = app.post_message_pause {
        if start.elapsed() >= Duration::from_millis(300) {
            app.post_message_pause = None;
            app.start_next_message();
        }
    }

    // Advance intro typewriter
    if let Some(ref mut tw) = app.intro_typewriter {
        tw.tick();
    }

    // Advance story if needed
    if app.advance_story
        && app.typewriter.is_none()
        && app.post_message_pause.is_none()
        && app.screen == Screen::Game
    {
        app.process_current_node();
    }
}

// ── Rendering ────────────────────────────────────────────────

/// Main render function.
pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::LanguageSelect => {
            draw_prompt_screen(frame, app, sys_msg(Msg::LanguagePrompt, Language::En))
        }
        Screen::ContinueOrNew => {
            draw_prompt_screen(frame, app, sys_msg(Msg::ContinueOrNew, app.lang()))
        }
        Screen::Intro => draw_intro(frame, app),
        Screen::Game => draw_game(frame, app),
        Screen::Waiting => draw_waiting(frame, app),
        Screen::Ending => draw_ending(frame, app),
    }

    // Draw overlay on top
    if app.overlay == Overlay::PauseMenu {
        draw_pause_menu(frame, app);
    }
}

fn draw_game(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Layout: chat area + status bar
    let [chat_area, status_area] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(area);

    // Build chat lines
    let mut lines: Vec<Line> = Vec::new();

    // Banner
    lines.push(Line::from("").centered());
    lines.push(
        Line::from(Span::styled(
            "E S H A R A",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .centered(),
    );
    lines.push(Line::from("").centered());
    lines.push(
        Line::from(Span::styled(
            "─".repeat(40),
            Style::default().fg(Color::DarkGray),
        ))
        .centered(),
    );
    lines.push(Line::from("").centered());

    // Chat entries
    for entry in &app.chat {
        match entry {
            ChatEntry::Elara(text) => {
                lines.push(Line::from(vec![
                    Span::styled(
                        "  Elara: ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(text.as_str(), Style::default().fg(Color::Cyan)),
                ]));
            }
            ChatEntry::Player(text) => {
                lines.push(
                    Line::from(vec![Span::styled(
                        format!("  {} >", text),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )])
                    .right_aligned(),
                );
            }
            ChatEntry::System(text) => {
                lines.push(
                    Line::from(Span::styled(
                        text.as_str(),
                        Style::default().fg(Color::DarkGray),
                    ))
                    .centered(),
                );
            }
            ChatEntry::Separator(label) => {
                lines.push(Line::from("").centered());
                lines.push(
                    Line::from(Span::styled(
                        format!("── {} ──", label),
                        Style::default().fg(Color::DarkGray),
                    ))
                    .centered(),
                );
                lines.push(Line::from("").centered());
            }
        }
        lines.push(Line::from("")); // spacing between messages
    }

    // Current typewriter message
    if let Some(ref tw) = app.typewriter {
        let lang = app.lang();
        if tw.is_indicating() {
            let elapsed = tw.indicator_start.elapsed().as_millis() as usize;
            let dots = ".".repeat((elapsed / ANIM_FRAME_MS as usize) % 3 + 1);
            lines.push(Line::from(Span::styled(
                format!("  {}{}", sys_msg(Msg::ElaraTyping, lang), dots),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            )));
        } else {
            let visible = tw.visible_text();
            if !visible.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(
                        "  Elara: ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(visible, Style::default().fg(Color::Cyan)),
                ]));
            }
        }
        lines.push(Line::from(""));
    }

    // Choices
    if !app.choices.is_empty() && app.typewriter.is_none() && app.post_message_pause.is_none() {
        lines.push(Line::from(""));
        for (i, choice) in app.choices.iter().enumerate() {
            let (prefix, style) = if i == app.choice_index {
                (
                    "  > ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    "    ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::DIM),
                )
            };
            lines.push(Line::from(Span::styled(
                format!("{}{}", prefix, choice),
                style,
            )));
        }
    }

    let text = Text::from(lines);
    let chat_height = chat_area.height as usize;
    let total_lines = wrapped_line_count(&text, chat_area.width);
    let scroll = if total_lines > chat_height {
        (total_lines - chat_height) as u16
    } else {
        0
    };

    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    frame.render_widget(paragraph, chat_area);

    // Status bar
    let hint = format!(
        "[Esc] {}",
        sys_msg(Msg::PauseMenuHint, app.lang()).trim_start_matches("[Esc] ")
    );
    let status = Line::from(Span::styled(
        format!(" {}", hint),
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(Paragraph::new(status), status_area);
}

fn draw_pause_menu(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let lang = app.lang();

    // Centered popup
    let popup_width = 40u16.min(area.width.saturating_sub(4));
    let popup_height = 9u16.min(area.height.saturating_sub(4));
    let popup_area = centered_rect(popup_width, popup_height, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title(format!(
            " {} ",
            sys_msg(Msg::PauseMenuTitle, lang).trim_matches('-').trim()
        ))
        .title_alignment(ratatui::layout::Alignment::Center)
        .padding(Padding::new(1, 1, 1, 0));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let items = vec![
        sys_msg(Msg::MenuResume, lang),
        sys_msg(Msg::MenuChangeLanguage, lang),
        sys_msg(Msg::MenuSaveQuit, lang),
    ];

    let mut lines = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let (prefix, style) = if i == app.menu_index {
            (
                "> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            (
                "  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::DIM),
            )
        };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, item),
            style,
        )));
    }

    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text), inner);
}

fn draw_prompt_screen(frame: &mut Frame, app: &App, title: &str) {
    let area = frame.area();

    let [_top, center, _bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(app.prompt_options.len() as u16 + 6),
        Constraint::Fill(1),
    ])
    .areas(area);

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "E S H A R A",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .centered(),
    );
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(title, Style::default().fg(Color::DarkGray))).centered());
    lines.push(Line::from(""));

    for (i, opt) in app.prompt_options.iter().enumerate() {
        let (prefix, style) = if i == app.prompt_index {
            (
                "> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            (
                "  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::DIM),
            )
        };
        lines.push(Line::from(Span::styled(format!("{}{}", prefix, opt), style)).centered());
    }

    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), center);
}

fn draw_intro(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "E S H A R A",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .centered(),
    );
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "─".repeat(40),
            Style::default().fg(Color::DarkGray),
        ))
        .centered(),
    );
    lines.push(Line::from(""));

    if let Some(ref tw) = app.intro_typewriter {
        let visible = tw.visible_text();
        for line in visible.lines() {
            lines.push(
                Line::from(Span::styled(line, Style::default().fg(Color::DarkGray))).centered(),
            );
        }

        if tw.is_done() {
            lines.push(Line::from(""));
            lines.push(Line::from(""));
            let hint = if app.lang() == Language::Fr {
                "Appuyez sur une touche..."
            } else {
                "Press any key..."
            };
            lines.push(
                Line::from(Span::styled(
                    hint,
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::DIM),
                ))
                .centered(),
            );
        }
    }

    let [_top, center, _bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(lines.len() as u16),
        Constraint::Fill(1),
    ])
    .areas(area);

    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), center);
}

fn draw_waiting(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "E S H A R A",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .centered(),
    );
    lines.push(Line::from(""));

    if let Some(ref msg) = app.wait_message {
        for line in msg.lines() {
            lines.push(
                Line::from(Span::styled(line, Style::default().fg(Color::DarkGray))).centered(),
            );
        }
    }

    lines.push(Line::from(""));

    for (i, opt) in app.prompt_options.iter().enumerate() {
        let (prefix, style) = if i == app.prompt_index {
            (
                "> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            (
                "  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::DIM),
            )
        };
        lines.push(Line::from(Span::styled(format!("{}{}", prefix, opt), style)).centered());
    }

    let [_top, center, _bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(lines.len() as u16),
        Constraint::Fill(1),
    ])
    .areas(area);

    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), center);
}

fn draw_ending(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let lang = app.lang();

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "E S H A R A",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .centered(),
    );
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "─".repeat(40),
            Style::default().fg(Color::DarkGray),
        ))
        .centered(),
    );
    lines.push(Line::from(""));

    lines.push(
        Line::from(Span::styled(
            format!("--- {} ---", sys_msg(Msg::EndingReached, lang)),
            Style::default().fg(Color::DarkGray),
        ))
        .centered(),
    );
    lines.push(Line::from(""));

    if let Some(ref ending) = app.ending_reached {
        if let Some(info) = app.story_data.ending_info(ending) {
            lines.push(
                Line::from(Span::styled(
                    format!("\"{}\"", info.title.get(lang)),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .centered(),
            );
            lines.push(Line::from(""));
            lines.push(
                Line::from(Span::styled(
                    info.description.get(lang).to_string(),
                    Style::default().fg(Color::DarkGray),
                ))
                .centered(),
            );
        }
    }

    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            format!(
                "{} {}",
                sys_msg(Msg::DaysSurvived, lang),
                app.game_state.day
            ),
            Style::default().fg(Color::DarkGray),
        ))
        .centered(),
    );
    lines.push(Line::from(""));
    lines.push(
        Line::from(Span::styled(
            "─".repeat(40),
            Style::default().fg(Color::DarkGray),
        ))
        .centered(),
    );
    lines.push(Line::from(""));

    lines.push(
        Line::from(Span::styled(
            sys_msg(Msg::PlayAgain, lang),
            Style::default().fg(Color::DarkGray),
        ))
        .centered(),
    );
    lines.push(Line::from(""));

    for (i, opt) in app.prompt_options.iter().enumerate() {
        let (prefix, style) = if i == app.prompt_index {
            (
                "> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            (
                "  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::DIM),
            )
        };
        lines.push(Line::from(Span::styled(format!("{}{}", prefix, opt), style)).centered());
    }

    let [_top, center, _bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(lines.len() as u16),
        Constraint::Fill(1),
    ])
    .areas(area);

    let text = Text::from(lines);
    frame.render_widget(Paragraph::new(text).wrap(Wrap { trim: false }), center);
}

/// Estimate the number of visual lines a `Text` will occupy when wrapped to `width`.
fn wrapped_line_count(text: &Text, width: u16) -> usize {
    if width == 0 {
        return text.lines.len();
    }
    let w = width as usize;
    text.lines
        .iter()
        .map(|line| {
            let line_width: usize = line.spans.iter().map(|s| s.content.len()).sum();
            if line_width == 0 {
                1 // empty lines still take one row
            } else {
                (line_width + w - 1) / w // ceil division
            }
        })
        .sum()
}

/// Helper: create a centered rect of given width/height within an area.
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

// ── Main event loop ──────────────────────────────────────────

/// Run the ratatui event loop. This is the main entry point for the UI.
pub fn run(mut app: App, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let tick_rate = Duration::from_millis(30);

    loop {
        // Draw
        terminal.draw(|frame| draw(frame, &app))?;

        // Check quit
        if app.should_quit {
            break;
        }

        // Poll events
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not release/repeat)
                if key.kind == KeyEventKind::Press {
                    handle_key(&mut app, key.code);
                }
            }
        }

        // Tick animations
        tick(&mut app);

        // Check Ctrl+C flag
        if crate::is_interrupted() {
            let _ = save_game(&app.game_state);
            break;
        }
    }

    Ok(())
}
