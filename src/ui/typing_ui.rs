mod chart;
mod common_ui;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{Block, Borders, Gauge, Paragraph}, Frame};

use crate::{app::AppState, type_test::TypingTest};

pub fn draw_typing_tab(frame: &mut Frame, typing_test: &TypingTest, app_state: &AppState) {
    match app_state {
        AppState::EndScreen => draw_end_screen(frame, typing_test),
        _ => draw_typing_screen(frame, typing_test),
    }
}

pub fn draw_typing_screen(frame: &mut Frame, typing_test: &TypingTest) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Info-Bar
            Constraint::Percentage(30), // Goaltext
            Constraint::Percentage(50), // Space for future use (chart)
            Constraint::Length(3),      // Progress-Bar
        ])
        .split(frame.area());

    //Infotext
    let info_text = format!(
        "Mistakes: {}, Current Index: {}, Char: {}, Accuracy: {:.2}%, test_data size: {}",
        typing_test.mistakes,
        typing_test.index,
        typing_test
            .target_text
            .chars()
            .nth(typing_test.index)
            .unwrap_or(' '),
        typing_test.accuracy(),
        typing_test.test_data_history.len(),
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

    // chart
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
    frame.render_widget(chart, chunks[2]);

    // Progress Bar
    let progress = typing_test.progress();
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Fortschritt"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(progress);

    frame.render_widget(gauge, chunks[3]);
}

pub fn draw_end_screen(frame: &mut Frame, typing_test: &TypingTest) {
    // Ihre bestehende draw_end_screen Implementierung
}
