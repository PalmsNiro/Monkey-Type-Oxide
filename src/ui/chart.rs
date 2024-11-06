use ratatui::{style::{Color, Style}, symbols, text::Span, widgets::{Axis, Block, Chart, Dataset, GraphType}};

use crate::type_test::TestDataPerSecond;



fn create_chart<'a>(
    test_data_history: &'a [TestDataPerSecond],
    wpm_points: &'a [(f64, f64)],
    wpm_raw_points: &'a [(f64, f64)],
) -> Chart<'a> {
    //get the maximum out of the y values
    // Iterator for y-Values
    let max_y_points = wpm_points
        .iter()
        .map(|&(_, y)| y)
        .fold(f64::NEG_INFINITY, f64::max);

    let max_y_raw_points = wpm_raw_points
        .iter()
        .map(|&(_, y)| y)
        .fold(f64::NEG_INFINITY, f64::max);

    // compare maxima
    let max_y = max_y_points.max(max_y_raw_points);

    // Find exact maximum time
    let max_time = test_data_history
        .iter()
        .map(|data| data.timestamp)
        .max()
        .unwrap_or(0);

    let wpm_dataset = Dataset::default()
        .name("WPM")
        .marker(symbols::Marker::Braille)
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
        .block(Block::bordered().title("WPM"))
        .x_axis(
            Axis::default()
                .title("Time (s)")
                .bounds([0.0, max_time as f64])
                .labels(vec![
                    Span::from("0"),
                    Span::from(format!("{}", max_time / 2)),
                    Span::from(format!("{}", max_time)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Words Per Minute")
                .bounds([0.0, max_y + 10.0])
                .labels(vec![
                    Span::from("0"),
                    Span::from(format!("{}", ((max_y + 10.0) / 2.0) as u32)),
                    Span::from(format!("{}", (max_y + 5.0) as u32)),
                ]),
        )
}