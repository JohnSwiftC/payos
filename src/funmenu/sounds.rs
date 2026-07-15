use std::fs::File;
use std::path::Path;

use ratatui::style::Stylize;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::input::InputEvent;
use crate::{App, Interupt, people};
use crate::{Page, PageSignal, PageState};

use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::Widget;

use rodio::{Decoder, MixerDeviceSink, source::Source};

use crate::style;

pub(crate) struct Sound {
    name: &'static str,
    file: File,
}

impl Sound {
    pub(crate) fn from_file<F: AsRef<Path>>(name: &'static str, path: F) -> std::io::Result<Self> {
        let file = File::open(path)?;

        Ok(Self { name, file })
    }
}

pub struct Sounds<'a> {
    source: &'a Sound,
}

impl<'a> Interupt for Sounds<'a> {
    fn render(&mut self, app: &mut App, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1), // header
            Constraint::Length(1), // gap
            Constraint::Length(1), // url
            Constraint::Length(1), // gap
            Constraint::Length(1), // status
            Constraint::Min(0),
        ])
        .split(area);

        Line::from(vec![
            "━┥ ".fg(style::BORDER),
            "CONFIG SERVER ONLINE".fg(style::INFO).bold(),
            " ┝━".fg(style::BORDER),
        ])
        .centered()
        .render(layout[1], buf);

        Line::from(vec![
            "Currently playing: ".fg(style::TEXT_DIM),
            self.source.name.fg(style::TEXT),
            "...".fg(style::TEXT_DIM),
        ])
        .centered()
        .render(layout[3], buf);
    }

    fn callback(&mut self, app: &mut App) {}
}

pub fn interupt(source: &Sound) -> Box<Sounds<'_>> {
    Box::new(Sounds { source })
}
