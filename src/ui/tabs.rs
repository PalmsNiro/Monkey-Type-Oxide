use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use ratatui::{
    layout::Rect, style::{Color, Style, Stylize}, text::{Line, Span}, widgets::{Block, Borders, Paragraph, Tabs}, Frame
};

use crate::{app::OptionsState, app_options::AppOptions};

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

pub fn draw_options(frame: &mut Frame<'_>, main_layout: &Vec<Rect>, options: &AppOptions, options_state: &OptionsState) {
    let options_content = vec![
        (format!("Test Language: {}", options.test_language.to_string()), 0),
        (format!("Test Type: {}", options.test_type.to_string()), 1),
        (format!("Timed Race: {}", options.time_race_enabled.then_some("enabled").unwrap_or("disabled")), 2),
        (format!("Hardcore: {}", options.hardcore_enabled.then_some("enabled").unwrap_or("disabled")), 3),
        (format!("(WIP) UI Language: {}", options.ui_language.to_string()), 4),
    ];

    let options_text: Vec<Line> = options_content
        .into_iter()
        .map(|(text, index)| {
            if index == options_state.selected_option {
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Yellow)),
                    Span::styled(text, Style::default().fg(Color::Yellow))
                ])
            } else {
                Line::from(text)
            }
        })
        .collect();

    frame.render_widget(
        Paragraph::new(options_text)
            .block(Block::default().borders(Borders::ALL).title("Options")),
        main_layout[1],
    );
}