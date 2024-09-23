use std::io;
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Block, Borders},
    style::{Color, Style, Modifier},
};
use crossterm::event::{self, KeyCode, KeyEventKind};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;
    let app_result = run(&mut terminal);
    terminal.clear()?;
    drop(terminal);
    app_result
}

fn run(terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
    let string_to_type = String::from("This will be the String for the Terminal Type Speed Test! Lets see how fast you can type what is standing here.");
    
    let colored_chars: Vec<(char, Style)> = string_to_type
        .chars()
        .enumerate()
        .map(|(i, c)| {
            match c {
                'T' => (c, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                'S' => (c, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                '!' => (c, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                _ if i % 2 == 0 => (c, Style::default().fg(Color::Cyan)),
                _ => (c, Style::default().fg(Color::White)),
            }
        })
        .collect();

    loop {
        terminal.draw(|frame| {
            let colored_text = colored_chars
                .iter()
                .map(|(c, style)| {
                    Span::styled(c.to_string(), *style)
                })
                .collect::<Vec<_>>();

            let paragraph = Paragraph::new(Line::from(colored_text))
                .block(Block::default().borders(Borders::ALL).style(Style::default().bg(Color::Black)));

            frame.render_widget(paragraph, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                return Ok(());
            }
        }
    }
}