use ratatui::{
    border,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

pub struct Grid {
    cols: usize,
    rows: usize,
    highlighted: usize,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cols,
            rows,
            highlighted: 0,
        }
    }
}

impl Widget for Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Ratio(1, self.cols as u32));
        let row_constraints = (0..self.rows).map(|_| Constraint::Ratio(1, self.rows as u32));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let block = if i == self.highlighted {
                Block::bordered().border_set(border::ROUNDED).blue()
            } else {
                Block::bordered().border_set(border::ROUNDED).white()
            };

            Paragraph::new(format!("Area {:02}", i + 1))
                .block(block)
                .render(cell, buf);
        }
    }
}
