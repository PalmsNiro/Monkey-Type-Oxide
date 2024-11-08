use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use log::info;
use ratatui::{prelude::Backend, widgets::TableState, Terminal};
use std::{
    io::{self, stdout},
    process, thread,
    time::{Duration, Instant},
};

use crate::{
    app_options::AppOptions,
    type_test::TypingTest,
    ui::{draw_ui, tabs::SelectedTab},
};

#[derive(Default, Clone)]
pub enum AppState {
    #[default]
    StartScreen,
    RunningTest,
    EndScreen,
}

pub struct App {
    pub options: AppOptions,
    pub typing_test: TypingTest,
    pub state: AppState,
    selected_tab: SelectedTab,
}

impl App {
    pub fn new() -> Self {
        let opt = AppOptions::new();
        Self {
            options: opt.clone(),
            typing_test: TypingTest::new(opt.test_language.clone(), opt.test_type.clone()),
            state: AppState::StartScreen,
            selected_tab: SelectedTab::Tab1,
        }
    }

    fn start_new_test(&mut self) {
        self.typing_test.reset(); // Reset Test
        self.state = AppState::StartScreen; // Reset App-State
    }

    fn handle_key_event(&mut self) -> Result<(), io::Error> {
        if let event::Event::Key(key) = event::read()? {
            match self.state {
                AppState::StartScreen => {
                    if key.kind == KeyEventKind::Press {
                        match (key.code, key.modifiers) {
                            // CTRL Kombinationen
                            (KeyCode::Char('q'), KeyModifiers::CONTROL) => exit_app(),
                            (KeyCode::Char('l'), KeyModifiers::CONTROL) => self.next_tab(),
                            (KeyCode::Char('h'), KeyModifiers::CONTROL) => self.previous_tab(),

                            // Normale Tasten (nur wenn keine Modifiers gedrückt sind)
                            (KeyCode::Char(c), KeyModifiers::NONE) => self.typing_test.type_char(c),
                            (KeyCode::Esc, _) => exit_app(),
                            _ => {}
                        }
                    }
                }

                AppState::RunningTest => {
                    if key.kind == KeyEventKind::Press {
                        match (key.code, key.modifiers) {
                            // CTRL Kombinationen
                            (KeyCode::Char('q'), KeyModifiers::CONTROL) => exit_app(),
                            (KeyCode::Char('l'), KeyModifiers::CONTROL) => self.next_tab(),
                            (KeyCode::Char('h'), KeyModifiers::CONTROL) => self.previous_tab(),

                            // Normale Tasten (nur wenn keine Modifiers gedrückt sind)
                            (KeyCode::Char(c), KeyModifiers::NONE) => self.typing_test.type_char(c),
                            (KeyCode::Backspace, KeyModifiers::NONE) => {
                                self.typing_test.backspace()
                            }
                            (KeyCode::Esc, _) => exit_app(),
                            _ => {}
                        }
                    }
                }

                AppState::EndScreen => {
                    if key.kind == KeyEventKind::Press {
                        match (key.code, key.modifiers) {
                            // CTRL Kombinationen
                            (KeyCode::Char('q'), KeyModifiers::CONTROL) => exit_app(),
                            (KeyCode::Char('l'), KeyModifiers::CONTROL) => self.next_tab(),
                            (KeyCode::Char('h'), KeyModifiers::CONTROL) => self.previous_tab(),

                            // Normale Tasten
                            (KeyCode::Char('r'), KeyModifiers::NONE)
                            | (KeyCode::Char('R'), KeyModifiers::NONE) => self.start_new_test(),
                            (KeyCode::Char('q'), KeyModifiers::NONE)
                            | (KeyCode::Char('Q'), KeyModifiers::NONE) => exit_app(),
                            (KeyCode::Esc, _) => process::exit(0),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
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

            // terminal.draw(|f| draw_ui(f, &self.typing_test, &self.state))?;
            terminal.draw(|f| draw_ui(f, &self.state, &self.selected_tab, &self.typing_test))?;

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

fn cleanup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    // Raw mode deaktivieren
    disable_raw_mode()?;

    // Terminal säubern
    stdout().execute(Clear(ClearType::All))?;

    // Optional: Cursor an den Anfang setzen
    stdout().execute(crossterm::cursor::MoveTo(0, 0))?;

    Ok(())
}

fn exit_app() {
    if let Err(e) = cleanup_terminal() {
        eprintln!("Error cleaning up terminal: {}", e);
    }
    process::exit(0);
}

