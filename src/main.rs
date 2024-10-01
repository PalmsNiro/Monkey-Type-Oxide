use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::io;

use rand::seq::SliceRandom;
use rand::thread_rng;
use random_word::Lang;

use textwrap::{wrap, Options, WordSplitter};

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
    // let mut width = get_terminal_width() - 10;
    let random_sentence = get_random_sentence(30);
    let string_to_type = random_sentence.clone(); // Keine Umbrüche hier

    let mut colored_chars: Vec<(char, Style)> = string_to_type
        .chars()
        .map(|c| (c, Style::default().fg(Color::DarkGray)))
        .collect();

    let mut user_input = String::new();
    let mut index = 0;
    let mut mistakes = 0;

    loop {
        // width = get_terminal_width() - 10;
        terminal.draw(|f| draw_ui(f, &string_to_type, &colored_chars, index, mistakes))?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                    if let Some(target_char) = string_to_type.chars().nth(index) {
                        user_input.push(c);
                        if c == target_char {
                            if let Some((_, style)) = colored_chars.get_mut(index) {
                                *style = Style::default().fg(Color::Green);
                            }
                        } else {
                            if let Some((_, style)) = colored_chars.get_mut(index) {
                                *style = Style::default().fg(Color::Red);
                            }
                            mistakes += 1;
                        }
                        index += 1;
                    }
                }
                KeyCode::Backspace if key.kind == KeyEventKind::Press => {
                    if !user_input.is_empty() {
                        user_input.pop();
                        if index > 0 {
                            index -= 1;
                            if let Some((_, style)) = colored_chars.get_mut(index) {
                                *style = Style::default().fg(Color::DarkGray);
                            }
                        }
                    }
                }
                KeyCode::Esc if key.kind == KeyEventKind::Press => {
                    return Ok(());
                }
                _ => {}
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

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let options = Options::new(width)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(WordSplitter::NoHyphenation)
        .break_words(false);
    wrap(text, options).into_iter().map(|s| s.into_owned()).collect()
}

// fn get_terminal_width() -> usize {
//     terminal_size()
//         .map(|(Width(w), _)| w as usize)
//         .unwrap_or(80)
// }

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

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Info"));

    let available_width = frame.area().width as usize - 4; // -4 für die Ränder und etwas Puffer
    let wrapped_text = wrap_text(string_to_type, available_width);

    let mut colored_text: Vec<Line> = Vec::new();
    let mut current_index = 0;
    let mut _cursor_line = 0;
    let mut _cursor_column = 0;

    for (line_index, line) in wrapped_text.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::new();
        let mut line_length = 0;

        for c in line.chars() {
            let style = if current_index == index {
                _cursor_line = line_index;
                _cursor_column = line_length;
                colored_chars[current_index].1.clone().bg(Color::Yellow).fg(Color::Black)
            } else {
                colored_chars[current_index].1
            };
            spans.push(Span::styled(c.to_string(), style));
            current_index += 1;
            line_length += 1;
        }

        // Füge ein Leerzeichen am Ende jeder Zeile hinzu, außer bei der letzten
        if line_index < wrapped_text.len() - 1 {
            let space_style = if current_index == index {
                _cursor_line = line_index;
                _cursor_column = line_length;
                Style::default().bg(Color::Yellow).fg(Color::Black)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            spans.push(Span::styled(" ", space_style));
            current_index += 1;
        }

        colored_text.push(Line::from(spans));
    }

    let target_text = Paragraph::new(colored_text)
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
}