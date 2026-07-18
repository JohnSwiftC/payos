use std::fs::File;

use ratatui::style::Stylize;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{App, Interupt};

use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::Widget;

use rodio::Decoder;

use crate::style;

pub(crate) struct Sound {
    name: &'static str,
    file_path: &'static str,
}

impl Sound {
    pub(crate) fn new(name: &'static str, file_path: &'static str) -> Self {
        Self { name, file_path }
    }
}

pub struct Sounds {
    source: &'static Sound,
}

impl Interupt for Sounds {
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

        Line::from(vec![
            "━┥ ".fg(style::BORDER),
            "PLAYBACK".fg(style::INFO).bold(),
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

    fn callback(&mut self, app: &mut App) {
        let audio_file = File::open(self.source.file_path).expect("failed to open audio file");
        let source = Decoder::try_from(audio_file).unwrap();
        let player = rodio::Player::connect_new(app.mixer_device_sink.mixer());

        player.append(source);

        player.sleep_until_end();
    }
}

pub fn interupt(source: &'static Sound) -> Box<Sounds> {
    Box::new(Sounds { source })
}
