use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub struct App {
    grid: Vec<Color>,
    cols: usize,
    rows: usize,
    running: bool,
    cursor: usize,
    mode: Mode,
    leader_mode: Option<LeaderMode>,
    register: Option<Color>,
}

pub enum Mode {
    Normal,
    Insert(String),
}

pub enum LeaderMode {
    Space,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn try_from_hex_str(hex: &str) -> Result<Self, ()> {
        if hex.len() != 6 {
            return Err(());
        }

        let Ok(r) = u8::from_str_radix(&hex[0..2], 16) else {
            return Err(());
        };
        let Ok(g) = u8::from_str_radix(&hex[2..4], 16) else {
            return Err(());
        };
        let Ok(b) = u8::from_str_radix(&hex[4..6], 16) else {
            return Err(());
        };

        Ok(Self { r, g, b })
    }
}

impl App {
    pub fn new() -> Self {
        App {
            grid: vec![Color::black(); 8 * 8],
            cols: 8,
            rows: 8,
            running: true,
            cursor: 0,
            mode: Mode::Normal,
            leader_mode: None,
            register: None,
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_mode(&self) -> &Mode {
        &self.mode
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn get_color_at(&self, index: usize) -> Option<Color> {
        if index < self.grid.len() {
            Some(self.grid[index].clone())
        } else {
            None
        }
    }

    fn set_color_at(&mut self, color: Color, index: usize) -> Option<Color> {
        let out = self.get_color_at(index);
        if index < self.grid.len() {
            self.grid[index] = color;
            return out;
        } else {
            eprint!("Tried to set color outside grid");
            return None;
        }
    }

    fn insert_color_at(&mut self, color: Color, index: usize) -> Result<(), &str> {
        if index > self.grid.len() {
            Err("Can't insert color outside grid")
        } else if self.grid.len() >= self.cols * self.rows {
            Err("Can't insert color; grid full")
        } else {
            self.grid.insert(index, color);
            Ok(())
        }
    }

    fn delete_color_at(&mut self, index: usize) -> Option<Color> {
        // Ensure a minimum of one color
        if self.grid.len() <= 1 {
            return None;
        }

        if index < self.grid.len() {
            Some(self.grid.remove(index))
        } else {
            eprint!("Tried to delete color outside grid");
            return None;
        }
    }

    pub fn normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn move_cursor(&mut self, x: isize, y: isize) {
        self.cursor = self
            .cursor
            .saturating_add_signed(x + y * self.cols as isize)
            .min(self.grid.len() - 1);
    }

    pub fn delete(&mut self) {
        self.register = self.delete_color_at(self.get_cursor());
        self.cursor = self.get_cursor().min(self.grid.len() - 1)
    }

    pub fn yank(&mut self) {
        self.register = self.get_color_at(self.get_cursor())
    }

    pub fn paste_after(&mut self) {
        if let Some(color) = self.register {
            let _ = self.insert_color_at(color, self.get_cursor() + 1);
        }
    }

    pub fn paste_before(&mut self) {
        if let Some(color) = self.register {
            let _ = self.insert_color_at(color, self.get_cursor());
        }
    }

    pub fn replace(&mut self) {
        if let Some(color) = self.register {
            self.set_color_at(color, self.get_cursor());
        }
    }

    pub fn insert_mode(&mut self) {
        match self.mode {
            Mode::Insert(_) => {}
            Mode::Normal => {
                if let Some(color) = self.get_color_at(self.get_cursor()) {
                    self.mode = Mode::Insert(color.to_hex());
                }
            }
        }
    }

    pub fn append_mode(&mut self) {
        let success = self.insert_color_at(Color::black(), self.get_cursor());
        match success {
            Ok(_) => {
                self.cursor = (self.cursor + 1).min(self.grid.len() - 1);
                self.insert_mode();
            }
            Err(_) => {}
        }
    }

    pub fn insert_at_end(&mut self) {
        let success = self.insert_color_at(Color::black(), self.grid.len());
        match success {
            Ok(_) => {
                self.cursor = self.grid.len() - 1;
                self.insert_mode();
            }
            Err(_) => {}
        }
    }

    pub fn insert_append_char(&mut self, c: char) {
        if let Mode::Insert(ref mut contents) = self.mode {
            if contents.len() < 6 {
                contents.push(c);
            }
        }
    }

    pub fn insert_delete_char(&mut self) {
        if let Mode::Insert(ref mut contents) = self.mode {
            contents.pop();
        }
    }

    pub fn insert_clear_chars(&mut self) {
        if let Mode::Insert(ref mut contents) = self.mode {
            contents.clear();
        }
    }

    pub fn insert_confirm(&mut self) {
        if let Mode::Insert(ref contents) = self.mode {
            if let Ok(color) = Color::try_from_hex_str(&contents) {
                self.set_color_at(color, self.get_cursor());
            }
            self.mode = Mode::Normal;
        }
    }

    // TODO: THATS WHERE I LAST STOPPED WORKING

    pub fn get_leader_mode(&self) -> &Option<LeaderMode> {
        &self.leader_mode
    }

    pub fn space_leader_mode(&mut self) {
        self.leader_mode = Some(LeaderMode::Space);
    }

    pub fn clear_leader_mode(&mut self) {
        self.leader_mode = None;
    }

    fn yank_to_clipboard_at(&mut self, index: usize) {
        let color = self.grid.get(index).unwrap().to_hex();
        todo!("Yanked {color} to clipboard");
    }

    pub fn yank_to_clipboard(&mut self) {
        self.yank_to_clipboard_at(self.get_cursor());
    }

    fn paste_clipboard_at(&mut self, index: usize) {
        // TODO: handle error gracefully
        let clipboard = ClipboardContext::new().unwrap().get_contents().unwrap();
        if let Ok(color) = Color::try_from_hex_str(&clipboard) {
            self.grid.insert(index, color);
        }
    }

    pub fn paste_clipboard_after(&mut self) {
        self.paste_clipboard_at(self.get_cursor() + 1);
    }

    pub fn paste_clipboard_before(&mut self) {
        self.paste_clipboard_at(self.get_cursor());
    }

    fn replace_clipboard_at(&mut self, index: usize) {
        // TODO: handle error gracefully
        let clipboard = ClipboardContext::new().unwrap().get_contents().unwrap();
        if let Ok(color) = Color::try_from_hex_str(&clipboard) {
            self.set_color_at(color, index);
        }
    }

    pub fn replace_clipboard(&mut self) {
        self.replace_clipboard_at(self.get_cursor());
    }
}
