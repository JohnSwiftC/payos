use crate::App;
use crate::Interupt;
use crate::util;
use ratatui::{buffer::Buffer, layout::Rect};

pub struct Boot;

impl Interupt for Boot {
    fn render(&mut self, app: &mut App, area: Rect, buf: &mut Buffer) {
        util::render_centered_image(&app.sunrise_image, &mut app.image_protocol, area, buf);
    }

    fn callback(&mut self, app: &mut App) {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

pub fn interupt() -> Box<Boot> {
    Box::new(Boot)
}
