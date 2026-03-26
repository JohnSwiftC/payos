use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crossterm::event::Event;

use crate::App;
use crate::WidgetFn;
use crate::{Page, PageSignal};

pub fn secret_render(app: &App, area: Rect, buf: &mut Buffer) {
    let t = Text::from("Test");

    let p = Paragraph::new(t);
    p.render(area, buf);
}

pub fn secret_callback(app: &mut App, event: Event) -> Option<PageSignal> {
    Some(PageSignal::Back)
}

pub fn secret_page() -> Page {
    Page {
        render: secret_render,
        event_callback: secret_callback,
    }
}
