mod cat;
mod dog;

use ratatui::{buffer::Buffer, layout::Rect};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::App;
use crate::popup;
use crate::{Page, PageSignal};

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    app.code.render(area, buf);
}

pub fn callback(app: &mut App, event: Event) -> Option<PageSignal> {
    let key = match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => key,
        _ => {
            return None;
        }
    };

    match key.code {
        KeyCode::Backspace => app.code.pop(),

        KeyCode::Enter => match app.code.get_code().as_deref() {
            Some("1234") => return Some(PageSignal::Push(cat::page())),
            Some("6824") => return Some(PageSignal::Push(dog::page())),

            _ => return Some(PageSignal::Interupt(popup::unauth::interupt())),
        },

        KeyCode::Char(c) => app.code.push(c),
        _ => (),
    }

    None
}

pub fn page() -> Page {
    Page {
        render,
        event_callback: callback,
        on_load: None,
    }
}
