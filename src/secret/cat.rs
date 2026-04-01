use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::App;
use crate::WidgetFn;
use crate::{Page, PageSignal};

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    let text = Text::from("This is a cat");

    text.render(area, buf);
}

pub fn event_callback(app: &mut App, event: Event) -> Option<PageSignal> {
    None
}

pub fn page() -> Page {
    Page {
        render: render,
        event_callback: event_callback,
    }
}
