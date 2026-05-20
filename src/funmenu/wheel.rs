
use ratatui::{buffer::Buffer, layout::Rect};

use crate::App;
use crate::Interupt;

pub struct Wheel {
    curr: &'static str,
}

impl Interupt for Wheel {
    fn render(&mut self, _: &mut App, _area: Rect, _buf: &mut Buffer) {}

    fn callback(&mut self, _app: &mut App) {}
}

pub fn interupt() -> Box<Wheel> {
    Box::new(Wheel { curr: "Default" })
}
