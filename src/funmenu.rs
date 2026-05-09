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
    None
}

pub fn page() -> Page {
    Page {
        state: PageState::new(FunmenuState { index: 0 }),
        render,
        event_callback: callback,
        on_load: None,
    }
}
