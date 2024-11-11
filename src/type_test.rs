use log::error;
use ratatui::style::{Color, Style};
use std::time::{Duration, Instant};

use crate::text_gen::get_sentence;

use crate::app_options::{Language, TestType};

#[derive(Clone)]
pub struct TestDataPerSecond {
    pub mistakes: usize,
    pub wpm: i64,
    pub wpm_raw: i64,
    pub timestamp: u64, // Second of measurement
}

pub struct TypingTest {
    language: Language,
    test_type: TestType,
    pub target_text: String,
    pub colored_chars: Vec<(char, Style)>,
    pub user_input: String,
    pub index: usize,
    pub mistakes: usize,
    pub total_chars_tipped: usize,
    // pub total_words: usize,
    pub text_finished: bool,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub correct_words_chars: i32,
    last_word_start: usize,
    pub test_data_history: Vec<TestDataPerSecond>,
    mistakes_in_current_second: usize,
}
impl TypingTest {
    pub fn new(lan: Language, test_type: TestType) -> Self {
        let text = get_sentence(lan.clone(), test_type.clone());
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
            total_chars_tipped: 0,
            // total_words: words_amount,
            text_finished: false,
            start_time: None,
            end_time: None,
            correct_words_chars: 0,
            last_word_start: 0,
            test_data_history: Vec::new(),
            mistakes_in_current_second: 0,
        }
    }

    pub fn type_char(&mut self, c: char) {
        if let Some(target_char) = self.target_text.chars().nth(self.index) {
            if self.index == 0 {
                self.start_timer();
            }

            self.user_input.push(c);
            self.total_chars_tipped += 1;

            let is_current_char_correct = c == target_char;

            if let Some((_, style)) = self.colored_chars.get_mut(self.index) {
                *style = if is_current_char_correct {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                };
            }

            if !is_current_char_correct {
                self.mistakes += 1;
                self.mistakes_in_current_second += 1;
            }

            self.check_for_correct_word(target_char, is_current_char_correct);
        }
    }

    fn check_for_correct_word(&mut self, target_char: char, is_current_char_correct: bool) {
        // Check if end of word reached (whitespace or end of text)
        let is_word_end = target_char == ' ' || self.index == self.target_text.len() - 1;

        if is_word_end {
            // Sicherheitscheck für Indizes
            if self.last_word_start > self.index {
                error!("Warning: Invalid word boundaries. Resetting word start.");
                self.last_word_start = self.index;
            }

            // Safe extraction
            let target_word =
                match self.safe_extract_word(&self.target_text, self.last_word_start, self.index) {
                    Ok(word) => word,
                    Err(e) => {
                        error!("Warning: Could not extract target word: {}", e);
                        String::new()
                    }
                };

            let user_word =
                match self.safe_extract_word(&self.user_input, self.last_word_start, self.index) {
                    Ok(word) => word,
                    Err(e) => {
                        error!("Warning: Could not extract user word: {}", e);
                        String::new()
                    }
                };

            // Compare Words
            let word_correct = target_word == user_word;

            if word_correct {
                self.correct_words_chars += target_word.chars().count() as i32;
            }

            self.last_word_start = self.index + 1;
        }

        self.index += 1;
    }

    fn safe_extract_word(&self, text: &str, start: usize, end: usize) -> Result<String, String> {
        if start > end {
            return Err("Start index greater than end index".to_string());
        }
        if start >= text.len() {
            return Err("Start index out of bounds".to_string());
        }

        // Find safe Char-Border
        let safe_start = text
            .char_indices()
            .find(|(i, _)| *i >= start)
            .map(|(i, _)| i)
            .unwrap_or(start);

        let safe_end = std::cmp::min(
            text.char_indices()
                .find(|(i, _)| *i > end)
                .map(|(i, _)| i)
                .unwrap_or(text.len()),
            text.len(),
        );

        // Safe Extraction of string slice
        match text.get(safe_start..safe_end) {
            Some(slice) => Ok(slice.to_string()),
            None => Err("Could not extract word slice".to_string()),
        }
    }

    pub fn backspace(&mut self) {
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
        if self.total_chars_tipped == 0 {
            100.0
        } else {
            ((self.total_chars_tipped - self.mistakes) as f64 / self.total_chars_tipped as f64)
                * 100.0
        }
    }

    pub fn progress(&self) -> u16 {
        if self.target_text.is_empty() {
            0
        } else {
            ((self.index as f64 / self.target_text.len() as f64) * 100.0) as u16
        }
    }

    pub fn reset(&mut self, lan: Language, test_type: TestType) {
        let new_test = TypingTest::new(lan, test_type);
        *self = new_test;
    }

    pub fn start_timer(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn stop_timer(&mut self) {
        if self.end_time.is_none() {
            self.end_time = Some(Instant::now());
        }
    }

    pub fn get_elapsed_time(&self) -> Duration {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => end - start,
            (Some(start), None) => Instant::now() - start,
            _ => Duration::from_secs(0),
        }
    }

    // total number of characters in the correctly typed words (including spaces), divided by 5 and normalised to 60 seconds.
    pub fn get_wpm(&self) -> i64 {
        let elapsed_seconds = self.get_elapsed_time().as_secs_f64() as i32;
        if elapsed_seconds == 0 {
            return 0;
        }

        let words = self.correct_words_chars  / 5;
        ((words * 60) / elapsed_seconds).try_into().unwrap() 
    }

    // calculated just like wpm, but also includes incorrect words.
    pub fn get_wpm_raw(&self) -> i64 {
        let elapsed_seconds = self.get_elapsed_time().as_secs_f64() as usize;
        if elapsed_seconds == 0 {
            return 0;
        }

        // Raw WPM = (alle getippten Zeichen / 5) * (60 / Zeit)
        let raw_words = self.total_chars_tipped  / 5;
        ((raw_words * 60) / elapsed_seconds).try_into().unwrap()
    }

    pub fn update_test_data(&mut self) {
        if self.start_time.is_some() {
            // Only update if test runnning
            let current_second = self.get_elapsed_time().as_secs() as u64;

            let last_recorded = self
                .test_data_history
                .last()
                .map(|data| data.timestamp)
                .unwrap_or(0);

            // Check if entry for this second already exists
            if self.test_data_history.is_empty() {
                let initial_data = TestDataPerSecond {
                    timestamp: 0,
                    wpm: self.get_wpm(),
                    wpm_raw: self.get_wpm_raw(),
                    mistakes: self.mistakes,
                };
                self.test_data_history.push(initial_data);
            }

            // Fill all missing seconds
            for sec in (last_recorded + 1)..=current_second {
                let fill_data = TestDataPerSecond {
                    timestamp: sec,
                    wpm: self.get_wpm(),
                    wpm_raw: self.get_wpm_raw(),
                    mistakes: self.mistakes,
                };
                self.test_data_history.push(fill_data);
            }
        }
    }

    pub fn get_test_data_for_second(&self, second: u64) -> Option<&TestDataPerSecond> {
        self.test_data_history
            .iter()
            .find(|metrics| metrics.timestamp == second)
    }

    pub fn get_all_test_data(&self) -> &[TestDataPerSecond] {
        &self.test_data_history
    }
}
