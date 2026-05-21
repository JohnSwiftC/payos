use std::collections::VecDeque;
use std::time::Duration;

use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::text::{Line, Span};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::App;
use crate::Interupt;
use crate::style;
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

        let line = if self.end {
            vec![
                "▌ ".fg(style::SUCCESS).bold(),
                name,
                " ▐".fg(style::SUCCESS).bold(),
            ]
        } else {
            build_wheel_spinner(&self.names, index, bottom_bar_w)
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

        Line::from(line).centered().render(layout[3], buf);

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

fn build_wheel_spinner(names: &[String], index: usize, width: usize) -> Vec<Span<'_>> {
    let max_index = names.len() - 1;
    let center = names[index].clone();

    let side_budget = width.saturating_sub(center.len() + 5) / 2;

    let mut left_spans: VecDeque<Span> = VecDeque::new();
    let mut right_spans: Vec<Span> = Vec::new();
    let mut left_width = 0usize;
    let mut right_width = 0usize;
    let mut left = index;
    let mut right = index;
    let mut left_done = false;
    let mut right_done = false;

    while !(left_done && right_done) {
        if !left_done {
            let i = if left == 0 { max_index } else { left - 1 };
            let add = names[i].len() + 1;
            if left_width + add <= side_budget {
                left = i;
                left_spans.push_front(names[i].clone().fg(style::CREAM));
                left_spans.push_front(" ".into());
                left_width += add;
            } else {
                left_done = true;
            }
        }

        if !right_done {
            let i = if right == max_index { 0 } else { right + 1 };
            let add = names[i].len() + 1;
            if right_width + add <= side_budget {
                right = i;
                right_spans.push(" ".into());
                right_spans.push(names[i].clone().fg(style::CREAM));
                right_width += add;
            } else {
                right_done = true;
            }
        }
    }

    if left_width < right_width {
        left_spans.push_front(" ".repeat(right_width - left_width).into());
    } else if right_width < left_width {
        right_spans.push(" ".repeat(left_width - right_width).into());
    }

    let mut result: Vec<Span> = left_spans.into();
    result.push(" ▌ ".fg(style::SUCCESS).bold());
    result.push(center.fg(style::ALERT));
    result.push(" ▐".fg(style::SUCCESS).bold());
    result.extend(right_spans);
    result
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
