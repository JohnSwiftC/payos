use crate::App;
use crate::Interupt;
use crate::style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
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

    Line::from(bar.clone().fg(style::SUCCESS))
        .centered()
        .render(layout[1], buf);

    Line::from(vec![
        "▌ ".fg(style::SUCCESS).bold(),
        "ACCESS GRANTED".fg(style::ON_SUCCESS).bold(),
        "  —  ".fg(style::BORDER),
        app.interupt_args.granted_name.fg(style::SUCCESS).bold(),
        " ▐".fg(style::SUCCESS).bold(),
    ])
    .centered()
    .render(layout[3], buf);

    Line::from(bar.fg(style::SUCCESS))
        .centered()
        .render(layout[5], buf);
}

pub fn callback(_: &mut App) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn interupt() -> Interupt {
    Interupt { render, callback }
}

pub fn with_name(app: &mut App, name: &'static str) -> Interupt {
    app.interupt_args.granted_name = name;
    self::interupt()
}
