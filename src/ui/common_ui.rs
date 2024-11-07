use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use textwrap::{wrap, Options, WordSplitter};

pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let options = Options::new(width)
        .word_separator(textwrap::WordSeparator::AsciiSpace)
        .word_splitter(WordSplitter::NoHyphenation)
        .break_words(false);
    wrap(text, options)
        .into_iter()
        .map(|s| s.into_owned())
        .collect()
}

pub  fn create_colored_text<'a>(
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
