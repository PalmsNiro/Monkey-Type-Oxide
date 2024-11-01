use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};
use std::{io, process};

use crate::{options::Options, type_test::TypingTest, ui::draw_ui};

pub enum AppState {
    StartScreen,
    RunningTest,
    EndScreen,
}

pub struct App {
    pub options: Options,
    pub typing_test: TypingTest,
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        let opt = Options::new();
        Self {
            options: opt.clone(),
            typing_test: TypingTest::new(
                opt.words_amount.clone() as usize,
                opt.test_language.clone(),
                opt.test_type.clone(),
            ),
            state: AppState::StartScreen,
        }
    }

    fn reset(&mut self) {
        self.typing_test.reset(); // Reset Test
        self.state = AppState::StartScreen; // Reset App-State
    }

    fn handle_key_event(&mut self) -> Result<(), io::Error> {
        match self.state {
            AppState::EndScreen => Ok(if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                        if c == 'r' || c == 'R' {
                            self.reset();
                        }
                        if c == 'q' || c == 'Q' {
                            process::exit(0);
                        }
                    }
                    _ => {}
                }
            }),
            _ => Ok(()),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        loop {
            terminal.draw(|f| draw_ui(f, &self.typing_test, &self.state))?;

            match self.state {
                AppState::StartScreen => {
                    self.typing_test.handle_key_event()?;
                    if self.typing_test.progress() > 0 {
                        self.state = AppState::RunningTest
                    }
                }
                AppState::RunningTest => {
                    self.typing_test.handle_key_event()?;
                    if self.typing_test.index == self.typing_test.target_text.len() {
                        self.typing_test.text_finished = true;
                    }
                    if self.typing_test.text_finished {
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
