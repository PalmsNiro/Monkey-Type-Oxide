#[derive(Clone)]
pub enum Language {
    De,
    En,
}

#[derive(Clone)]
pub enum TestType {
    RandomWords,
    Quotes,
}

#[derive(Clone)]
pub struct Options {
    pub ui_language: Language,
    pub test_language: Language,
    pub words_amount: i16,
    pub test_type: TestType,
}
impl Options {
    pub fn new() -> Self {
        Self {
            ui_language: Language::De,
            test_language: Language::De,
            words_amount: 15,
            test_type: TestType::RandomWords,
        }
    }

}
