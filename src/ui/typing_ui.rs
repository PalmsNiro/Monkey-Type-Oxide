
use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{Block, Borders, Gauge, Paragraph}, Frame};

use crate::{app::AppState, type_test::TypingTest};

use super::{create_chart, create_colored_text, wrap_text};

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