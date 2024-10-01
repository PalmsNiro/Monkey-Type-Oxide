use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, style::{Color, Style}, Terminal};
use std::io;

use crate::ui::draw_ui;

pub struct App {
    pub target_text: String,
    pub colored_chars: Vec<(char, Style)>,
    pub user_input: String,
    pub index: usize,
    pub mistakes: usize,
    pub total_chars: usize,
}

impl App {
    pub fn new(target_text: String) -> Self {
        let colored_chars = target_text
            .chars()
            .map(|c| (c, Style::default().fg(Color::DarkGray)))
            .collect();

        Self {
            target_text,
            colored_chars,
            user_input: String::new(),
            index: 0,
            mistakes: 0,
            total_chars: 0,  // Initialisierung des neuen Feldes
        }
    }

    pub fn type_char(&mut self, c: char) {
        if let Some(target_char) = self.target_text.chars().nth(self.index) {
            self.user_input.push(c);
            self.total_chars += 1;  // Inkrementierung der Gesamtanzahl der getippten Zeichen
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

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        loop {
            terminal.draw(|f| draw_ui(f, &self))?;
    
            if let event::Event::Key(key) = event::read()? {
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
            }
        }
    }
}
