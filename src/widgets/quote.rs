use ratatui::{
    layout::Constraint,
    style::Stylize,
    text::{Line, Text},
    widgets::Widget,
};

use crate::WidgetFn;
use crate::style;

/// Centered pull-quote with smart-quote brackets, like:
///
///     ❝
///   <line 1>
///   <line 2>
///     ❞
pub fn quote(lines: &'static [&'static str]) -> WidgetFn {
    Box::new(move |r, b| {
        let mut content: Vec<Line> = Vec::with_capacity(lines.len() + 4);
        content.push(Line::from("❝".fg(style::PRIMARY).bold()).centered());
        content.push(Line::from(""));
        for &line in lines {
            content.push(Line::from(line.fg(style::TEXT).italic()).centered());
        }
        content.push(Line::from(""));
        content.push(Line::from("❞".fg(style::PRIMARY).bold()).centered());

        let text = Text::from(content);
        let spot = r.centered(
            Constraint::Length(text.width() as u16),
            Constraint::Length(text.height() as u16),
        );
        text.render(spot, b);
    })
}
