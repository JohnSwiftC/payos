use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::Widget,
};

use crate::style;

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
        let layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1), // header
            Constraint::Length(1), // gap
            Constraint::Length(1), // pin row
            Constraint::Length(1), // gap
            Constraint::Length(1), // hint
            Constraint::Min(0),
        ])
        .split(area);

        // ━┥ ENTER ACCESS CODE ┝━
        Line::from(vec![
            "━┥ ".fg(style::BORDER),
            "ENTER ACCESS CODE".fg(style::PRIMARY).bold(),
            " ┝━".fg(style::BORDER),
        ])
        .centered()
        .render(layout[1], buf);

        // [ X ]   [ X ]   [ _ ]   [ _ ]
        let mut spans: Vec<Span<'static>> = Vec::new();
        for i in 0..4 {
            let filled = self.code[i].is_some();
            let bracket_color = if filled { style::PRIMARY } else { style::BORDER };

            spans.push("[".fg(bracket_color));
            spans.push(" ".into());
            spans.push(match self.code[i] {
                Some(c) => c.to_string().fg(style::TEXT).bold(),
                None => "·".fg(style::BORDER),
            });
            spans.push(" ".into());
            spans.push("]".fg(bracket_color));
            if i < 3 {
                spans.push("   ".into());
            }
        }
        Line::from(spans).centered().render(layout[3], buf);

        // // press <enter> to submit
        Line::from(vec![
            "// press ".fg(style::TEXT_DIM).italic(),
            "<enter>".fg(style::HINT_KEY).bold(),
            " to submit".fg(style::TEXT_DIM).italic(),
        ])
        .centered()
        .render(layout[5], buf);
    }
}
