mod typing_ui;
// mod options;
// mod account;
// mod about;
mod chart;
mod common_ui;
mod layout_ui;
pub mod tabs;

// Re-export wichtiger Komponenten
// pub use self::layout_ui::create_main_layout;
// pub use self::typing_ui::draw_typing_tab;

use crate::app::AppState;
use crate::type_test::TypingTest;
use ratatui::Frame;
use tabs::SelectedTab;

pub use common_ui::{wrap_text, create_colored_text};
pub use typing_ui::draw_typing_screen;
pub use chart::create_chart;

pub fn draw_ui(frame: &mut Frame, app_state: &AppState, selected_tab: &SelectedTab, typing_test: &TypingTest) {
    // Erstelle das Haupt-Layout mit Tabs
    let main_layout = layout_ui::create_main_layout(frame);

    // Rendere die Tab-Leiste
    layout_ui::draw_tabs(frame, main_layout[0], selected_tab);

    // Rendere den aktiven Tab-Inhalt
    match selected_tab {
        // SelectedTab::Tab1 => todo!(),
        // SelectedTab::Tab2 => todo!(),
        // SelectedTab::Tab3 => todo!(),
        // SelectedTab::Tab4 => todo!(),
        // SelectedTab::Tab1 => typing_ui::draw_typing_tab(frame, main_layout[1], app_state),
        SelectedTab::Tab1 => typing_ui::draw_typing_tab(frame, typing_test, app_state),
        // SelectedTab::Tab2 => options_ui::draw_options_tab(frame, main_layout[1]),
        // SelectedTab::Tab3 => account::draw_account_tab(frame, main_layout[1]),
        // SelectedTab::Tab4 => about::draw_about_tab(frame, main_layout[1]),
        _ => {}
    }
}
