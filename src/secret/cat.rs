use ratatui::{buffer::Buffer, layout::Rect};

use crate::{App, util};
use crate::{Page, PageSignal, PageState};
use crossterm::event::Event;

pub fn render(_state: PageState, app: &mut App, area: Rect, buf: &mut Buffer) {
    util::render_centered_image(&app.cat_image, &mut app.image_protocol, area, buf);
}

pub fn event_callback(_state: PageState, _app: &mut App, _event: Event) -> Option<PageSignal> {
    None
}

pub fn on_load(_state: PageState, app: &mut App) {
    app.image_protocol = app.picker.new_resize_protocol(app.cat_image.clone());
}

pub fn page() -> Page {
    Page {
        state: PageState::new(()),
        render,
        event_callback,
        on_load: Some(on_load),
    }
}
