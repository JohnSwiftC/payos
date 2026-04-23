use crate::App;
use crate::Interupt;
use crate::util;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
};

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    util::render_centered_image(&app.sunrise_image, &mut app.image_protocol, area, buf);
}

pub fn callback(app: &mut App) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
