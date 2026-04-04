use crate::App;
use crate::Interupt;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
};

pub fn render(_: &mut App, area: Rect, buf: &mut Buffer) {
    let text = Text::from(Line::from("Unauthorized".red().bold().on_white()));
    let centered = area.centered(
        Constraint::Length(text.width() as u16),
        Constraint::Length(1),
    );

    text.render(centered, buf);
}

pub fn callback(_: &mut App) {
    std::thread::sleep(std::time::Duration::from_secs(3));
}

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
