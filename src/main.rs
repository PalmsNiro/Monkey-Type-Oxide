mod app;
mod game;
mod ui;

use ratatui::prelude::*;
use std::fs::File;
use std::io;
// use std::thread::Builder;

use log::{info, error, debug, warn};
use env_logger::Builder;
use chrono::Local;
use std::io::Write;

use crate::app::App;
use crate::game::get_random_sentence;

fn main() -> io::Result<()> {
    //Logger setup
    if let Err(e) = setup_logger() {
        eprintln!("Failed to set up logger: {}", e);
        return Ok(());
    }

    info!("starting the Applications");
    
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;

    let mut app = App::new(get_random_sentence(30));

    let result = app.run(&mut terminal);

    terminal.clear()?;
    terminal.show_cursor()?;

    result
}

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let logfile = File::create(format!("game_log_{}.txt", Local::now().format("%Y-%m-%d_%H-%M-%S")))?;

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