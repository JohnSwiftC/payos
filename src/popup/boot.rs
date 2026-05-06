use crate::App;
use crate::Interupt;
use crate::util;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
};

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    util::render_centered_image(&app.sunrise_image, &mut app.image_protocol, area, buf);
}

pub fn callback(_app: &mut App) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
