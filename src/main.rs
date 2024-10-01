mod app;
mod game;
mod ui;

use ratatui::prelude::*;
use std::io;

use crate::app::App;
use crate::game::get_random_sentence;

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;

    let mut app = App::new(get_random_sentence(30));

    let result = app.run(&mut terminal);

    terminal.clear()?;
    terminal.show_cursor()?;

    result
}
