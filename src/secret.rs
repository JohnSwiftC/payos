mod cat;
mod dog;

use ratatui::{buffer::Buffer, layout::Rect};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::App;
use crate::popup;
use crate::widgets::code::Code;
use crate::{Page, PageSignal, PageState};

struct SecretState {
    code: Code,
}

pub fn render(state: PageState, app: &mut App, area: Rect, buf: &mut Buffer) {
    let state = state.access::<SecretState>();

    state.code.render(area, buf);
}

pub fn callback(state: PageState, app: &mut App, event: Event) -> Option<PageSignal> {
    let mut state = state.access::<SecretState>();

    let key = match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => key,
        _ => {
            return None;
        }
    };

    match key.code {
        KeyCode::Backspace => state.code.pop(),

        KeyCode::Enter => match state.code.get_code().as_deref() {
            Some("1234") => return Some(PageSignal::Push(cat::page())),
            Some("6824") => return Some(PageSignal::Push(dog::page())),

            _ => return Some(PageSignal::Interupt(popup::unauth::interupt())),
        },

        KeyCode::Char(c) => state.code.push(c),
        _ => (),
    }

    None
}

pub fn page() -> Page {
    Page {
        state: PageState::new(SecretState { code: Code::new() }),
        render,
        event_callback: callback,
        on_load: None,
    }
}
