use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use ratatui_image::StatefulImage;

use crate::{App, util};
use crate::{Page, PageSignal};
use crossterm::event::Event;
use image::imageops::FilterType;

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    util::render_centered_image(&app.cat_image, &mut app.image_protocol, area, buf);
}

pub fn event_callback(_app: &mut App, _event: Event) -> Option<PageSignal> {
    None
}

pub fn on_load(app: &mut App) {
    app.image_protocol = app.picker.new_resize_protocol(app.cat_image.clone());
}

pub fn page() -> Page {
    Page {
        render,
        event_callback,
        on_load: Some(on_load),
    }
}
