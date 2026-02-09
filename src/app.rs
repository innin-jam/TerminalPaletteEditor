pub use crate::app::color::Color;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use eyre::{Error, Result, eyre};
use ratatui::DefaultTerminal;
// TODO: use eyre errors instead of default

pub struct App {
    grid: Vec<Color>,
    cols: usize,
    rows: usize,
    running: bool,
    cursor: usize,
    mode: Mode,
    leader_mode: Option<LeaderMode>,
    register: Option<Color>,
    multiplier: u8,
}

pub enum Mode {
    Normal,
    Insert(String),
}

pub enum LeaderMode {
    Space,
    Color,
}

mod color;

impl App {
    pub fn new() -> Self {
        App {
            grid: vec![Color::default()],
            cols: 8,
            rows: 8,
            running: true,
            cursor: 0,
            mode: Mode::Normal,
            leader_mode: None,
            register: None,
            multiplier: 64,
        }
    }

    // TODO: Move logic from main to inside App
    // TODO: Create handle_events function, called by main
    // TODO: Move all keybinds into handle_events

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

    pub fn try_get_color_at(&self, index: usize) -> Result<Color> {
        if index < self.grid.len() {
            Ok(self.grid[index].clone())
        } else {
            Err(eyre!("Failed to get color at {index}; out of range"))
        }
    }

    fn try_get_mut_color_at(&mut self, index: usize) -> Result<&mut Color> {
        if index < self.grid.len() {
            Ok(&mut self.grid[index])
        } else {
            Err(eyre!("Failed to get mut color at {index}; out of range"))
        }
    }

    fn set_color_at(&mut self, color: Color, index: usize) -> Result<Color> {
        let out = self.try_get_color_at(index);
        if index < self.grid.len() {
            self.grid[index] = color;
            return out;
        } else {
            Err(eyre!("Can't set color outside grid"))
        }
    }

    fn try_insert_color_at(&mut self, color: Color, index: usize) -> Result<()> {
        if index > self.grid.len() {
            Err(eyre!("Can't insert color outside grid"))
        } else if self.grid.len() >= self.cols * self.rows {
            Err(eyre!(
                "Tried to insert color at {index}, which is out of bounds {}, {}",
                self.cols,
                self.rows
            ))
        } else {
            self.grid.insert(index, color);
            Ok(())
        }
    }

    fn try_delete_color_at(&mut self, index: usize) -> Result<Color> {
        // Ensure a minimum of one color
        if self.grid.len() <= 1 {
            return Err(eyre!("Can't delete final color"));
        }

        if index < self.grid.len() {
            Ok(self.grid.remove(index))
        } else {
            return Err(eyre!("Can't delete color outside grid"));
        }
    }

    // --- Normal Mode ---
    /// Enter normal mode
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
        self.register = self.try_delete_color_at(self.get_cursor()).ok();
        self.cursor = self.get_cursor().min(self.grid.len() - 1)
    }

    pub fn yank(&mut self) {
        self.register = self.try_get_color_at(self.get_cursor()).ok()
    }

    pub fn paste_after(&mut self) {
        if let Some(color) = self.register {
            let _ = self.try_insert_color_at(color, self.get_cursor() + 1);
        }
    }

    pub fn paste_before(&mut self) {
        if let Some(color) = self.register {
            let _ = self.try_insert_color_at(color, self.get_cursor());
        }
    }

    pub fn replace(&mut self) {
        if let Some(color) = self.register {
            let _ = self.set_color_at(color, self.get_cursor());
        }
    }

    pub fn insert_mode(&mut self) {
        match self.mode {
            Mode::Insert(_) => {}
            Mode::Normal => {
                if let Ok(color) = self.try_get_color_at(self.get_cursor()) {
                    self.mode = Mode::Insert(color.to_hex());
                }
            }
        }
    }

    pub fn append_mode(&mut self) {
        let success = self.try_insert_color_at(Color::default(), self.get_cursor() + 1);
        match success {
            Ok(_) => {
                self.cursor = (self.cursor + 1).min(self.grid.len() - 1);
                self.insert_mode();
            }
            Err(_) => {}
        }
    }

    pub fn insert_at_end(&mut self) {
        let success = self.try_insert_color_at(Color::default(), self.grid.len());
        match success {
            Ok(_) => {
                self.cursor = self.grid.len() - 1;
                self.insert_mode();
            }
            Err(_) => {}
        }
    }

    pub fn insert_at_start(&mut self) {
        let success = self.try_insert_color_at(Color::default(), 0);
        match success {
            Ok(_) => {
                self.cursor = 0;
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
                let _ = self.set_color_at(color, self.get_cursor());
            }
            self.mode = Mode::Normal;
        }
    }

    pub fn get_leader_mode(&self) -> &Option<LeaderMode> {
        &self.leader_mode
    }

    pub fn clear_leader_mode(&mut self) {
        self.leader_mode = None;
    }

    // --- Space Leader Mode ---
    pub fn space_leader_mode(&mut self) {
        self.leader_mode = Some(LeaderMode::Space);
    }

    pub fn yank_to_clipboard(&mut self) {
        if let Ok(color) = self.try_get_color_at(self.get_cursor()) {
            todo!("Yanked {} to clipboard", color.to_hex());
        }
    }

    pub fn paste_clipboard_before(&mut self) {
        if let Ok(color) = try_color_from_clipboard() {
            let _ = self.try_insert_color_at(color, self.get_cursor());
        }
    }

    pub fn paste_clipboard_after(&mut self) {
        if let Ok(color) = try_color_from_clipboard() {
            let _ = self.try_insert_color_at(color, self.get_cursor() + 1);
        }
    }

    pub fn replace_clipboard(&mut self) {
        if let Ok(color) = try_color_from_clipboard() {
            let _ = self.set_color_at(color, self.get_cursor());
        }
    }

    // --- Color Leader Mode ---
    pub fn color_leader_mode(&mut self) {
        self.leader_mode = Some(LeaderMode::Color)
    }

    pub fn inc_color_multiplier(&mut self) {
        self.multiplier = self.multiplier.saturating_mul(4).min(64);
    }

    pub fn dec_color_multiplier(&mut self) {
        self.multiplier = self.multiplier.saturating_div(4).max(1);
    }

    pub fn operate_on_color<F>(&mut self, f: F)
    where
        F: Fn(&mut Color, u8),
    {
        let m = self.multiplier;
        if let Ok(color) = self.try_get_mut_color_at(self.get_cursor()) {
            f(color, m);
        }
    }
}

fn try_color_from_clipboard() -> Result<Color> {
    if let Ok(mut clipboard) = ClipboardContext::new()
        && let Ok(contents) = clipboard.get_contents()
    {
        // TODO: allow importing `#RRGGBB` and `(RR, GG, BB)`
        if let Ok(color) = Color::try_from_hex_str(&contents) {
            return Ok(color);
        };

        return Err(eyre!("Failed to parse clipboard contents"));
    };
    Err(eyre!("Can't access system clipboard"))
}
