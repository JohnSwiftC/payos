use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::Text,
    widgets::Widget,
};

pub struct Code {
    index: usize,
    code: [Option<char>; 4],
}

impl Code {
    pub fn new() -> Self {
        Self {
            index: 0,
            code: [None; 4],
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

    pub fn get_code(&self) -> Option<String> {
        if self.code[0] == None {
            return None;
        }

        let mut code_string = String::with_capacity(4);
        for i in 0..self.index {
            if let Some(digit) = self.code[i] {
                code_string.push(digit);
            } else {
                code_string.push('_');
            }
        }

        Some(code_string)
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut code_string = String::with_capacity(4);
        for i in 0..4 {
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
