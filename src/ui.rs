use ratatui::{
    Frame,
    buffer::Buffer,
    crossterm::style::Color,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::app::{self, App};

struct Grid<'a> {
    cel_width: u16,
    cel_height: u16,
    app: &'a App,
}

impl Widget for Grid<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.app.get_cols()).map(|_| Constraint::Length(self.cel_width));
        let row_constraints = (0..self.app.get_rows()).map(|_| Constraint::Length(self.cel_height));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let is_on_cursor = self.app.get_cursor() == i;
            let mut label = "".to_string();
            let mut color = Color::Reset;

            if is_on_cursor && let app::Mode::Insert(contents) = self.app.get_mode() {
                label = contents.clone();
                label.push('▏');
                if let Ok(rgb) = app::Color::try_from_hex_str(&contents) {
                    color = Color::Rgb {
                        r: rgb.r,
                        g: rgb.g,
                        b: rgb.b,
                    };
                }
            } else {
                if let Ok(rgb) = self.app.try_get_color_at(i) {
                    label = rgb.to_hex();
                    color = Color::Rgb {
                        r: rgb.r,
                        g: rgb.g,
                        b: rgb.b,
                    };
                }
            }

            let swatch = Paragraph::new(label);

            if is_on_cursor {
                swatch
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Thick)
                            .padding(Padding::new(1, 0, 0, 0)),
                    )
                    .alignment(Alignment::Left)
            } else {
                swatch
                    .block(Block::default().padding(Padding::new(0, 0, cell.height / 2, 0)))
                    .alignment(Alignment::Center)
            }
            .bg(color)
            .render(cell, buf);
        }
    }
}

pub fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let grid = Grid {
        cel_width: 10,
        cel_height: 3,
        app,
    };

    let centered = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Min(grid.cel_width * app.get_cols() as u16),
        Constraint::Min(0),
    ])
    .split(
        Layout::vertical([
            Constraint::Min(0),
            Constraint::Min(grid.cel_height * app.get_rows() as u16),
            Constraint::Min(0),
        ])
        .split(area)[1],
    );
    frame.render_widget(Block::new().bg(Color::Rgb { r: 0, b: 0, g: 0 }), area);
    frame.render_widget(grid, centered[1]);
}
