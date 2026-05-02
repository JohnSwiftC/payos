use ratatui::{buffer::Buffer, layout::Rect};

use crate::{App, util};
use crate::{Page, PageSignal, PageState};
use crossterm::event::Event;

pub fn render(_state: PageState, app: &mut App, area: Rect, buf: &mut Buffer) {
    util::render_centered_image(&app.dog_image, &mut app.image_protocol, area, buf);
}

pub fn event_callback(_state: PageState, _app: &mut App, _event: Event) -> Option<PageSignal> {
    None
}

pub fn on_load(_state: PageState, app: &mut App) {
    // This interupt has a sleep call like unauth
    // but it will wait longer than that because it takes time
    // for the dock image to be rendered in the runder_centered_image function
    app.interupt = Some(super::popup::granted::with_name(app, "John"));
    app.image_protocol = app.picker.new_resize_protocol(app.dog_image.clone());
}

pub fn page() -> Page {
    Page {
        state: PageState::new(()),
        render,
        event_callback,
        on_load: Some(on_load),
    }
}
