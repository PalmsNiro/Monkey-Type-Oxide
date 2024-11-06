use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use super::tabs::SelectedTab;

pub fn create_main_layout(frame: &mut Frame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tabs
            Constraint::Min(0),     // Content
        ])
        .split(frame.area()).to_vec()
}

pub fn draw_tabs(frame: &mut Frame, area: Rect, selected_tab: &SelectedTab) {
    // Implementierung der Tab-Leiste
}