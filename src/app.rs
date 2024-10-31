use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    prelude::Backend,
    style::{Color, Style},
    Terminal,
};
use std::io;

use crate::{
    game::get_random_sentence,
    ui::{draw_typing_screen, draw_ui},
};

const WORDS_AMOUNT: u16 = 15;

pub enum AppState {
    StartScreen,
    RunningTest,
    EndScreen,
}

pub struct App {
    pub target_text: String,
    pub colored_chars: Vec<(char, Style)>,
    pub user_input: String,
    pub index: usize,
    pub mistakes: usize,
    pub total_chars: usize,
    pub text_finished: bool,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        let text = get_random_sentence(WORDS_AMOUNT as usize);
        
        let colored_chars = text
            .chars()
            .map(|c| (c, Style::default().fg(Color::DarkGray)))
            .collect();

        Self {
            target_text: text,
            colored_chars,
            user_input: String::new(),
            index: 0,
            mistakes: 0,
            total_chars: 0, // Initialisierung des neuen Feldes
            text_finished: false,
            state: AppState::StartScreen,
        }
    }

    pub fn type_char(&mut self, c: char) {
        if let Some(target_char) = self.target_text.chars().nth(self.index) {
            self.user_input.push(c);
            self.total_chars += 1; // Inkrementierung der Gesamtanzahl der getippten Zeichen
            if c == target_char {
                if let Some((_, style)) = self.colored_chars.get_mut(self.index) {
                    *style = Style::default().fg(Color::Green);
                }
            } else {
                if let Some((_, style)) = self.colored_chars.get_mut(self.index) {
                    *style = Style::default().fg(Color::Red);
                }
                self.mistakes += 1;
            }
            self.index += 1;
        }
    }

    pub fn backspace(&mut self) {
        if !self.user_input.is_empty() {
            self.user_input.pop();
            if self.index > 0 {
                self.index -= 1;
                if let Some((_, style)) = self.colored_chars.get_mut(self.index) {
                    *style = Style::default().fg(Color::DarkGray);
                }
            }
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_chars == 0 {
            100.0
        } else {
            ((self.total_chars - self.mistakes) as f64 / self.total_chars as f64) * 100.0
        }
    }

    pub fn progress(&self) -> u16 {
        if self.target_text.is_empty() {
            0
        } else {
            ((self.index as f64 / self.target_text.len() as f64) * 100.0) as u16
        }
    }

    fn reset(&mut self) {
        let new_app = App::new();
        *self = new_app;
    }

    fn handle_key_event(&mut self) -> Result<(), io::Error> {
        match self.state {
            AppState::EndScreen => Ok(if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                        if c == 'r' || c == 'R' {
                            self.reset();
                            println!("reset called");
                        }
                    }
                    _ => {}
                }
            }),
            _ => Ok(if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                        self.type_char(c);
                    }
                    KeyCode::Backspace if key.kind == KeyEventKind::Press => {
                        self.backspace();
                    }
                    KeyCode::Esc if key.kind == KeyEventKind::Press => {
                        return Ok(());
                    }
                    _ => {}
                }
            }),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        loop {
            terminal.draw(|f| draw_ui(f, &self))?;

            match self.state {
                AppState::StartScreen => {
                    self.handle_key_event()?;
                    if self.progress() > 0 {
                        self.state = AppState::RunningTest
                    }
                }
                AppState::RunningTest => {
                    self.handle_key_event()?;
                    if self.index == self.target_text.len() {
                        self.text_finished = true;
                    }
                    if self.text_finished {
                        self.state = AppState::EndScreen
                    }
                }
                AppState::EndScreen => {
                    self.handle_key_event()?;
                }
            }
        }
    }
}
