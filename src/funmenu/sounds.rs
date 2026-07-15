use ratatui::style::Stylize;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::input::InputEvent;
use crate::{App, people};
use crate::{Page, PageSignal, PageState};

use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::Widget;

use crate::style;

pub fn render(state: PageState, app: &mut App, area: Rect, buf: &mut Buffer) {
    let layout = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(area);

    Line::from("Hit any button for a random sound".fg(style::TEXT))
        .centered()
        .render(layout[3], buf);
}

pub fn event_callback(_state: PageState, _app: &mut App, _event: InputEvent) -> Option<PageSignal> {
    // play some sound here lol
    None
}

pub fn page() -> Page {
    Page {
        state: PageState::new(()),
        render,
        event_callback,
        on_load: None,
    }
}
