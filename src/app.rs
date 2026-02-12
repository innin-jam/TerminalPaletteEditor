pub use crate::app::color::Color;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use eyre::{Result, eyre};
use ratatui::crossterm::event::{KeyCode, KeyModifiers};

mod color;

pub struct App {
    grid: Vec<Color>,
    cols: usize,
    rows: usize,
    running: bool,
    cursor: usize,
    mode: Mode,
    leader_mode: Option<LeaderMode>,
    register: Option<Color>,
    multiplier: i32,
    action: Option<Action>,
}

pub enum Mode {
    Normal,
    Insert(String),
    Color,
}

pub enum LeaderMode {
    Space,
}

enum Action {
    AppendMode,
    ColorMode,
    Delete,
    InsertAtEnd,
    InsertAtSart,
    InsertMode,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    NormalMode,
    InsertConfirm,
    InsertAppendChar(char),
    InsertDeleteChar,
    InsertClear,
    PasteAfter,
    PasteBefore,
    PasteClipboardAfter,
    PasteClipboardBefore,
    Quit,
    Replace,
    SpaceLeaderMode,
    Yank,
    YankToClipboard,
    ColorAddRed,
    ColorAddGreen,
    ColorAddBlue,
    ColorRemoveRed,
    ColorRemoveGreen,
    ColorRemoveBlue,
}

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
            action: None,
        }
    }

    // TODO: Move logic from main to inside App
    // TODO: Create handle_events function, called by main
    // TODO: Move all keybinds into handle_events

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    // TODO: if we wanted to add selection functionality, this would have to output an enum{single, selection(start, end)} or simply a selection(start, end)
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn color_at(&self, index: usize) -> Result<Color> {
        if index < self.grid.len() {
            Ok(self.grid[index].clone())
        } else {
            Err(eyre!("Failed to get color at {index}; out of range"))
        }
    }

    fn mut_color_at(&mut self, index: usize) -> Result<&mut Color> {
        if index < self.grid.len() {
            Ok(&mut self.grid[index])
        } else {
            Err(eyre!("Failed to get mut color at {index}; out of range"))
        }
    }

    fn set_color_at(&mut self, color: Color, index: usize) -> Result<Color> {
        let out = self.color_at(index);
        if index < self.grid.len() {
            self.grid[index] = color;
            return out;
        } else {
            Err(eyre!("Can't set color outside grid"))
        }
    }

    fn insert_color_at(&mut self, color: Color, index: usize) -> Result<()> {
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

    fn delete_color_at(&mut self, index: usize) -> Result<Color> {
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

    fn handle_action(&mut self) {
        let Some(ref action) = self.action else {
            return;
        };
        match action {
            Action::AppendMode => self.append_mode(),
            Action::ColorMode => self.color_leader_mode(),
            Action::Delete => self.delete(),
            Action::InsertAtEnd => self.insert_at_end(),
            Action::InsertAtSart => self.insert_at_start(),
            Action::InsertMode => self.insert_mode(),
            Action::MoveDown => self.move_cursor(0, 1),
            Action::MoveLeft => self.move_cursor(-1, 0),
            Action::MoveRight => self.move_cursor(1, 0),
            Action::MoveUp => self.move_cursor(0, -1),
            Action::NormalMode => self.normal_mode(),
            Action::InsertConfirm => self.insert_confirm(),
            Action::InsertAppendChar(char) => self.insert_append_char(c),
            Action::InsertDeleteChar => self.insert_delete_char(),
            Action::InsertClear => self.insert_clear_chars(),
            Action::PasteAfter => self.paste_after(),
            Action::PasteBefore => self.paste_before(),
            Action::PasteClipboardAfter => self.paste_clipboard_after(),
            Action::PasteClipboardBefore => self.paste_clipboard_before(),
            Action::Quit => self.quit(),
            Action::Replace => self.replace(),
            Action::SpaceLeaderMode => self.space_leader_mode(),
            Action::Yank => self.yank(),
            Action::YankToClipboard => self.yank_to_clipboard(),
            Action::ColorAddRed => self.operate_on_color(|color, m| color.add_red(m)),
            Action::ColorAddGreen => self.operate_on_color(|color, m| color.add_green(m)),
            Action::ColorAddBlue => self.operate_on_color(|color, m| color.add_blue(m)),
            Action::ColorRemoveRed => self.operate_on_color(|color, m| color.add_red(-m)),
            Action::ColorRemoveGreen => self.operate_on_color(|color, m| color.add_green(-m)),
            Action::ColorRemoveBlue => self.operate_on_color(|color, m| color.add_blue(-m)),
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
        self.register = self.delete_color_at(self.cursor()).ok();
        self.cursor = self.cursor().min(self.grid.len() - 1)
    }

    pub fn yank(&mut self) {
        self.register = self.color_at(self.cursor()).ok()
    }

    pub fn paste_after(&mut self) {
        if let Some(color) = self.register {
            let _ = self.insert_color_at(color, self.cursor() + 1);
        }
    }

    pub fn paste_before(&mut self) {
        if let Some(color) = self.register {
            let _ = self.insert_color_at(color, self.cursor());
        }
    }

    pub fn replace(&mut self) {
        if let Some(color) = self.register {
            let _ = self.set_color_at(color, self.cursor());
        }
    }

    pub fn insert_mode(&mut self) {
        if let Ok(color) = self.color_at(self.cursor()) {
            self.mode = Mode::Insert(color.hex());
        }
    }

    pub fn append_mode(&mut self) {
        let success = self.insert_color_at(Color::default(), self.cursor() + 1);
        match success {
            Ok(_) => {
                self.cursor = (self.cursor + 1).min(self.grid.len() - 1);
                self.insert_mode();
            }
            Err(_) => {}
        }
    }

    pub fn insert_at_end(&mut self) {
        let success = self.insert_color_at(Color::default(), self.grid.len());
        match success {
            Ok(_) => {
                self.cursor = self.grid.len() - 1;
                self.insert_mode();
            }
            Err(_) => {}
        }
    }

    pub fn insert_at_start(&mut self) {
        let success = self.insert_color_at(Color::default(), 0);
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
                let _ = self.set_color_at(color, self.cursor());
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
        if let Ok(color) = self.color_at(self.cursor()) {
            todo!("Yanked {} to clipboard", color.hex());
        }
    }

    pub fn paste_clipboard_before(&mut self) {
        if let Ok(color) = try_color_from_clipboard() {
            let _ = self.insert_color_at(color, self.cursor());
        }
    }

    pub fn paste_clipboard_after(&mut self) {
        if let Ok(color) = try_color_from_clipboard() {
            let _ = self.insert_color_at(color, self.cursor() + 1);
        }
    }

    pub fn replace_clipboard(&mut self) {
        if let Ok(color) = try_color_from_clipboard() {
            let _ = self.set_color_at(color, self.cursor());
        }
    }

    // --- Color Leader Mode ---
    pub fn color_leader_mode(&mut self) {
        self.mode = Mode::Color;
    }

    pub fn inc_color_multiplier(&mut self) {
        self.multiplier = self.multiplier.saturating_mul(4).min(64);
    }

    pub fn dec_color_multiplier(&mut self) {
        self.multiplier = self.multiplier.saturating_div(4).max(1);
    }

    pub fn operate_on_color<F>(&mut self, f: F)
    where
        F: Fn(&mut Color, i32),
    {
        let m = self.multiplier;
        if let Ok(color) = self.mut_color_at(self.cursor()) {
            f(color, m);
        }
    }

    pub fn handle_events(&mut self, key_code: KeyCode, key_modifiers: KeyModifiers) {
        if let Some(leader_mode) = self.get_leader_mode() {
            match leader_mode {
                LeaderMode::Space => {
                    self.space_leader_mode_keymap(key_code, key_modifiers);
                }
            }
            self.clear_leader_mode();
            return;
        }
        match self.mode() {
            Mode::Normal => self.normal_mode_keymap(key_modifiers, key_code),
            Mode::Insert(_) => self.insert_mode_keymap(key_code, key_modifiers),
            Mode::Color => self.color_mode_keymap(key_code, key_modifiers),
        }
    }

    fn normal_mode_keymap(&mut self, key_modifiers: KeyModifiers, key_code: KeyCode) {
        match (key_modifiers, key_code) {
            (KeyModifiers::SHIFT, key_code) => match key_code {
                KeyCode::Char('P') => self.paste_before(),
                KeyCode::Char('A') => self.insert_at_end(),
                KeyCode::Char('I') => self.insert_at_start(),
                KeyCode::Char('R') => self.replace(),
                _ => {}
            },
            (KeyModifiers::NONE, key_code) => match key_code {
                KeyCode::Char('c') => self.color_leader_mode(),
                KeyCode::Char('i') => self.insert_mode(),
                KeyCode::Char('a') => self.append_mode(),
                KeyCode::Char('d') => self.delete(),
                KeyCode::Char('p') => self.paste_after(),
                KeyCode::Char('y') => self.yank(),
                KeyCode::Char(' ') => self.space_leader_mode(),
                KeyCode::Left | KeyCode::Char('h') => self.move_cursor(-1, 0),
                KeyCode::Down | KeyCode::Char('j') => self.move_cursor(0, 1),
                KeyCode::Up | KeyCode::Char('k') => self.move_cursor(0, -1),
                KeyCode::Right | KeyCode::Char('l') => self.move_cursor(1, 0),
                _ => {}
            },
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                self.quit();
            }
            _ => {}
        }
    }

    fn insert_mode_keymap(&mut self, key_code: KeyCode, key_modifiers: KeyModifiers) {
        match (key_modifiers, key_code) {
            (KeyModifiers::CONTROL, key_code) => match key_code {
                KeyCode::Char('w') => self.insert_clear_chars(),
                _ => {}
            },
            (KeyModifiers::NONE, key_code) => match key_code {
                KeyCode::Enter => self.insert_confirm(),
                KeyCode::Esc => self.normal_mode(),
                KeyCode::Char(c) if "012345678293abcdef".contains(c) => self.insert_append_char(c),
                KeyCode::Backspace => self.insert_delete_char(),
                _ => {}
            },
            _ => {}
        }
    }

    fn space_leader_mode_keymap(&mut self, key_code: KeyCode, key_modifiers: KeyModifiers) {
        match (key_modifiers, key_code) {
            (KeyModifiers::SHIFT, KeyCode::Char('P')) => self.paste_clipboard_before(),
            (KeyModifiers::SHIFT, KeyCode::Char('R')) => self.replace_clipboard(),
            (KeyModifiers::NONE, KeyCode::Char('p')) => self.paste_clipboard_after(),
            (KeyModifiers::NONE, KeyCode::Char('y')) => self.yank_to_clipboard(),
            _ => {}
        }
    }

    fn color_mode_keymap(&mut self, key_code: KeyCode, key_modifiers: KeyModifiers) {
        match (key_modifiers, key_code) {
            (KeyModifiers::CONTROL, KeyCode::Char('a')) => self.inc_color_multiplier(),
            (KeyModifiers::CONTROL, KeyCode::Char('x')) => self.dec_color_multiplier(),
            (KeyModifiers::NONE, KeyCode::Char('r')) => {
                self.operate_on_color(|color, m| color.add_red(m))
            }
            (KeyModifiers::SHIFT, KeyCode::Char('R')) => {
                self.operate_on_color(|color, m| color.add_red(-m))
            }
            (KeyModifiers::NONE, KeyCode::Char('g')) => {
                self.operate_on_color(|color, m| color.add_green(m))
            }
            (KeyModifiers::SHIFT, KeyCode::Char('G')) => {
                self.operate_on_color(|color, m| color.add_green(-m))
            }
            (KeyModifiers::NONE, KeyCode::Char('b')) => {
                self.operate_on_color(|color, m| color.add_blue(m))
            }
            (KeyModifiers::SHIFT, KeyCode::Char('B')) => {
                self.operate_on_color(|color, m| color.add_blue(-m))
            }
            (KeyModifiers::NONE, KeyCode::Esc) => {
                self.normal_mode();
            }
            _ => {}
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
