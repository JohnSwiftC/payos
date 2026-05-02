use crate::App;
use crate::Interupt;
use crate::style;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

pub fn render(_: &mut App, area: Rect, buf: &mut Buffer) {
    let layout = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(area);

    let bar_w = area.width.saturating_sub(8).max(1) as usize;
    let bar = "═".repeat(bar_w);

    Line::from(bar.clone().fg(style::ALERT))
        .centered()
        .render(layout[1], buf);

    Line::from(vec![
        "▌".fg(style::ALERT).bold(),
        Span::styled(
            "  ACCESS DENIED  ",
            Style::new().fg(style::ON_ALERT).bg(style::ALERT).bold(),
        ),
        "▐".fg(style::ALERT).bold(),
    ])
    .centered()
    .render(layout[3], buf);

    Line::from(bar.fg(style::ALERT))
        .centered()
        .render(layout[5], buf);
}

pub fn callback(_: &mut App) {
    std::thread::sleep(std::time::Duration::from_secs(3));
}

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
