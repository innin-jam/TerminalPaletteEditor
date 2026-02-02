use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::style::Color,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::app::App;

struct Grid<'a> {
    cel_width: u16,
    cel_height: u16,
    app: &'a App,
}

// TODO: removed Grid.{cols, rows}; vvv should instead use let (cols, rows) = (app.get_cols, app.get_rows)
impl Widget for Grid<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(self.cel_width));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(self.cel_height));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            if let Some(color) = self.app.get_color_at(i) {
                let swatch = Paragraph::new(format!("{}", color.to_hex()))
                    .alignment(Alignment::Center)
                    .bg(Color::Rgb {
                        r: color.r,
                        g: color.g,
                        b: color.b,
                    });

                if self.app.get_cursor() == i {
                    swatch
                        .block(
                            Block::bordered()
                                .border_type(BorderType::Thick)
                                .padding(Padding::new(0, 0, cell.height / 2, 0)),
                        )
                        .render(cell, buf);
                } else {
                    swatch
                        .block(Block::default().padding(Padding::new(0, 0, cell.height / 2, 0)))
                        .render(cell, buf);
                }
            }
        }
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let (cols, rows) = app.get_dimensions();
    let grid = Grid {
        cols,
        rows,
        cel_width: 10,
        cel_height: 4,
        app,
    };

    // TODO: got rid of cols, should instead use app.get_rows(), app.get_cols()
    let centered = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Min(grid.cel_width * grid.cols as u16),
        Constraint::Min(0),
    ])
    .split(
        Layout::vertical([
            Constraint::Min(0),
            Constraint::Min(grid.cel_height * grid.rows as u16),
            Constraint::Min(0),
        ])
        .split(area)[1],
    );
    frame.render_widget(Block::new().bg(Color::Rgb { r: 0, b: 0, g: 0 }), area);
    frame.render_widget(grid, centered[1]);
}
