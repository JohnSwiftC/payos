use ratatui::{buffer::Buffer, layout::Rect};

use crate::input::InputEvent;
use crate::{App, people};
use crate::{Page, PageSignal, PageState};

use crate::widgets::grid;
use crate::widgets::richbutton;

mod wheel;

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
        richbutton::action_button("Spin the Wheel", "Just like CS:GO"),
        richbutton::action_button("Game 2", "Insert game 2 lol"),
        richbutton::action_button("Reels", "Stimmax"),
    ];

    grid.render(area, buf, &buttons);
}

pub fn callback(state: PageState, app: &mut App, event: InputEvent) -> Option<PageSignal> {
    let mut state = state.access::<FunmenuState>();

    match event {
        InputEvent::Down => state.increment_index(),
        InputEvent::Up => state.decrement_index(),
        InputEvent::Left => state.decrement_index(),
        InputEvent::Right => state.increment_index(),

        InputEvent::Enter => match state.index {
            0 => {
                return Some(PageSignal::Interupt(wheel::interupt(
                    app.store.get_people(),
                )));
            }

            1 => {
                return Some(PageSignal::Push(people::page()));
            }

            _ => return None,
        },

        _ => (),
    }

    None
}

pub fn page() -> Page {
    Page {
        state: PageState::new(FunmenuState { index: 0, max: 2 }),
        render,
        event_callback: callback,
        on_load: None,
    }
}
