pub struct App {
    grid: Vec<Color>,
    grid_width: usize,
    grid_height: usize,
    running: bool,
    cursor: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl App {
    pub fn new() -> Self {
        App {
            grid: [Color { r: 0, g: 0, b: 0 }; 8 * 8].to_vec(),
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
        if pos < self.grid.len() {
            self.grid[pos] = color;
        } else {
            panic!("Tried to edit color outside grid")
        }
    }

    pub fn get_color_at(&self, pos: usize) -> Option<Color> {
        if pos < self.grid.len() {
            Some(self.grid[pos].clone())
        } else {
            None
        }
    }
}
