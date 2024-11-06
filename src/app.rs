use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use log::info;
use ratatui::{prelude::Backend, widgets::TableState, Terminal};
use std::{
    io, process, thread,
    time::{Duration, Instant},
};

use crate::{
    options::Options,
    type_test::{TestDataPerSecond, TypingTest},
    ui::draw_ui,
};


#[derive(Default, Clone)]
pub enum AppState {
    #[default]
    StartScreen,
    RunningTest,
    EndScreen,
}

// #[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
// enum SelectedTab {
//     #[default]
//     #[strum(to_string = "Tab 1")] // start screen / test area / end screen
//     Tab1,
//     #[strum(to_string = "Tab 2")] // options
//     Tab2,
//     #[strum(to_string = "Tab 3")] // account
//     Tab3,
//     #[strum(to_string = "Tab 4")] // about
//     Tab4,
// }
// impl SelectedTab {
//     /// Get the previous tab, if there is no previous tab return the current tab.
//     fn previous(self) -> Self {
//         let current_index: usize = self as usize;
//         let previous_index = current_index.saturating_sub(1);
//         Self::from_repr(previous_index).unwrap_or(self)
//     }

//     /// Get the next tab, if there is no next tab return the current tab.
//     fn next(self) -> Self {
//         let current_index = self as usize;
//         let next_index = current_index.saturating_add(1);
//         Self::from_repr(next_index).unwrap_or(self)
//     }
// }

pub struct App {
    pub options: Options,
    pub typing_test: TypingTest,
    pub state: AppState,
    selected_tab: SelectedTab,
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
            selected_tab: SelectedTab::Tab1,
        }
    }

    fn start_new_test(&mut self) {
        self.typing_test.reset(); // Reset Test
        self.state = AppState::StartScreen; // Reset App-State
    }

    fn handle_key_event(&mut self) -> Result<(), io::Error> {
        match self.state {
            AppState::StartScreen => {
                if let event::Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                            self.typing_test.type_char(c)
                        }
                        KeyCode::Esc => process::exit(0),
                        _ => {}
                    }
                    match key.modifiers {
                        KeyModifiers::CONTROL => {
                            if key.kind == KeyEventKind::Press{
                                if key.code == KeyCode::Char('q') {process::exit(0)};
                                if key.code == KeyCode::Char('l') {self.next_tab();};
                                if key.code == KeyCode::Char('h') {self.previous_tab();};
                            }
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
            AppState::RunningTest => {
                if let event::Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                            self.typing_test.type_char(c)
                        }
                        KeyCode::Backspace => self.typing_test.backspace(),
                        KeyCode::Esc => process::exit(0),
                        _ => {}
                    }
                    match key.modifiers {
                        KeyModifiers::CONTROL
                            if (key.code == KeyCode::Char('q')
                                && key.kind == KeyEventKind::Press) =>
                        {
                            process::exit(0)
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
            AppState::EndScreen => {
                if let event::Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                            if c == 'r' || c == 'R' {
                                self.start_new_test();
                            }
                            if c == 'q' || c == 'Q' {
                                process::exit(0);
                            }
                        }
                        KeyCode::Esc => process::exit(0),
                        _ => {}
                    }
                }
                Ok(())
            }
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        let mut last_update = Instant::now();
        let update_interval = Duration::from_secs(1);

        loop {
            let now = Instant::now();

            // Update metrics every second while test is running, regardless of input(meaning also, when afk)
            if let AppState::RunningTest = self.state {
                if now.duration_since(last_update) >= update_interval {
                    self.typing_test.update_test_data();
                    last_update = now;
                }
            }

            terminal.draw(|f| draw_ui(f, &self.typing_test, &self.state))?;

            match self.state {
                AppState::StartScreen => {
                    self.handle_key_event()?;
                    if self.typing_test.progress() > 0 {
                        self.typing_test.update_test_data();
                        last_update = Instant::now();
                        self.state = AppState::RunningTest;
                    }
                }
                AppState::RunningTest => {
                    self.handle_key_event()?;
                    if self.typing_test.index == self.typing_test.target_text.len() {
                        self.typing_test.text_finished = true;
                        self.typing_test.stop_timer();
                    }
                    if self.typing_test.text_finished {
                        self.state = AppState::EndScreen
                    }
                }
                AppState::EndScreen => {
                    self.handle_key_event()?;
                }
            }

            // Optional: Delay to reduce weight on cpu
            // thread::sleep(Duration::from_millis(10));
        }
    }
}
