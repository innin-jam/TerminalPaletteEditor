use color::{Lab, OpaqueColor};
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
        let col_constraints = (0..self.app.cols()).map(|_| Constraint::Length(self.cel_width));
        let row_constraints = (0..self.app.rows()).map(|_| Constraint::Length(self.cel_height));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let is_on_cursor = self.app.cursor() == i;
            let mut label = "".to_string();
            let mut color = Color::Reset;
            let mut fg_color = Color::Reset;

            if is_on_cursor && let app::Mode::Insert(contents) = self.app.mode() {
                label = contents.clone();
                label.push('▏');
                if let Ok((r, g, b)) =
                    crate::app::Color::try_from_hex_str(&contents).map(|c| c.rgb())
                {
                    color = Color::Rgb { r, g, b };
                    fg_color = find_foreground_color(r, g, b);
                }
            } else {
                if let Ok(rgb) = self.app.color_at(i) {
                    label = rgb.hex();
                    let (r, g, b) = rgb.rgb();
                    color = Color::Rgb { r, g, b };
                    fg_color = find_foreground_color(r, g, b);
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
            .fg(fg_color)
            .render(cell, buf);
        }
    }
}

fn find_foreground_color(r: u8, g: u8, b: u8) -> Color {
    let color: OpaqueColor<Lab> = OpaqueColor::from_rgb8(r, g, b).convert();
    let diff = 0.3
        * if color.relative_luminance() > 0.5 {
            -1.
        } else {
            1.
        };
    let color = color.map_lightness(|l| l + diff).to_rgba8();
    Color::Rgb {
        r: color.r,
        g: color.g,
        b: color.b,
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
        Constraint::Min(grid.cel_width * app.cols() as u16),
        Constraint::Min(0),
    ])
    .split(
        Layout::vertical([
            Constraint::Min(0),
            Constraint::Min(grid.cel_height * app.rows() as u16),
            Constraint::Min(0),
        ])
        .split(area)[1],
    );
    frame.render_widget(Block::new().bg(Color::Rgb { r: 0, b: 0, g: 0 }), area);
    frame.render_widget(grid, centered[1]);
}
