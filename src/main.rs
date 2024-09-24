use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::{io, ops::Index};

fn main() -> io::Result<()> {
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;
    let app_result = run(&mut terminal);
    terminal.clear()?;
    drop(terminal);
    app_result
}

fn run(terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
    let string_to_type = String::from("This will be the String for the Terminal Type Speed Test! Lets see how fast you can type what is standing here.");
    let mut user_input = String::new();
    let mut index = 0;
    let mut mistakes = 0;

    let mut colored_chars: Vec<(char, Style)> = string_to_type
        .chars()
        .map(|c| (c, Style::default().fg(Color::DarkGray)))
        .collect();

    loop {
        terminal.draw(|frame| {
            let target_char = string_to_type.chars().nth(index).unwrap_or(' ');
            let info_text = format!(
                "Fehler: {}, Aktueller Index: {}, Zeichen: {}",
                mistakes, index, target_char
            );

            let colored_text: Vec<Span> = colored_chars
                .iter()
                .enumerate()
                .map(|(i, (c, style))| {
                    if i == index {
                        // Cursor-Effekt: Gelber Hintergrund für das aktuelle Zeichen
                        Span::styled(
                            c.to_string(),
                            style.clone().bg(Color::Yellow).fg(Color::Black)
                        )
                    } else {
                        Span::styled(c.to_string(), *style)
                    }
                })
                .collect();

            let info = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL).title("Info"));

            // let input_display = Paragraph::new(user_input.as_str())
            // .block(Block::default().borders(Borders::ALL).title("Ihre Eingabe"));

            let target_text = Paragraph::new(Line::from(colored_text))
                .block(Block::default().borders(Borders::ALL).title("Zieltext"));

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(frame.area());

            frame.render_widget(info, chunks[0]);
            frame.render_widget(target_text, chunks[1]);
            // frame.render_widget(input_display, chunks[2]);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                    if c == 'q' && user_input.is_empty() {
                        return Ok(());
                    } else {
                        user_input.push(c);
                        if let Some(target_char) = string_to_type.chars().nth(index) {
                            if c == target_char {
                                // Ändere die Farbe des korrekten Zeichens auf Weiß
                                if let Some((_, style)) = colored_chars.get_mut(index) {
                                    *style = Style::default().fg(Color::White);
                                }
                            } else {
                                // Optional: Ändere die Farbe des falschen Zeichens auf Rot
                                if let Some((_, style)) = colored_chars.get_mut(index) {
                                    *style = Style::default().fg(Color::Red);
                                }
                                mistakes += 1;
                            }
                            index += 1;
                        }
                    }
                }
                KeyCode::Backspace if key.kind == KeyEventKind::Press => {
                    // Entfernen Sie das letzte Zeichen, wenn Backspace gedrückt wird
                    if !user_input.is_empty() {
                        user_input.pop();
                        if index > 0 {
                            index -= 1;
                            // Setze die Farbe des gelöschten Zeichens zurück auf Grau
                            if let Some((_, style)) = colored_chars.get_mut(index) {
                                *style = Style::default().fg(Color::DarkGray);
                            }
                        }
                    }
                }
                KeyCode::Esc if key.kind == KeyEventKind::Press => {
                    return Ok(());
                }
                _ => {} // Ignorieren Sie alle anderen Tasten
            }
        }
    }
}
