use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use ratatui_image::{Image, picker::Picker, protocol::Protocol};

use crate::App;
use crate::WidgetFn;
use crate::{Page, PageSignal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use image::imageops::FilterType;

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    let resized =
        app.cat_image
            .resize_exact(area.width.into(), area.height.into(), FilterType::Triangle);
    let protocol = app
        .picker
        .new_protocol(
            app.cat_image.clone(),
            area,
            ratatui_image::Resize::Fit(None),
        )
        .unwrap();

    let cat_widget = Image::new(&protocol);
    cat_widget.render(area, buf);
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
