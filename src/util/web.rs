use crate::App;
use crate::Interupt;
use crate::style;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use std::os::unix::process::CommandExt;
use std::process;
use std::time::Duration;

use local_ip_address;

pub struct Web;

impl Interupt for Web {
    fn render(&mut self, _: &mut App, area: Rect, buf: &mut Buffer) {
        let local = local_ip_address::local_ip().unwrap().to_string();

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
            "▶ ".fg(style::PRIMARY),
            "http://".fg(style::TEXT_DIM),
            local.fg(style::TEXT).bold().underlined(),
            ":8080".fg(style::TEXT_DIM),
        ])
        .centered()
        .render(layout[3], buf);

        Line::from(vec![
            "// ".fg(style::BORDER),
            "awaiting connections on LAN".fg(style::TEXT_DIM).italic(),
        ])
        .centered()
        .render(layout[5], buf);
    }

    fn callback(&mut self, _: &mut App) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let web_dir = root.join("web");
        let db_path = root.join("payos.db");

        let mut cmd = process::Command::new("go");
        cmd.args(["run", ".", db_path.to_str().unwrap()])
            .current_dir(&web_dir)
            .stderr(process::Stdio::null())
            .stdout(process::Stdio::null())
            .process_group(0);

        let mut child = cmd
            .spawn()
            .unwrap_or_else(|e| panic!("spawn failed (cmd = {cmd:?}): {e}"));

        let pgid = child.id();

        loop {
            // Did the server exit on its own (killed externally)?
            match child.try_wait() {
                Ok(Some(status)) => {
                    if !status.success() {
                        eprintln!("go exited: {status}");
                    }
                    break;
                }
                Ok(None) => {} // still running
                Err(e) => panic!("failed to wait on go process: {e}"),
            }

            if event::poll(Duration::from_millis(100)).unwrap_or(false)
                && let Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('*'),
                    kind: KeyEventKind::Press,
                    ..
                })) = event::read()
            {
                let _ = process::Command::new("kill")
                    .args(["-TERM", &format!("-{pgid}")])
                    .status();
                let _ = child.wait();
                break;
            }
        }
    }
}

pub fn interupt() -> Box<Web> {
    Box::new(Web)
}
