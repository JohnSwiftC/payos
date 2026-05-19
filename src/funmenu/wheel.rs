use std::time::Duration;

use ratatui::text::Text;
use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::App;
use crate::Interupt;

pub struct Wheel {
    curr: &'static str,
}

impl Interupt for Wheel {
    fn render(&mut self, _: &mut App, area: Rect, buf: &mut Buffer) {}

    fn callback(&mut self, app: &mut App) {}
}

pub fn interupt() -> Box<Wheel> {
    Box::new(Wheel { curr: "Default" })
}
