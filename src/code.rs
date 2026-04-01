use ratatui::{
    border,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Text,
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

pub struct Code {
    index: usize,
    code: [Option<char>; 4],
    correct: String,
}

impl Code {
    pub fn new(correct: String) -> Self {
        Self {
            index: 0,
            code: [None; 4],
            correct,
        }
    }

    pub fn push(&mut self, digit: char) {
        if self.index == 4 {
            return;
        }

        self.code[self.index] = Some(digit);
        self.index += 1;
    }

    pub fn pop(&mut self) {
        if self.index == 0 {
            return;
        }

        self.index -= 1;
        self.code[self.index] = None;
    }

    pub fn is_correct(&self) -> bool {
        let mut code_string = String::with_capacity(4);
        for i in 0..self.index {
            if let Some(digit) = self.code[i] {
                code_string.push(digit);
            } else {
                code_string.push('_');
            }
        }

        code_string == self.correct
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut code_string = String::with_capacity(4);
        for i in 0..self.index {
            if let Some(digit) = self.code[i] {
                code_string.push(digit);
            } else {
                code_string.push('_');
            }
        }

        let text = Text::from(code_string);
        let centered = area.centered(
            Constraint::Length(text.width() as u16),
            Constraint::Length(1),
        );

        text.render(centered, buf);
    }
}
