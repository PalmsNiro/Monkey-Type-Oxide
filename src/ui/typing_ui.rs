use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::{app::AppState, app_options::AppOptions, type_test::TypingTest};

use super::{create_chart, create_colored_text, wrap_text};

pub fn draw_typing_tab(
    frame: &mut Frame,
    typing_test: &TypingTest,
    app_state: &AppState,
    options: &AppOptions,
) {
    match app_state {
        AppState::EndScreen => draw_end_screen(frame, typing_test),
        _ => draw_typing_screen(frame, typing_test, options),
    }
}

pub fn draw_typing_screen(frame: &mut Frame, typing_test: &TypingTest, options: &AppOptions) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),      // Spacing after tabs
            Constraint::Percentage(30), // Goaltext
            Constraint::Percentage(50), // chart
            Constraint::Length(3),      // Progress-Bar (text)
            Constraint::Length(3),      //Progress-Bar Time(if enabled)
        ])
        .split(frame.area());

    // Goal Text
    let available_width = chunks[1].width as usize - 4;
    let wrapped_text = wrap_text(&typing_test.target_text, available_width);
    let colored_text =
        create_colored_text(&wrapped_text, &typing_test.colored_chars, typing_test.index);
    let target_text = Paragraph::new(colored_text)
        .block(Block::default().borders(Borders::ALL).title("Goal text"));
    frame.render_widget(target_text, chunks[1]);

    // chart
    let wpm_points: Vec<(f64, f64)> = typing_test
        .test_data_history
        .iter()
        .map(|f| (f.timestamp as f64, f.wpm as f64))
        .collect();

    let wpm_raw_points: Vec<(f64, f64)> = typing_test
        .test_data_history
        .iter()
        .map(|f| (f.timestamp as f64, f.wpm_raw as f64))
        .collect();

    let chart = create_chart(&typing_test.test_data_history, &wpm_points, &wpm_raw_points);
    frame.render_widget(chart, chunks[2]);

    // Progress Bar (Text Progress)
    let progress = typing_test.progress();
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(progress);
    frame.render_widget(gauge, chunks[3]);

    // Time Race Bar (if activated)
    if options.time_race_enabled {
        const TIME_LIMIT: f64 = 30.0; // TODO: aus den Options holen
        let elapsed = typing_test.get_elapsed_time().as_secs_f64();
        let remaining_time = (TIME_LIMIT - elapsed).max(0.0);
        let time_progress = ((remaining_time / TIME_LIMIT) * 100.0) as u16;

        // Color based on time progress
        let color = match time_progress {
            0..=20 => Color::Red,
            21..=50 => Color::Yellow,
            _ => Color::Green,
        };

        let time_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Remaining Time"),
            )
            .gauge_style(Style::default().fg(color))
            .percent(time_progress)
            .label(format!("{:.1}s", remaining_time));

        frame.render_widget(time_gauge, chunks[4]);
    }
}

pub fn draw_end_screen(frame: &mut Frame, typing_test: &TypingTest) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // spacing after tabs
            Constraint::Length(8), // stats
            Constraint::Min(0),    //chart
        ])
        .split(frame.area());

    let stats_text = if typing_test.text_finished {
        create_test_stats_text(typing_test)
    } else {
        vec![
            Line::from(vec![Span::styled(
                "Test failed, try again!",
                Style::default().fg(Color::LightRed),
            )]),
            Line::from("Press 'r' to restart the test"),
        ]
    };

    frame.render_widget(
        Paragraph::new(stats_text)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Test stats")),
        chunks[1],
    );

    //chart
    let wpm_points: Vec<(f64, f64)> = typing_test
        .test_data_history
        .iter()
        .map(|f| (f.timestamp as f64, f.wpm as f64))
        .collect();

    let wpm_raw_points: Vec<(f64, f64)> = typing_test
        .test_data_history
        .iter()
        .map(|f| (f.timestamp as f64, f.wpm_raw as f64))
        .collect();

    let chart = create_chart(&typing_test.test_data_history, &wpm_points, &wpm_raw_points);
    frame.render_widget(chart, chunks[2]);
}

fn create_test_stats_text(typing_test: &TypingTest) -> Vec<Line<'_>> {
    //accuracy text
    let accuracy_text = format!("Accuracy: {:.2}", typing_test.accuracy());
    //time text
    let elapsed = typing_test.get_elapsed_time();
    let seconds = elapsed.as_secs() % 60;
    let minutes = (elapsed.as_secs() / 60) % 60;
    let time_text = format!("You needed {}:{:02} minutes", minutes, seconds,);
    //wpm text
    let wpm_text = format!(
        "Wpm: {:.1} \n Wpm raw: {:.1}",
        typing_test.get_wpm(),
        typing_test.get_wpm_raw()
    );
    //errors text
    let error_text = format!(
        "Mistakes: {} out of {} total characters",
        typing_test.mistakes,
        typing_test.target_text.len()
    );

    let stats_text = vec![
        Line::from(accuracy_text),
        Line::from(time_text),
        Line::from(wpm_text),
        Line::from(error_text),
    ];
    stats_text
}
