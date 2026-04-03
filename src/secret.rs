pub mod cat;

use ratatui::{buffer::Buffer, layout::Rect};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::App;
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

        KeyCode::Enter => {
            if app.code.is_correct() {
                return Some(PageSignal::Push(cat::page()));
            }
        }

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
