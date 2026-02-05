pub struct App {
    grid: Vec<Color>,
    grid_width: usize,
    grid_height: usize,
    running: bool,
    cursor: usize,
}

#[derive(Debug, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl App {
    pub fn new() -> Self {
        App {
            grid: vec![Color(0, 0, 0)],
            grid_width: 8,
            grid_height: 8,
            running: true,
            cursor: 0,
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_grid(&self) -> Vec<Color> {
        self.grid.clone()
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.grid_width, self.grid_height)
    }

    pub fn move_cursor(&mut self, x: isize, y: isize) {
        self.cursor = self
            .cursor
            .saturating_add_signed(x + y * self.grid_width as isize)
            .min(self.grid_width * self.grid_height);
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_color_at(&mut self, color: Color, pos: usize) {
        for _ in 0..(pos.saturating_sub(self.grid.iter().count() - 1)) {
            self.grid.push(Color(0, 0, 0))
        }
        self.grid[pos] = color;
    }
}
