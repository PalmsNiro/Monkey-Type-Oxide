mod app;
mod ui;
mod type_test;
mod options;

use ratatui::prelude::*;
use std::io;

use crate::app::App;

fn main() -> io::Result<()> {
    
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;

    let mut app = App::new();

    let result = app.run(&mut terminal);

    terminal.clear()?;
    terminal.show_cursor()?;

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;

    result
}