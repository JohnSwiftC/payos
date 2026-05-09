use std::time::Duration;

use ratatui::text::Text;
use ratatui::widgets::Widget;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::App;
use crate::Interupt;

fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    if app.interupt_args.spinner_text != "Test 1" {
        app.interupt_args.spinner_text = "Test 1";
    } else {
        app.interupt_args.spinner_text = "Test 2";
    }

    let text = Text::from(app.interupt_args.spinner_text);

    text.render(area, buf);
}

fn callback(app: &mut App) {
    std::thread::sleep(Duration::from_secs(3));

    if app.interupt_args.spinner_text == "Test 1" {
        app.interupt = Some(interupt());
    }
}

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
