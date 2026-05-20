use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::text::{Line, Span};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::App;
use crate::Interupt;
use crate::style;
use crate::util::saved;
use ratatui::style::Stylize;
use ratatui::widgets::Widget;

pub struct Wheel {
    names: Vec<String>,
    iteration: usize,
}

impl Interupt for Wheel {
    fn render(&mut self, _: &mut App, area: Rect, buf: &mut Buffer) {
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

        let index = self.iteration % self.names.len();
        let name = self.names[index].clone();

        Line::from(bar.clone().fg(style::ALERT))
            .centered()
            .render(layout[1], buf);

        Line::from(vec![
            "▌ ".fg(style::SUCCESS).bold(),
            name.fg(style::ALERT),
            " ▐".fg(style::SUCCESS).bold(),
        ])
        .centered()
        .render(layout[3], buf);

        Line::from(bar.fg(style::ALERT))
            .centered()
            .render(layout[5], buf);
    }

    fn callback(&mut self, app: &mut App) {
        if self.iteration != 40 {
            app.interupt = Some(Box::new(Wheel {
                names: app.store.get_people(),
                iteration: self.iteration + 1,
            }))
        }

        std::thread::sleep(std::time::Duration::from_millis(
            self.iteration as u64 * 100,
        ));
    }
}

pub fn interupt(names: Vec<String>) -> Box<Wheel> {
    Box::new(Wheel {
        names,
        iteration: 0,
    })
}
