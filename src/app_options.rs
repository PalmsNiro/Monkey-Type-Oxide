#[derive(Clone)]
pub enum Language {
    De,
    En,
}

#[derive(Clone)]
pub enum TestType {
    RandomWords, // full on random words, there willl be stuff you never heard about
    RandomWords1K, // word selection out of the top 1k words for a language
    RandomWords10K, // word selection out of the top 10k words for a language
    Quotes, // random qutos
    TimeRace, // race against the time, 30 sec
    Jokes, // silly jokes
    Hardcore, // No mistakes allowed
}

#[derive(Clone)]
pub struct AppOptions {
    pub ui_language: Language,
    pub test_language: Language,
    pub test_type: TestType,
}
impl AppOptions {
    pub fn new() -> Self {
        Self {
            ui_language: Language::De,
            test_language: Language::En,
            test_type: TestType::RandomWords1K,
        }
    }
}
