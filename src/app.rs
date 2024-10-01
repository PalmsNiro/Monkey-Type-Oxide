use ratatui::style::{Color, Style};

pub struct App {
    pub target_text: String,
    pub colored_chars: Vec<(char, Style)>,
    pub user_input: String,
    pub index: usize,
    pub mistakes: usize,
}

impl App {
    pub fn new(target_text: String) -> Self {
        let colored_chars = target_text
            .chars()
            .map(|c| (c, Style::default().fg(Color::DarkGray)))
            .collect();

        Self {
            target_text,
            colored_chars,
            user_input: String::new(),
            index: 0,
            mistakes: 0,
        }
    }

    pub fn type_char(&mut self, c: char) {
        if let Some(target_char) = self.target_text.chars().nth(self.index) {
            self.user_input.push(c);
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
}