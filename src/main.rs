mod grid;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;

use crate::grid::Grid;

enum PageSignal {
    Back,
    Push(Page),
}

struct Page {
    render: fn(Rect, &mut Buffer),
    event_callback: fn(&mut Self, Event) -> Option<PageSignal>,
}

struct App {
    stack: Vec<Page>,
    rows: usize,
    cols: usize,
}

impl App {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            stack: Vec::new(),
            rows,
            cols,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .border_set(border::DOUBLE)
            .title(Line::from(" Sunrise V Landline ".bold().yellow()));

        if let Some(page) = self.stack.last() {
            (page.render)(block.inner(frame.area()), frame.buffer_mut());
        } else {
            Self::render(block.inner(frame.area()), frame.buffer_mut());
        }

        block.render(frame.area(), frame.buffer_mut());
    }

    fn render(area: Rect, buf: &mut Buffer) {
        let grid = Grid::new(
            2,
            1,
            vec![
                Box::new(|r, b| {
                    Text::from("Hello").render(r, b);
                }),
                Box::new(|r, b| {
                    Text::from("World!").render(r, b);
                }),
            ],
        );
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        grid.render(layout[0], buf);
    }

    fn prop_input(&mut self) {
        let event = event::read().expect("Failed to read a key event, awesome!");
    }

    fn event_callback(&mut self, event: Event) -> Option<PageSignal> {
        None
    }
}

fn main() -> io::Result<()> {
    let mut app = App::new(3, 2);
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
