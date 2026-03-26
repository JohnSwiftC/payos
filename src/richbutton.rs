use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::WidgetFn;

pub fn action_button(title: &'static str, desc: &'static str) -> WidgetFn {
    Box::new(|r, b| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(r);

        let title = Text::from(title.blue().bold()).centered();

        title.render(layout[0], b);

        let desc = Text::from(desc.italic()).centered();

        desc.render(layout[1], b);
    })
}
