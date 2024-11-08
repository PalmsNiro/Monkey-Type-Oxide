use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn create_main_layout(frame: &mut Frame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Tabs
            Constraint::Min(0),    // Content
        ])
        .split(frame.area())
        .to_vec()
}
