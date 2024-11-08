use std::default;

use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum Language {
    #[default]
    #[strum(to_string = "English")]
    En,
    #[strum(to_string = "German")]
    De,
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

#[derive(Clone)]
pub struct AppOptions {
    pub ui_language: Language,
    pub test_language: Language,
    pub test_type: TestType,
    pub time_race: bool,
    pub hardcore: bool,
}
impl AppOptions {
    pub fn new() -> Self {
        Self {
            test_language: Language::En,
            test_type: TestType::RandomWords1K,
            time_race: false,
            hardcore: false,
            ui_language: Language::De,
        }
    }
}
