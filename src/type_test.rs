use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::style::{Color, Style};

use rand::seq::SliceRandom;
use rand::thread_rng;
use random_word::Lang;

use crate::options::{Language,TestType};
// #[derive(Clone)]
// pub enum Language {
//     De,
//     En,
// }
// pub enum TestType {
//     RandomWords,
//     Quotes,
// }

const WORDS_AMOUNT: u16 = 15;

pub struct TypingTest {
    language: Language,
    test_type: TestType,
    pub target_text: String,
    pub colored_chars: Vec<(char, Style)>,
    pub user_input: String,
    pub index: usize,
    pub mistakes: usize,
    pub total_chars: usize,
    pub text_finished: bool,
}
impl TypingTest {
    pub fn new(lan: Language, test_type: TestType) -> Self {
        let text = Self::get_random_sentence(WORDS_AMOUNT as usize, &lan); // Übergebe die Sprache als Parameter

        let colored_chars = text
            .chars()
            .map(|c| (c, Style::default().fg(Color::DarkGray)))
            .collect();

        Self {
            language: lan,
            test_type: test_type,
            target_text: text,
            colored_chars,
            user_input: String::new(),
            index: 0,
            mistakes: 0,
            total_chars: 0,
            text_finished: false,
        }
    }

    // Statische Methode ohne &self
    pub fn get_random_sentence(words_amount: usize, language: &Language) -> String {
        let lan = match language {
            Language::De => Lang::De,
            Language::En => Lang::En,
        };

        let word_list: &[&str] = random_word::all(lan);
        let mut rng = thread_rng();

        let selected_words: Vec<&str> = word_list
            .choose_multiple(&mut rng, words_amount)
            .cloned()
            .collect();

        let sentence = selected_words.join(" ");
        let capitalized =
            sentence.chars().nth(0).unwrap().to_uppercase().to_string() + &sentence[1..];

        capitalized
            .replace("ä", "ae")
            .replace("ö", "oe")
            .replace("ü", "ue")
            .replace("Ä", "Ae")
            .replace("Ö", "Oe")
            .replace("Ü", "Ue")
            .replace("ß", "ss")
    }

    pub fn handle_key_event(&mut self) -> Result<(), io::Error> {
        Ok(if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) if key.kind == KeyEventKind::Press => {
                    self.type_char(c);
                }
                KeyCode::Backspace if key.kind == KeyEventKind::Press => {
                    self.backspace();
                }
                KeyCode::Esc if key.kind == KeyEventKind::Press => {
                    return Ok(());
                }
                _ => {}
            }
        })
    }

    fn type_char(&mut self, c: char) {
        if let Some(target_char) = self.target_text.chars().nth(self.index) {
            self.user_input.push(c);
            self.total_chars += 1; // Inkrementierung der Gesamtanzahl der getippten Zeichen
            if c == target_char {
                if let Some((_, style)) = self.colored_chars.get_mut(self.index) {
                    *style = Style::default().fg(Color::Green);
                }
            } else {
                if let Some((_, style)) = self.colored_chars.get_mut(self.index) {
                    *style = Style::default().fg(Color::Red);
                }
                self.mistakes += 1;
            }
            self.index += 1;
        }
    }

    fn backspace(&mut self) {
        if !self.user_input.is_empty() {
            self.user_input.pop();
            if self.index > 0 {
                self.index -= 1;
                if let Some((_, style)) = self.colored_chars.get_mut(self.index) {
                    *style = Style::default().fg(Color::DarkGray);
                }
            }
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_chars == 0 {
            100.0
        } else {
            ((self.total_chars - self.mistakes) as f64 / self.total_chars as f64) * 100.0
        }
    }

    pub fn progress(&self) -> u16 {
        if self.target_text.is_empty() {
            0
        } else {
            ((self.index as f64 / self.target_text.len() as f64) * 100.0) as u16
        }
    }

    pub fn reset(&mut self) {
        let new_test = TypingTest::new(self.language.clone(), self.test_type.clone());
        *self = new_test;
    }
}
