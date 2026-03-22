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
    render: fn(&App, Rect, &mut Buffer),
    event_callback: fn(&mut App, Event) -> Option<PageSignal>,
}

struct App {
    stack: Vec<Page>,
    rows: usize,
    cols: usize,
    highlighted: usize,
}

impl App {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            stack: Vec::new(),
            rows,
            cols,
            highlighted: 0,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.prop_input();
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .border_set(border::DOUBLE)
            .title(Line::from(" Sunrise V Landline ".bold().yellow()).centered())
            .title_bottom(
                Line::from(vec![" Exit Page With ".blue(), "<*> ".blue().bold()]).centered(),
            );

        if let Some(page) = self.stack.last() {
            (page.render)(self, block.inner(frame.area()), frame.buffer_mut());
        } else {
            self.render(block.inner(frame.area()), frame.buffer_mut());
        }

        block.render(frame.area(), frame.buffer_mut());
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let grid = Grid::new(
            self.rows,
            self.cols,
            self.highlighted,
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

        let signal = if let Some(page) = self.stack.last() {
            (page.event_callback)(self, event)
        } else {
            self.event_callback(event)
        };

        if let Some(signal) = signal {
            match signal {
                PageSignal::Push(p) => self.stack.push(p),
                PageSignal::Back => {
                    let _ = self.stack.pop();
                }
            }
        }
    }

    fn event_callback(&mut self, event: Event) -> Option<PageSignal> {
        let key = match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => key,
            _ => {
                return None;
            }
        };

        let max_index = self.rows * self.cols - 1;
        match key.code {
            KeyCode::Down => {
                self.highlighted = max_index.min(self.highlighted + self.cols);
            }

            KeyCode::Up => {
                if self.cols > self.highlighted {
                    self.highlighted = 0;
                } else {
                    self.highlighted -= self.cols;
                }
            }

            KeyCode::Right => {
                self.highlighted = max_index.min(self.highlighted + 1);
            }

            KeyCode::Left => {
                if self.highlighted != 0 {
                    self.highlighted -= 1;
                }
            }

            _ => (),
        }

        None
    }
}

fn main() -> io::Result<()> {
    let mut app = App::new(2, 1);
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
