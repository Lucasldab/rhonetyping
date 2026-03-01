use std::time::{Duration, Instant};
use crate::snippets::{Language, MENU_OPTIONS, random_snippet};

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Menu,
    Typing,
    Results,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CharState {
    Untyped,
    Correct,
    Wrong,
}

pub struct App {
    pub screen: Screen,
    pub selected_menu: usize,
    pub language: Language,

    // Typing state
    pub snippet: String,
    pub chars: Vec<char>,
    pub char_states: Vec<CharState>,
    pub cursor: usize,
    pub errors: usize,

    // Timing
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,

    // Live stats
    pub wpm: f64,
    pub accuracy: f64,

    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let language = Language::English;
        let snippet = random_snippet(language);
        let chars: Vec<char> = snippet.chars().collect();
        let len = chars.len();
        App {
            screen: Screen::Menu,
            selected_menu: 0,
            language,
            snippet,
            chars,
            char_states: vec![CharState::Untyped; len],
            cursor: 0,
            errors: 0,
            started_at: None,
            finished_at: None,
            wpm: 0.0,
            accuracy: 100.0,
            should_quit: false,
        }
    }

    fn load_snippet(&mut self) {
        self.snippet = random_snippet(self.language);
        self.chars = self.snippet.chars().collect();
        let len = self.chars.len();
        self.char_states = vec![CharState::Untyped; len];
        self.cursor = 0;
        self.errors = 0;
        self.started_at = None;
        self.finished_at = None;
        self.wpm = 0.0;
        self.accuracy = 100.0;
    }

    pub fn menu_next(&mut self) {
        self.selected_menu = (self.selected_menu + 1) % MENU_OPTIONS.len();
    }

    pub fn menu_prev(&mut self) {
        if self.selected_menu == 0 {
            self.selected_menu = MENU_OPTIONS.len() - 1;
        } else {
            self.selected_menu -= 1;
        }
    }

    pub fn start_session(&mut self) {
        self.language = MENU_OPTIONS[self.selected_menu];
        self.load_snippet();
        self.screen = Screen::Typing;
    }

    pub fn go_to_menu(&mut self) {
        self.screen = Screen::Menu;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn restart_session(&mut self) {
        self.load_snippet();
        self.screen = Screen::Typing;
    }

    pub fn new_snippet(&mut self) {
        self.load_snippet();
        self.screen = Screen::Typing;
    }

    pub fn type_char(&mut self, c: char) {
        if self.cursor >= self.chars.len() {
            return;
        }
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }

        if self.chars[self.cursor] == c {
            self.char_states[self.cursor] = CharState::Correct;
        } else {
            self.char_states[self.cursor] = CharState::Wrong;
            self.errors += 1;
        }
        self.cursor += 1;
        self.update_stats();

        if self.cursor == self.chars.len() {
            self.finished_at = Some(Instant::now());
            self.screen = Screen::Results;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        if self.char_states[self.cursor] == CharState::Wrong {
            self.errors = self.errors.saturating_sub(1);
        }
        self.char_states[self.cursor] = CharState::Untyped;
        self.update_stats();
    }

    pub fn tick(&mut self) {
        if self.started_at.is_some() && self.finished_at.is_none() {
            self.update_stats();
        }
    }

    fn update_stats(&mut self) {
        if let Some(start) = self.started_at {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                // WPM: chars typed / 5 (standard word length) / minutes
                let correct_chars = self.char_states.iter()
                    .filter(|s| **s == CharState::Correct)
                    .count() as f64;
                self.wpm = (correct_chars / 5.0) / (elapsed / 60.0);
            }
            let typed = self.cursor;
            if typed > 0 {
                self.accuracy = ((typed - self.errors) as f64 / typed as f64) * 100.0;
            }
        }
    }

    pub fn elapsed(&self) -> Duration {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => end.duration_since(start),
            (Some(start), None) => start.elapsed(),
            _ => Duration::ZERO,
        }
    }

    pub fn progress(&self) -> f64 {
        if self.chars.is_empty() { return 0.0; }
        self.cursor as f64 / self.chars.len() as f64
    }
}
