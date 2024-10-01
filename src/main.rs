mod app;
mod ui;
mod game;

// use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use std::io;

use crate::app::App;
// use crate::ui::draw_ui;
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

// fn run(terminal: &mut Terminal<impl Backend>, app: &mut App) -> io::Result<()> {
//     loop {
//         terminal.draw(|f| draw_ui(f, app))?;

//         if let event::Event::Key(key) = event::read()? {
//             match key.code {
//                 KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
//                     app.type_char(c);
//                 }
//                 KeyCode::Backspace if key.kind == KeyEventKind::Press => {
//                     app.backspace();
//                 }
//                 KeyCode::Esc if key.kind == KeyEventKind::Press => {
//                     return Ok(());
//                 }
//                 _ => {}
//             }
//         }
//     }
// }