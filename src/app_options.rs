use std::default;

use strum::{Display, EnumIter, FromRepr};

/*
    !!!
    TODO: Check how we can reduce the repeated writing of the next/previous functions for iterating over enums
*/

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum Language {
    #[default]
    #[strum(to_string = "English")]
    En,
    #[strum(to_string = "German")]
    De,
}
impl Language{
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

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum TestType {
    #[strum(to_string = "Random Words (Chaos)")]
    RandomWords, // full on random words, there willl be stuff you never heard about
    #[default]
    #[strum(to_string = "Random Words (Top 1.000)")]
    RandomWords1K, // word selection out of the top 1k words for a language
    #[strum(to_string = "Random Words (Top 10.000)")]
    RandomWords10K, // word selection out of the top 10k words for a language
    #[strum(to_string = "Quotes")]
    Quotes, // random qutos
    #[strum(to_string = "Jokes")]
    Jokes, // silly jokes
}
impl TestType{
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

#[derive(Clone)]
pub struct AppOptions {
    pub ui_language: Language,
    pub test_language: Language,
    pub test_type: TestType,
    pub time_race_enabled: bool,
    pub hardcore_enabled: bool,
}
impl AppOptions {
    pub fn new() -> Self {
        Self {
            test_language: Language::En,
            test_type: TestType::RandomWords1K,
            time_race_enabled: false,
            hardcore_enabled: false,
            ui_language: Language::En,
        }
    }
}
