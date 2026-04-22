use crate::App;
use crate::Interupt;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
};

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    let mut message = String::new();
    message.push_str("Welcome ");
    message.push_str(app.interupt_args.granted_name);
    message.push('!');

    let text = Text::from(message).blue();
    let centered = area.centered(
        Constraint::Length(text.width() as u16),
        Constraint::Length(1),
    );

    text.render(centered, buf);
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
