mod code;
mod grid;
mod richbutton;
mod secret;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use image::DynamicImage;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use ratatui_image::{picker::Picker, protocol::Protocol};
use std::io;
use std::process;

use crate::code::Code;
use crate::grid::Grid;

pub type WidgetList = Vec<Box<dyn Fn(Rect, &mut Buffer)>>;
pub type WidgetFn = Box<dyn Fn(Rect, &mut Buffer)>;

pub enum PageSignal {
    Back,
    Push(Page),
}

pub struct Page {
    pub render: fn(&App, Rect, &mut Buffer),
    pub event_callback: fn(&mut App, Event) -> Option<PageSignal>,
}

struct App {
    stack: Vec<Page>,
    rows: usize,
    cols: usize,
    highlighted: usize,
    widgets: Vec<WidgetFn>,
    code: Code,
    cat_image: DynamicImage,
    picker: Picker,
}

impl App {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            rows: 2,
            cols: 1,
            highlighted: 0,
            widgets: vec![
                richbutton::action_button("Secret Login", "Requires a password"),
                richbutton::action_button("Some Random Thing", "Does this thing"),
            ],
            code: Code::new("1234".into()),
            cat_image: image::ImageReader::open("cat.jpg")
                .unwrap()
                .decode()
                .unwrap(),
            picker: Picker::from_query_stdio().unwrap(),
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
        let grid = Grid::new(self.rows, self.cols, self.highlighted);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        grid.render(layout[0], buf, &self.widgets);
    }

    fn prop_input(&mut self) {
        let event = event::read().expect("Failed to read a key event, awesome!");

        // * globally pops a page
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('*'),
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            if self.stack.len() == 0 {
                process::exit(0);
            } else {
                _ = self.stack.pop();
                return;
            }
        }

        let signal = if let Some(page) = self.stack.last() {
            (page.event_callback)(self, event)
        } else {
            self.event_callback(event)
        };

        if let Some(signal) = signal {
            match signal {
                PageSignal::Push(p) => self.stack.push(p),
                PageSignal::Back => {
                    _ = self.stack.pop();
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

            KeyCode::Enter => match self.highlighted {
                0 => return Some(PageSignal::Push(secret::page())),
                _ => (),
            },

            _ => (),
        }

        None
    }
}

fn main() -> io::Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
