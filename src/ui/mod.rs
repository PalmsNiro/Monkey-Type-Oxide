mod typing_ui;
// mod options;
// mod account;
// mod about;
mod chart;
mod common_ui;
mod layout_ui;
pub mod tabs;

use ratatui::{style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, Paragraph}, Frame};
use crate::{app::{AppState, OptionsState}, app_options::AppOptions};
use crate::type_test::TypingTest;

pub use chart::create_chart;
pub use common_ui::{create_colored_text, wrap_text};
use tabs::{SelectedTab,draw_tabs,draw_options};
use layout_ui::create_main_layout;


pub fn draw_ui(frame: &mut Frame, app_state: &AppState, selected_tab: &SelectedTab, typing_test: &TypingTest,options: &AppOptions, options_state:&OptionsState) {
    let main_layout = create_main_layout(frame);
    
    draw_tabs(frame, main_layout[0], selected_tab);
    
    match selected_tab {
        SelectedTab::Tab1 => typing_ui::draw_typing_tab(frame, typing_test, app_state, options),
        SelectedTab::Tab2 => draw_options(frame, &main_layout, options,options_state),
        SelectedTab::Tab3 => {
            // Account Tab
            let account_text = vec![
                Line::from("Account"),
                Line::from("Coming Soon..."),
            ];
            frame.render_widget(
                Paragraph::new(account_text)
                    .block(Block::default().borders(Borders::ALL).title("Account")),
                main_layout[1],
            );
        },
        SelectedTab::Tab4 => {
            // About Tab
            let about_text = vec![
                Line::from(vec![
                    Span::styled("MonkeyTypeOxide", Style::default().fg(Color::Green))
                ]),
                Line::from("Version 0.1.0"),
                Line::from(""),
                Line::from("A Rust-based typing test application"),
            ];
            frame.render_widget(
                Paragraph::new(about_text)
                    .block(Block::default().borders(Borders::ALL).title("About")),
                main_layout[1],
            );
        },
    }
}