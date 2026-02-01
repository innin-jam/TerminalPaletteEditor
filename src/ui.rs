use std::arch::x86_64::_mm512_maskz_broadcast_f64x4;

use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::style::Color,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::line::DOUBLE,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
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
    let grid = Grid { cols, rows, app };

    let centered = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Min(7 * 8),
        Constraint::Min(0),
    ])
    .split(
        Layout::vertical([
            Constraint::Min(0),
            Constraint::Min(3 * 8),
            Constraint::Min(0),
        ])
        .split(area)[1],
    );
    frame.render_widget(Block::new().bg(Color::Rgb { r: 0, b: 0, g: 0 }), area);
    frame.render_widget(grid, centered[1]);
}
