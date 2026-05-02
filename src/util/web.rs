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

use std::process;

use local_ip_address;

pub fn render(_: &mut App, area: Rect, buf: &mut Buffer) {
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

pub fn callback(_: &mut App) {
    let web_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("web");

    let mut cmd = process::Command::new("go");
    cmd.args(["run", ".", "payos.db"])
        .current_dir(&web_dir)
        .stderr(process::Stdio::null())
        .stdout(process::Stdio::null());

    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("spawn failed (cmd = {cmd:?}): {e}"));

    if !status.success() {
        eprintln!("go exited: {status}");
    }
}

pub fn interupt() -> Interupt {
    Interupt { render, callback }
}
