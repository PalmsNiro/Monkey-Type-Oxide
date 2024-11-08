use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Tabs},
    Frame,
};
use strum::IntoEnumIterator;

use super::tabs::SelectedTab;

pub fn create_main_layout(frame: &mut Frame) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
        ])
        .split(frame.area())
        .to_vec()
}

pub fn draw_tabs(frame: &mut Frame, area: Rect, selected_tab: &SelectedTab) {
    let titles = SelectedTab::iter()
        .map(|tab| {
            let (name, color) = match tab {
                SelectedTab::Tab1 => ("Typing Test", Color::Blue),
                SelectedTab::Tab2 => ("Options", Color::Green),
                SelectedTab::Tab3 => ("Account", Color::Magenta),
                SelectedTab::Tab4 => ("About", Color::Red),
            };
            format!("  {}  ", name).fg(Color::White).bg(color)
        })
        .collect::<Vec<_>>();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .highlight_style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .select((*selected_tab) as usize)
        .divider("|");

    frame.render_widget(tabs, area);
}
