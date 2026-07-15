use ratatui::style::Stylize;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::input::InputEvent;
use crate::popup::generic;
use crate::{App, people};
use crate::{Page, PageSignal, PageState};

use crate::widgets::grid;
use crate::widgets::richbutton;

use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::Widget;

use crate::style;

struct SoundsState;

pub fn render(state: PageState, app: &mut App, area: Rect, buf: &mut Buffer) {
    let state = state.access::<SoundsState>();

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
    None
}
