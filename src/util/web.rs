use crate::App;
use crate::Interupt;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
};

use local_ip_address;

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    let local = local_ip_address::local_ip().unwrap().to_string();

    let mut message = String::new();
    message.push_str("Visit ");
    message.push_str(&local);
    message.push_str(" for config");

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

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
