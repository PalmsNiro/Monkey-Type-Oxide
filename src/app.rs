use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
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

#[derive(Default)]
pub struct OptionsState {
    pub selected_option: usize,
    options_count: usize,
}

impl OptionsState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options_count: 5, //TODO Make Sure This matches the listed amount of options in the front end
        }
    }

    pub fn next(&mut self) {
        self.selected_option = (self.selected_option + 1) % self.options_count;
    }

    pub fn previous(&mut self) {
        if self.selected_option > 0 {
            self.selected_option -= 1;
        } else {
            self.selected_option = self.options_count - 1;
        }
    }
}

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
    options_state: OptionsState,
}

impl App {
    pub fn new() -> Self {
        let opt = AppOptions::new();
        Self {
            options: opt.clone(),
            typing_test: TypingTest::new(opt.test_language.clone(), opt.test_type.clone()),
            state: AppState::StartScreen,
            selected_tab: SelectedTab::Tab1,
            options_state: OptionsState::new(),
        }
    }

    fn start_new_test(&mut self) {
        self.typing_test.reset(self.options.test_language,self.options.test_type);  // Reset Test
        self.state = AppState::StartScreen; // Reset App-State
    }

    fn handle_key_event(&mut self) -> Result<(), io::Error> {
        if let event::Event::Key(key) = event::read()? {
            // Gemeinsame Shortcuts für alle Tabs
            if key.kind == KeyEventKind::Press {
                // Prüfe zuerst auf CONTROL-Kombinationen
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('q') => exit_app(),
                        KeyCode::Char('l') => self.next_tab(),
                        KeyCode::Char('h') => self.previous_tab(),
                        _ => {}
                    }
                } else {
                    // Wenn kein CONTROL, dann normale Eingabebehandlung
                    match key.code {
                        KeyCode::Esc => exit_app(),
                        _ => {
                            // Tab-specific Inputhandling
                            match self.selected_tab {
                                SelectedTab::Tab1 => self.handle_typing_input(key),
                                SelectedTab::Tab2 => self.handle_options_input(key),
                                SelectedTab::Tab3 => self.handle_account_input(key),
                                SelectedTab::Tab4 => self.handle_about_input(key),
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_typing_input(&mut self, key: KeyEvent) {
        match self.state {
            AppState::StartScreen | AppState::RunningTest => {
                match (key.code, key.modifiers) {
                    // Erlaube SHIFT Modifier für Großbuchstaben
                    (KeyCode::Char(c), KeyModifiers::NONE) | 
                    (KeyCode::Char(c), KeyModifiers::SHIFT) => self.typing_test.type_char(c),
                    (KeyCode::Backspace, KeyModifiers::NONE) => self.typing_test.backspace(),
                    _ => {}
                }
            }
            AppState::EndScreen => {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('r'), KeyModifiers::NONE) |
                    (KeyCode::Char('r'), KeyModifiers::SHIFT) |  // Erlaubt auch 'R'
                    (KeyCode::Char('R'), KeyModifiers::SHIFT) => self.start_new_test(),
                    _ => {}
                }
            }
        }
    }

    fn handle_options_input(&mut self, key: event::KeyEvent) {
        if key.modifiers == KeyModifiers::NONE {
            match key.code {
                KeyCode::Up => self.options_state.previous(),
                KeyCode::Char('k') => self.options_state.previous(),
                KeyCode::Down => self.options_state.next(),
                KeyCode::Char('j') => self.options_state.next(),
                KeyCode::Left => self.change_option_value(false),
                KeyCode::Char('h') => self.change_option_value(false),
                KeyCode::Right => self.change_option_value(true),
                KeyCode::Char('l') => self.change_option_value(true),
                _ => {}
            }
        }
    }

    fn handle_account_input(&mut self, key: event::KeyEvent) {}

    fn handle_about_input(&mut self, key: event::KeyEvent) {}

    fn change_option_value(&mut self, increase: bool) {
        match self.options_state.selected_option {
            0 => self.change_test_language(increase),
            1 => self.change_test_type(increase),
            2 => {self.options.time_race  =!self.options.time_race} // Time Race 
            3 => {self.options.hardcore  =!self.options.hardcore} // Hardcore
            4 => self.change_ui_language(increase), // UI Language 
            _=>{}
        }
        //TODO dont reset test on every change 
        //*-> change when returning to test screen , set flag here that test needs reset ->When returning to startscreen reset
        self.typing_test.reset(self.options.test_language,self.options.test_type); 
    }

    fn change_test_language(&mut self, increase: bool) {
        if increase{
            self.options.test_language = self.options.test_language.next();
        }else{
            self.options.test_language = self.options.test_language.previous();
        }
    }

    fn change_test_type(&mut self, increase: bool) {
        // Implementierung für das Ändern des Testtyps
        if increase{
            self.options.test_type = self.options.test_type.next();
        }else{
            self.options.test_type = self.options.test_type.previous();
        }
    }

    fn change_ui_language(&mut self, increase: bool) {
        // Implementierung für das Ändern des Testtyps
        if increase{
            self.options.ui_language = self.options.ui_language.next();
        }else{
            self.options.ui_language = self.options.ui_language.previous();
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

            // terminal.draw(|f| draw_ui(f, &self.typing_test, &self.state))?;
            terminal.draw(|f| {
                draw_ui(
                    f,
                    &self.state,
                    &self.selected_tab,
                    &self.typing_test,
                    &self.options,
                    &self.options_state,
                )
            })?;

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
    // deactivate  Raw mode 
    disable_raw_mode()?;

    // clearing Terminal
    stdout().execute(Clear(ClearType::All))?;

    // Optional: Set Cursor to  beginning
    stdout().execute(crossterm::cursor::MoveTo(0, 0))?;

    Ok(())
}

fn exit_app() {
    if let Err(e) = cleanup_terminal() {
        eprintln!("Error cleaning up terminal: {}", e);
    }
    process::exit(0);
}
