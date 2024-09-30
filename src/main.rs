use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::{io, ops::Index};

use rand::seq::SliceRandom;
use rand::thread_rng;
use random_word::Lang;

use terminal_size::{terminal_size, Width};
use textwrap::{wrap, Options};

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
    let width = get_terminal_width() - 10;
    // let string_to_type = String::from("This will be the String for the Terminal Type Speed Test! Lets see how fast you can type what is standing here.");
    let string_to_type = wrap_text(&get_random_sentence(30), width);
    // println!("{}", string_to_type);
    let mut user_input = String::new();
    let mut index = 0;
    let mut mistakes = 0;

    let mut colored_chars: Vec<(char, Style)> = string_to_type
        .chars()
        .map(|c| (c, Style::default().fg(Color::DarkGray)))
        .collect();

    loop {
        terminal.draw(|f| draw_ui(f, &string_to_type, &colored_chars, index, mistakes))?;

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

fn get_random_sentence(words_amount: usize) -> String {
    let word_list: &[&str] = random_word::all(Lang::De);
    let mut rng = thread_rng();

    let selected_words: Vec<&str> = word_list
        .choose_multiple(&mut rng, words_amount)
        .cloned()
        .collect();

    let sentence = selected_words.join(" ");
    sentence.chars().nth(0).unwrap().to_uppercase().to_string() + &sentence[1..] 
}

fn wrap_text(text: &str, width: usize) -> String {
    let options = Options::new(width)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(textwrap::WordSplitter::NoHyphenation);
    wrap(text, options).join("\n")
}

fn get_terminal_width() -> usize {
    terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80)
}

fn draw_ui(
    frame: &mut Frame,
    string_to_type: &str,
    colored_chars: &[(char, Style)],
    index: usize,
    mistakes: usize,
) {
    let target_char = string_to_type.chars().nth(index).unwrap_or(' ');
    let info_text = format!(
        "Fehler: {}, Aktueller Index: {}, Zeichen: {}",
        mistakes, index, target_char
    );

    let info =
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Info"));

    let colored_text: Vec<Line> = colored_chars
        .chunks(frame.area().width as usize - 2) // -2 für die Ränder
        .map(|chunk| {
            Line::from(
                chunk
                    .iter()
                    .enumerate()
                    .map(|(i, (c, style))| {
                        let adjusted_index = i
                            + (chunk.as_ptr() as usize - colored_chars.as_ptr() as usize)
                                / std::mem::size_of::<(char, Style)>();
                        if adjusted_index == index {
                            Span::styled(
                                c.to_string(),
                                style.clone().bg(Color::Yellow).fg(Color::Black),
                            )
                        } else {
                            Span::styled(c.to_string(), *style)
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let target_text = Paragraph::new(colored_text)
        .block(Block::default().borders(Borders::ALL).title("Zieltext"))
        .wrap(ratatui::widgets::Wrap { trim: true });

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
}
