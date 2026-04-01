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
    let img_w = app.cat_image.width() as u32;
    let img_h = app.cat_image.height() as u32;

    // we are using full height
    let render_height = area.height as u32;

    // preserve aspect ratio
    let render_width = ((render_height * img_w) / img_h) as u16;

    // clamp so it doesn't exceed area
    let render_width = render_width.min(area.width);
    let x = area.x + (area.width - render_width) / 2;

    let centered = Rect {
        x,
        y: area.y,
        width: render_width,
        height: area.height,
    };
    let protocol = app
        .picker
        .new_protocol(
            app.cat_image.clone(),
            centered,
            ratatui_image::Resize::Scale(Some(ratatui_image::FilterType::Triangle)),
        )
        .unwrap();

    let cat_widget = Image::new(&protocol);
    cat_widget.render(centered, buf);
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
