use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use textwrap::{wrap, Options, WordSplitter};

use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(frame.area());

    let info_text = format!(
        "Fehler: {}, Aktueller Index: {}, Zeichen: {}, Accuracy: {}%",
        app.mistakes,
        app.index,
        app.target_text.chars().nth(app.index).unwrap_or(' '),
        format!("{:.2}", app.accuracy())
    );

    let info =
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Info"));

    frame.render_widget(info, chunks[0]);

    let available_width = frame.area().width as usize - 4;
    let wrapped_text = wrap_text(&app.target_text, available_width);
    let colored_text = create_colored_text(&wrapped_text, &app.colored_chars, app.index);

    let target_text = Paragraph::new(colored_text)
        .block(Block::default().borders(Borders::ALL).title("Zieltext"));

    frame.render_widget(target_text, chunks[1]);
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let options = Options::new(width)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(WordSplitter::NoHyphenation)
        .break_words(false);
    wrap(text, options)
        .into_iter()
        .map(|s| s.into_owned())
        .collect()
}

fn create_colored_text<'a>(
    wrapped_text: &'a [String],
    colored_chars: &'a [(char, Style)],
    current_index: usize,
) -> Vec<Line<'a>> {
    let mut colored_text: Vec<Line> = Vec::new();
    let mut char_index = 0;

    for (line_index, line) in wrapped_text.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::new();

        for c in line.chars() {
            let style = if char_index == current_index {
                colored_chars[char_index]
                    .1
                    .clone()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
            } else {
                colored_chars[char_index].1
            };
            spans.push(Span::styled(c.to_string(), style));
            char_index += 1;
        }

        if line_index < wrapped_text.len() - 1 {
            let space_style = if char_index == current_index {
                Style::default().bg(Color::Yellow).fg(Color::Black)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            spans.push(Span::styled(" ", space_style));
            char_index += 1;
        }

        colored_text.push(Line::from(spans));
    }

    colored_text
}