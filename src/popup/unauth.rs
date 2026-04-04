use crate::App;
use crate::{Interupt, PageSignal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
};

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    let text = Text::from(Line::from("Unauthorized".red()));
    let centered = area.centered(
        Constraint::Length(text.width() as u16),
        Constraint::Length(1),
    );

    text.render(centered, buf);
}

pub fn callback(app: &mut App) {
    std::thread::sleep(std::time::Duration::from_secs(3));
}

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
