use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, Paragraph},
    Frame,
};
use textwrap::{wrap, Options, WordSplitter};

use crate::{
    app::AppState,
    type_test::{TestDataPerSecond, TypingTest},
};

pub fn draw_typing_screen(frame: &mut Frame, typing_test: &TypingTest) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Info-Bar
            Constraint::Percentage(40), // Goaltext
            Constraint::Percentage(40), // Space for future use
            Constraint::Length(3),      // Progress-Bar
        ])
        .split(frame.area());

    //Infotext
    let info_text = format!(
        "Fehler: {}, Aktueller Index: {}, Zeichen: {}, Genauigkeit: {:.2}%, ",
        typing_test.mistakes,
        typing_test.index,
        typing_test
            .target_text
            .chars()
            .nth(typing_test.index)
            .unwrap_or(' '),
        typing_test.accuracy(),
    );

    let info =
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Info"));

    frame.render_widget(info, chunks[0]);

    // Goal Text
    let available_width = chunks[1].width as usize - 4;
    let wrapped_text = wrap_text(&typing_test.target_text, available_width);
    let colored_text =
        create_colored_text(&wrapped_text, &typing_test.colored_chars, typing_test.index);
    let target_text = Paragraph::new(colored_text)
        .block(Block::default().borders(Borders::ALL).title("Zieltext"));
    frame.render_widget(target_text, chunks[1]);

    // Progress Bar
    let progress = typing_test.progress();
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Fortschritt"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(progress);

    frame.render_widget(gauge, chunks[3]);
}

pub fn draw_end_screen(frame: &mut Frame, typing_test: &TypingTest) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(50),
        ])
        .split(frame.area());

    //accuracy
    let accuracy_text = format!("Accuracy: {:.2}", typing_test.accuracy());
    let accuracy_info = Paragraph::new(accuracy_text).alignment(Alignment::Center);
    frame.render_widget(accuracy_info, chunks[0]);

    //time
    let elapsed = typing_test.get_elapsed_time();
    let seconds = elapsed.as_secs() % 60;
    let minutes = (elapsed.as_secs() / 60) % 60;
    let time_text = format!("You needed {}:{:02} minutes", minutes, seconds,);
    let time_info = Paragraph::new(time_text).alignment(Alignment::Center);
    frame.render_widget(time_info, chunks[1]);

    //wpm
    let wpm_text = format!(
        "Wpm: {:.1} \n Wpm raw: {:.1}",
        typing_test.get_wpm(),
        typing_test.get_wpm_raw()
    );
    let wpm_info = Paragraph::new(wpm_text).alignment(Alignment::Center);
    frame.render_widget(wpm_info, chunks[2]);

    //errors
    let error_text = format!(
        "Mistakes: {} out of {} total characters",
        typing_test.mistakes,
        typing_test.target_text.len()
    );
    let error_info = Paragraph::new(error_text).alignment(Alignment::Center);
    frame.render_widget(error_info, chunks[3]);

    //chart
    let wpm_points: Vec<(f64, f64)> = typing_test
        .test_data_history
        .iter()
        .map(|f| (f.timestamp as f64, f.wpm))
        .collect();

    let wpm_raw_points: Vec<(f64, f64)> = typing_test
        .test_data_history
        .iter()
        .map(|f| (f.timestamp as f64, f.wpm_raw))
        .collect();

    let chart = create_chart(&typing_test.test_data_history, &wpm_points, &wpm_raw_points);
    frame.render_widget(chart, chunks[4]);
}

pub fn draw_ui(frame: &mut Frame, typing_test: &TypingTest, state: &AppState) {
    match state {
        AppState::EndScreen => draw_end_screen(frame, typing_test),
        _ => draw_typing_screen(frame, typing_test),
    }
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

fn create_chart<'a>(
    test_data_history: &'a [TestDataPerSecond],
    wpm_points: &'a [(f64, f64)],
    wpm_raw_points: &'a [(f64, f64)],
) -> Chart<'a> {
    let wpm_dataset = Dataset::default()
        .name("WPM")
        .marker(symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(wpm_points);

    let wpm_raw_dataset = Dataset::default()
        .name("Raw WPM")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(wpm_raw_points);

    Chart::new(vec![wpm_dataset, wpm_raw_dataset])
        .block(Block::bordered().title("Typing Speed"))
        .x_axis(
            Axis::default()
                .title("Time (s)")
                .bounds([0.0, test_data_history.len() as f64])
                .labels(vec![
                    Span::from("0"),
                    Span::from(format!("{}", test_data_history.len() / 2)),
                    Span::from(format!("{}", test_data_history.len())),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Words Per Minute")
                .bounds([0.0, 100.0])
                .labels(vec![Span::from("0"), Span::from("50"), Span::from("100")]),
        )
}
