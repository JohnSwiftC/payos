use std::time::Duration;

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

use rand::seq::SliceRandom;

pub struct Wheel {
    names: Vec<String>,
    iteration: usize,
    end: bool,
}

impl Interupt for Wheel {
    fn render(&mut self, app: &mut App, area: Rect, buf: &mut Buffer) {
        if self.names.is_empty() {
            no_names_error(area, buf);
            return;
        }

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

        let desc = app.store.get_wheel_desc();

        let top_bar_w = area
            .width
            .saturating_sub(8)
            .saturating_sub(desc.len() as u16 + 2)
            .max(1) as usize;

        let bottom_bar_w = area.width.saturating_sub(8).max(1) as usize;

        let bar = "═".repeat(top_bar_w / 2);
        let bottom_bar = "═".repeat(bottom_bar_w);

        let index = self.iteration % self.names.len();
        let name = self.names[index].clone();

        let name = if self.end {
            name.fg(style::SUCCESS).bold()
        } else {
            name.fg(style::ALERT)
        };

        Line::from(vec![
            bar.clone().fg(style::ALERT),
            " ".bold(),
            desc.fg(style::PRIMARY).bold(),
            " ".bold(),
            bar.clone().fg(style::ALERT),
        ])
        .centered()
        .render(layout[1], buf);

        Line::from(vec![
            "▌ ".fg(style::SUCCESS).bold(),
            name,
            " ▐".fg(style::SUCCESS).bold(),
        ])
        .centered()
        .render(layout[3], buf);

        Line::from(bottom_bar.fg(style::ALERT))
            .centered()
            .render(layout[5], buf);
    }

    fn callback(&mut self, app: &mut App) {
        let names = std::mem::take(&mut self.names);

        if names.is_empty() {
            std::thread::sleep(Duration::from_secs(3));
            return;
        }

        let spins = 30;

        if self.iteration != 30 {
            let end = self.iteration + 1 == 30;
            app.interupt = Some(Box::new(Wheel {
                names,
                iteration: self.iteration + 1,
                end,
            }))
        }

        if self.iteration == spins {
            std::thread::sleep(std::time::Duration::from_secs(5));
        } else {
            std::thread::sleep(std::time::Duration::from_millis(
                self.iteration as u64 * self.iteration as u64,
            ));
        }
    }
}

fn no_names_error(area: Rect, buf: &mut Buffer) {
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

    Line::from(vec![bar.clone().fg(style::ALERT)])
        .centered()
        .render(layout[1], buf);

    Line::from(vec![
        "▌ ".fg(style::SUCCESS).bold(),
        "No names in config DB, visit the config menu!".fg(style::ALERT),
        " ▐".fg(style::SUCCESS).bold(),
    ])
    .centered()
    .render(layout[3], buf);

    Line::from(bar.fg(style::ALERT))
        .centered()
        .render(layout[5], buf);
}

pub fn interupt(mut names: Vec<String>) -> Box<Wheel> {
    let mut rng = rand::rng();
    names.shuffle(&mut rng);

    Box::new(Wheel {
        names,
        iteration: 0,
        end: false,
    })
}
