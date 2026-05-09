use crossterm::event::{Event, KeyCode, KeyEventKind};

pub enum InputEvent {
    Left,
    Right,
    Up,
    Down,
    Char(char),
    Backspace,
    Enter,
}

impl InputEvent {
    pub fn match_event(event: Event) -> Option<Self> {
        if let Event::Key(k) = event
            && k.kind == KeyEventKind::Press
        {
            match k.code {
                KeyCode::Left => Some(Self::Left),
                KeyCode::Right => Some(Self::Right),
                KeyCode::Up => Some(Self::Up),
                KeyCode::Down => Some(Self::Down),
                KeyCode::Backspace => Some(Self::Backspace),
                KeyCode::Enter => Some(Self::Enter),
                KeyCode::Char(c) => Some(Self::Char(c)),

                _ => None,
            }
        } else {
            None
        }
    }
}
