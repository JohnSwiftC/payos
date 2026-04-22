use crate::App;
use crate::Event;
use crate::Page;
use crate::PageSignal;
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
};

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    let people = app.store.get_people();
    let mut list = String::new();

    for p in people {
        list.push_str(&p);
        list.push(' ');
    }

    let text = Text::from(list).blue();
    let centered = area.centered(
        Constraint::Length(text.width() as u16),
        Constraint::Length(1),
    );

    text.render(centered, buf);
}

pub fn callback(app: &mut App, event: Event) -> Option<PageSignal> {
    None
}

pub fn page() -> Page {
    Page {
        render,
        event_callback: callback,
        on_load: None,
    }
}
