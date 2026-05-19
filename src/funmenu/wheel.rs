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
    fn render(&mut self, _: &mut App, area: Rect, buf: &mut Buffer) {
        if self.curr != "Test 1" {
            self.curr = "Test 1";
        } else {
            self.curr = "Test 2";
        }

        let text = Text::from(self.curr);

        text.render(area, buf);
    }

    fn callback(&mut self, app: &mut App) {
        std::thread::sleep(Duration::from_secs(3));

        if self.curr == "Test 1" {
            app.interupt = Some(interupt());
        }
    }
}

pub fn interupt() -> Box<Wheel> {
    Box::new(Wheel { curr: "Default" })
}
