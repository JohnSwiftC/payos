use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::Widget,
};

use crate::WidgetFn;
use crate::style;

pub fn action_button(title: &'static str, desc: &'static str) -> WidgetFn {
    Box::new(move |r, b| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(r);

        // ◆ TITLE
        Line::from(vec![
            "◆ ".fg(style::PRIMARY),
            title.fg(style::TEXT).bold(),
        ])
        .centered()
        .render(layout[1], b);

        // ─── divider
        let bar_w = r.width.saturating_sub(6).max(1) as usize;
        Line::from("─".repeat(bar_w).fg(style::BORDER))
            .centered()
            .render(layout[2], b);

        // // description
        Line::from(vec![
            "// ".fg(style::TEXT_DIM),
            desc.fg(style::TEXT_DIM).italic(),
        ])
        .centered()
        .render(layout[3], b);
    })
}
