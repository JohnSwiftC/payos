mod people;
mod popup;
mod secret;
mod util;
mod widgets;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use image::DynamicImage;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use sqlite::Connection;
use std::io;
use std::process;

use crate::widgets::grid::Grid;
use crate::widgets::{code::Code, richbutton};

use crate::util::saved;

pub type WidgetList = Vec<Box<dyn Fn(Rect, &mut Buffer)>>;
pub type WidgetFn = Box<dyn Fn(Rect, &mut Buffer)>;

pub enum PageSignal {
    Back,
    Push(Page),
    Interupt(Interupt),
}

pub struct Page {
    pub render: fn(&mut App, Rect, &mut Buffer),
    pub event_callback: fn(&mut App, Event) -> Option<PageSignal>,
    pub on_load: Option<fn(&mut App)>,
}

pub struct InteruptArgs {
    granted_name: &'static str,
}

impl Default for InteruptArgs {
    fn default() -> Self {
        Self {
            granted_name: "Default",
        }
    }
}

pub struct Interupt {
    pub render: fn(&mut App, Rect, &mut Buffer),
    pub callback: fn(&mut App),
}

pub struct App {
    stack: Vec<Page>,
    interupt: Option<Interupt>,
    interupt_args: InteruptArgs,
    store: saved::Store,

    rows: usize,
    cols: usize,
    highlighted: usize,
    widgets: Vec<WidgetFn>,
    code: Code,
    cat_image: DynamicImage,
    dog_image: DynamicImage,
    sunrise_image: DynamicImage,
    picker: Picker,
    image_protocol: StatefulProtocol,
}

impl App {
    fn default() -> Self {
        let dog_image = image::ImageReader::open("dog.jpg")
            .unwrap()
            .decode()
            .unwrap();

        let cat_image = image::ImageReader::open("cat.jpg")
            .unwrap()
            .decode()
            .unwrap();

        let sunrise_image = image::ImageReader::open("sunrise.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let picker = Picker::from_query_stdio().unwrap();
        let image_protocol = picker.new_resize_protocol(cat_image.clone());

        let store = util::saved::init_db();
        //store.add_person("John");
        //store.add_person("Bill");

        Self {
            stack: Vec::new(),
            interupt: None,
            interupt_args: InteruptArgs::default(),
            store,
            rows: 2,
            cols: 1,
            highlighted: 0,
            widgets: vec![
                richbutton::action_button("Secret Login", "Requires a password"),
                richbutton::action_button("Some Random Thing", "Does this thing"),
            ],
            code: Code::new(),
            cat_image,
            dog_image,
            sunrise_image,
            picker,
            image_protocol,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.image_protocol = self.picker.new_resize_protocol(self.sunrise_image.clone());

        self.interupt = Some(popup::boot::interupt());

        loop {
            if let Some(interupt) = self.interupt.take() {
                terminal.draw(|frame| self.draw_interupt(interupt.render, frame))?;
                (interupt.callback)(self);
                // Drain any input events that were buffered during the callback
                while event::poll(std::time::Duration::ZERO)? {
                    let _ = event::read();
                }
            }

            terminal.draw(|frame| self.draw(frame))?;
            self.prop_input();
        }
    }

    fn draw_interupt(&mut self, render: fn(&mut App, Rect, &mut Buffer), frame: &mut Frame) {
        let block = Block::bordered()
            .border_set(border::DOUBLE)
            .title(Line::from(" Sunrise IV Landline ".bold().yellow()).centered())
            .title_bottom(
                Line::from(vec![" Exit Page With ".blue(), "<*> ".blue().bold()]).centered(),
            );

        render(self, block.inner(frame.area()), frame.buffer_mut());

        block.render(frame.area(), frame.buffer_mut());
    }

    fn draw(&mut self, frame: &mut Frame) {
        let block = Block::bordered()
            .border_set(border::DOUBLE)
            .title(Line::from(" Sunrise IV Landline ".bold().yellow()).centered())
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

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let grid = Grid::new(self.rows, self.cols, self.highlighted);
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        grid.render(layout[0], buf, &self.widgets);

        util::render_centered_image(
            &self.sunrise_image,
            &mut self.image_protocol,
            layout[1],
            buf,
        );
    }

    fn on_load(&mut self) {
        self.image_protocol = self.picker.new_resize_protocol(self.sunrise_image.clone());
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
            if self.stack.is_empty() {
                process::exit(0);
            } else {
                _ = self.stack.pop();

                // Jank in the main pages
                // on_load function
                if self.stack.is_empty() {
                    self.on_load();
                }
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
                PageSignal::Push(p) => {
                    if let Some(on_load) = p.on_load {
                        on_load(self);
                    }
                    self.stack.push(p);
                }
                PageSignal::Back => {
                    _ = self.stack.pop();
                }
                PageSignal::Interupt(i) => {
                    self.interupt = Some(i);
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

            KeyCode::Enter => {
                if self.highlighted == 0 {
                    return Some(PageSignal::Push(secret::page()));
                }

                if self.highlighted == 1 {
                    self.interupt = Some(util::web::interupt());
                    return None;
                }
            }

            _ => (),
        }

        None
    }

    // Manually push a page if needed in interupt
    pub fn push_page(&mut self, page: Page) {
        self.stack.push(page);
    }
}

fn main() -> io::Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
