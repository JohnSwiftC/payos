pub mod saved;
pub mod web;

use image::DynamicImage;
use ratatui::widgets::StatefulWidget;
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui_image::FilterType;
use ratatui_image::StatefulImage;
use ratatui_image::protocol::StatefulProtocol;

pub fn render_centered_image(
    image: &DynamicImage,
    protocol: &mut StatefulProtocol,
    area: Rect,
    buf: &mut Buffer,
) {
    let img_w = image.width();
    let img_h = image.height();

    let render_width = ((area.height as u32 * img_w * 2) / img_h).min(area.width as u32) as u16;

    let centered = Rect {
        x: area.x + area.width.saturating_sub(render_width) / 2,
        y: area.y,
        width: render_width,
        height: area.height,
    };

    let image =
        StatefulImage::new().resize(ratatui_image::Resize::Scale(Some(FilterType::Triangle)));
    image.render(centered, buf, protocol);
}
