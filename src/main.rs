mod app;
mod game;
mod ui;

use ratatui::prelude::*;
use std::io;
// use std::thread::Builder;

use std::fs::File;
use log::{info, error, debug, warn};
use env_logger::Builder;
use chrono::Local;
use std::io::Write;


use crate::app::App;

fn main() -> io::Result<()> {
    // setup logger
    if let Err(e) = setup_logger() {
        eprintln!("Failed to set up logger: {}", e);
        return Ok(());
    }
    info!("Startet application");

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

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let logfile = File::create(format!("logs/game_log_{}.txt", Local::now().format("%Y-%m-%d_%H-%M-%S")))?;

    Builder::new()
        .target(env_logger::Target::Pipe(Box::new(logfile)))
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    Ok(())
}
