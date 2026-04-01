use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use ratatui_image::{
    Image, StatefulImage,
    picker::{Picker, ProtocolType},
    protocol::Protocol,
};

use crate::App;
use crate::WidgetFn;
use crate::{Page, PageSignal};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use image::imageops::FilterType;

pub fn render(app: &mut App, area: Rect, buf: &mut Buffer) {
    let img_w = app.cat_image.width() as u32;
    let img_h = app.cat_image.height() as u32;

    // For halfblocks, each terminal row effectively carries 2 vertical image pixels.
    // That means the image's aspect ratio in CELL SPACE is about 2 * img_w / img_h.
    let width_mul = if app.picker.protocol_type() == ProtocolType::Halfblocks {
        2
    } else {
        1
    };

    let render_width =
        ((area.height as u32 * img_w * width_mul) / img_h).min(area.width as u32) as u16;

    let centered = Rect {
        x: area.x + area.width.saturating_sub(render_width) / 2,
        y: area.y,
        width: render_width,
        height: area.height,
    };

    let image =
        StatefulImage::new().resize(ratatui_image::Resize::Scale(Some(FilterType::Triangle)));
    image.render(centered, buf, &mut app.image_protocol);
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
