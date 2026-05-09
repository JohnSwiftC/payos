mod cat;
mod dog;

use ratatui::{buffer::Buffer, layout::Rect};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::App;
use crate::InputEvent;
use crate::popup;
use crate::widgets::code::Code;
use crate::{Page, PageSignal, PageState};

struct SecretState {
    code: Code,
}

pub fn render(state: PageState, _app: &mut App, area: Rect, buf: &mut Buffer) {
    let state = state.access::<SecretState>();

    state.code.render(area, buf);
}

pub fn callback(state: PageState, _app: &mut App, event: InputEvent) -> Option<PageSignal> {
    let mut state = state.access::<SecretState>();

    match event {
        InputEvent::Backspace => state.code.pop(),

        InputEvent::Enter => match state.code.get_code().as_deref() {
            Some("1234") => return Some(PageSignal::Push(cat::page())),
            Some("6824") => return Some(PageSignal::Push(dog::page())),

            _ => return Some(PageSignal::Interupt(popup::unauth::interupt())),
        },

        InputEvent::Char(c) => state.code.push(c),
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
