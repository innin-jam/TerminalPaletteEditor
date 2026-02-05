use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::style::Color,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::app::App;

struct Grid<'a> {
    cols: usize,
    rows: usize,
    app: &'a App,
}

impl Widget for Grid<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(9));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            Paragraph::new(format!("Area {:02}", i + 1))
                .alignment(Alignment::Center)
                .block(
                    Block::new()
                        .bg(self.app.get_colour_at_cursor())
                        .padding(Padding::new(0, 0, cell.height / 2, 0)),
                )
                .render(cell, buf);
        }
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let (cols, rows) = app.get_dimensions();
    let grid = Grid { cols, rows };

    frame.render_widget(grid, area);
}
