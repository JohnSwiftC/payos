use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::Block;

pub const AMBER: Color = Color::Rgb(0xE8, 0x9F, 0x3B);
pub const ROSE: Color = Color::Rgb(0xC8, 0x5A, 0x7A);
pub const CREAM: Color = Color::Rgb(0xEA, 0xD8, 0xB6);
pub const ASH: Color = Color::Rgb(0x8C, 0x83, 0x91);
pub const SLATE: Color = Color::Rgb(0x4F, 0x47, 0x55);
pub const TEAL: Color = Color::Rgb(0x5E, 0x9C, 0x9C);
pub const RUST: Color = Color::Rgb(0xC9, 0x46, 0x36);

pub const PRIMARY: Color = AMBER; // titles, bullets, accent text
pub const SECONDARY: Color = ROSE; // alt accent (open slot)
pub const TEXT: Color = CREAM; // body / primary text
pub const TEXT_DIM: Color = ASH; // descriptions, comments, hint context
pub const BORDER: Color = SLATE; // default chrome / dividers
pub const BORDER_ACTIVE: Color = AMBER; // highlighted / active borders
pub const HINT_KEY: Color = CREAM; // <*>, <enter>, etc.
pub const INFO: Color = TEAL; // status/info banners
pub const ALERT: Color = RUST; // error / denied
pub const SUCCESS: Color = AMBER; // granted / ok
pub const ON_ALERT: Color = CREAM; // text drawn on top of ALERT fill
pub const ON_SUCCESS: Color = CREAM; // text drawn on top of SUCCESS fill

pub const CHASSIS: border::Set = border::Set {
    top_left: "┏",
    top_right: "┓",
    bottom_left: "┗",
    bottom_right: "┛",
    vertical_left: "┃",
    vertical_right: "┃",
    horizontal_top: "━",
    horizontal_bottom: "━",
};

pub const BRACKET: border::Set = border::Set {
    top_left: "╔",
    top_right: "╗",
    bottom_left: "╚",
    bottom_right: "╝",
    vertical_left: "║",
    vertical_right: "║",
    horizontal_top: "═",
    horizontal_bottom: "═",
};

pub const DOTTED: border::Set = border::Set {
    top_left: "·",
    top_right: "·",
    bottom_left: "·",
    bottom_right: "·",
    vertical_left: "╎",
    vertical_right: "╎",
    horizontal_top: "╌",
    horizontal_bottom: "╌",
};

pub fn chassis_block() -> Block<'static> {
    let title = Line::from(vec![
        "━┥ ".fg(BORDER),
        "SUNRISE IV LANDLINE".fg(PRIMARY).bold(),
        " ┝━".fg(BORDER),
    ])
    .centered();

    let hint = Line::from(vec![
        "━┥ exit page ".fg(BORDER),
        "<*>".fg(HINT_KEY).bold(),
        " ┝━".fg(BORDER),
    ])
    .centered();

    Block::bordered()
        .border_set(CHASSIS)
        .border_style(Style::new().fg(BORDER))
        .title(title)
        .title_bottom(hint)
}
