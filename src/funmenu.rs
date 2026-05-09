use ratatui::{buffer::Buffer, layout::Rect};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::App;
use crate::input::InputEvent;
use crate::popup;
use crate::{Page, PageSignal, PageState};

use crate::widgets::grid;
use crate::widgets::richbutton;

struct FunmenuState {
    index: usize,
    max: usize,
}

impl FunmenuState {
    fn increment_index(&mut self) {
        if self.index != self.max {
            self.index += 1;
        }
    }

    fn decrement_index(&mut self) {
        if self.index != 0 {
            self.index -= 1;
        }
    }
}

pub fn render(state: PageState, _app: &mut App, area: Rect, buf: &mut Buffer) {
    let state = state.access::<FunmenuState>();

    let grid = grid::Grid::new(3, 1, state.index);

    let buttons = vec![
        richbutton::action_button("Drinking Wheel", "Spin The Wheel"),
        richbutton::action_button("Game 2", "Insert game 2 lol"),
        richbutton::action_button("Reels", "Stimmax"),
    ];

    grid.render(area, buf, &buttons);
}

pub fn callback(state: PageState, _app: &mut App, event: InputEvent) -> Option<PageSignal> {
    let mut state = state.access::<FunmenuState>();

    match event {
        InputEvent::Down => state.increment_index(),
        InputEvent::Up => state.decrement_index(),
        InputEvent::Left => state.decrement_index(),
        InputEvent::Right => state.increment_index(),

        _ => (),
    }

    None
}

pub fn page() -> Page {
    Page {
        state: PageState::new(FunmenuState { index: 0, max: 0 }),
        render,
        event_callback: callback,
        on_load: None,
    }
}
