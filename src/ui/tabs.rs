use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use ratatui::{
    layout::Rect, style::{Color, Style, Stylize}, text::Line, widgets::{Block, Borders, Paragraph, Tabs}, Frame
};

use crate::app_options::AppOptions;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Tab 1")] // start screen / test area / end screen
    Tab1,
    #[strum(to_string = "Tab 2")] // options
    Tab2,
    #[strum(to_string = "Tab 3")] // account
    Tab3,
    #[strum(to_string = "Tab 4")] // about
    Tab4,
}
impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
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
            format!("  {}  ", name).fg(Color::White)
        })
        .collect::<Vec<_>>();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .highlight_style(Style::default().fg(Color::LightCyan))
        .select((*selected_tab) as usize)
        .divider("|");

    frame.render_widget(tabs, area);
}

pub fn draw_options(frame: &mut Frame<'_>, main_layout: &Vec<ratatui::prelude::Rect>, options: &AppOptions) {
    let test_language = format!("Test Langugage: {}", options.test_language.to_string());
    let test_type  = format!("Test Type: {}", options.test_type.to_string());
    let ui_language =format!("Ui Language (Not Yet Supported)") ;

    let options_text = vec![
        Line::from(test_language),
        Line::from(test_type),
        Line::from(ui_language),
    ];
    frame.render_widget(
        Paragraph::new(options_text)
            .block(Block::default().borders(Borders::ALL).title("Options")),
        main_layout[1],
    );
}