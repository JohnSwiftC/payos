use crate::App;
use crate::Interupt;
use crate::style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

#[allow(dead_code)]
pub struct Generic<F: Fn(&mut App)> {
    message: String,
    callback: F,
}

impl<F: Fn(&mut App)> Interupt for Generic<F> {
    fn render(&mut self, _app: &mut App, area: Rect, buf: &mut Buffer) {
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

        let bar_w = area.width.saturating_sub(8).max(1) as usize;
        let bar = "═".repeat(bar_w);

        Line::from(bar.clone().fg(style::SUCCESS))
            .centered()
            .render(layout[1], buf);

        Line::from(vec![
            "▌ ".fg(style::SUCCESS).bold(),
            "  —  ".fg(style::BORDER),
            self.message.clone().fg(style::SUCCESS).bold(),
            " ▐".fg(style::SUCCESS).bold(),
        ])
        .centered()
        .render(layout[3], buf);

        Line::from(bar.fg(style::SUCCESS))
            .centered()
            .render(layout[5], buf);
    }

    fn callback(&mut self, app: &mut App) {
        (self.callback)(app);
    }
}

// This is a big benefit of having interupts be trait objects
// defining interupts that have a generic works great because
// i can always technically return a Box<dyn Interupt> in these
// constructors
pub fn with_message<F: Fn(&mut App) + 'static>(message: String, callback: F) -> Box<dyn Interupt> {
    Box::new(Generic { message, callback })
}
