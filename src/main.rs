use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;

enum PageSignal {
    Back,
    Push(Page),
}

struct Page {
    render: &'static dyn Fn(Rect, &mut Buffer),
    event_callback: &'static dyn Fn() -> Option<PageSignal>,
}

struct App {
    stack: Vec<Page>,
}

impl App {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) {
        loop {
            terminal.draw(|frame| self.draw(frame));
        }
    }

    fn draw(&self, frame: &mut Frame) {
        if let Some(page) = self.stack.last() {
            (page.render)(frame.area(), frame.buffer_mut());
        }
    }
}

fn main() {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal));
}
