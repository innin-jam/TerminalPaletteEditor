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
    ColorAddBlue,
    ColorAddGreen,
    ColorAddHue,
    ColorAddLightness,
    ColorAddRed,
    ColorMode,
    ColorRemoveBlue,
    ColorRemoveGreen,
    ColorRemoveHue,
    ColorRemoveLightness,
    ColorRemoveRed,
    DecreaseMultiplier,
    Delete,
    IncreaseMultiplier,
    InsertAppendChar(char),
    InsertAtEnd,
    InsertAtStart,
    InsertClear,
    InsertConfirm,
    InsertDeleteChar,
    InsertMode,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    Noop,
    NormalMode,
    PasteAfter,
    PasteBefore,
    PasteClipboardAfter,
    PasteClipboardBefore,
    Quit,
    Replace,
    ReplaceClipboard,
    SpaceLeaderMode,
    Yank,
    YankToClipboard,
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
        }
    }

    pub fn handle_events(&mut self, key_code: KeyCode, key_modifiers: KeyModifiers) {
        if let Some(action) = self.handle_input(key_code, key_modifiers) {
            self.leader_mode = None;
            self.handle_action(action);
        }
    }

    fn handle_input(&self, key_code: KeyCode, key_modifiers: KeyModifiers) -> Option<Action> {
        let ctrl = matches!(key_modifiers, KeyModifiers::CONTROL);

        if matches!(key_code, KeyCode::Char('c')) && ctrl {
            return Some(Action::Quit);
        }

        if let Some(leader_mode) = self.leader_mode() {
            let leader_action = Some(match leader_mode {
                LeaderMode::Space => match key_code {
                    KeyCode::Esc => Action::Noop, // Clears leader, leader is cleared on `Some(action)`
                    KeyCode::Char('P') => Action::PasteClipboardBefore,
                    KeyCode::Char('R') => Action::ReplaceClipboard,
                    KeyCode::Char('p') => Action::PasteClipboardAfter,
                    KeyCode::Char('y') => Action::YankToClipboard,
                    _ => return None,
                },
            });
            return leader_action;
        }

        Some(match self.mode {
            Mode::Normal => match key_code {
                KeyCode::Char('h') | KeyCode::Left => Action::MoveLeft,
                KeyCode::Char('j') | KeyCode::Down => Action::MoveDown,
                KeyCode::Char('k') | KeyCode::Up => Action::MoveUp,
                KeyCode::Char('l') | KeyCode::Right => Action::MoveRight,
                KeyCode::Char('y') => Action::Yank,
                KeyCode::Char('p') => Action::PasteAfter,
                KeyCode::Char('P') => Action::PasteBefore,
                KeyCode::Char('i') => Action::InsertMode,
                KeyCode::Char('I') => Action::InsertAtStart,
                KeyCode::Char('A') => Action::InsertAtEnd,
                KeyCode::Char('a') => Action::AppendMode,
                KeyCode::Char('R') => Action::Replace,
                KeyCode::Char('c') => Action::ColorMode,
                KeyCode::Char('d') => Action::Delete,
                KeyCode::Char(' ') => Action::SpaceLeaderMode,
                _ => return None,
            },
            Mode::Insert(_) => match key_code {
                KeyCode::Char(c) if "012345678293abcdef".contains(c) => Action::InsertAppendChar(c),
                KeyCode::Backspace => Action::InsertDeleteChar,
                KeyCode::Char('w') if ctrl => Action::InsertClear,
                KeyCode::Enter => Action::InsertConfirm,
                KeyCode::Esc => Action::NormalMode,
                _ => return None,
            },
            Mode::Color => match key_code {
                KeyCode::Char('a') => Action::IncreaseMultiplier,
                KeyCode::Char('x') => Action::DecreaseMultiplier,
                KeyCode::Char('r') => Action::ColorAddRed,
                KeyCode::Char('R') => Action::ColorRemoveRed,
                KeyCode::Char('g') => Action::ColorAddGreen,
                KeyCode::Char('G') => Action::ColorRemoveGreen,
                KeyCode::Char('b') => Action::ColorAddBlue,
                KeyCode::Char('B') => Action::ColorRemoveBlue,
                KeyCode::Char('l') => Action::ColorAddLightness,
                KeyCode::Char('L') => Action::ColorRemoveLightness,
                KeyCode::Char('h') => Action::ColorAddHue,
                KeyCode::Char('H') => Action::ColorRemoveHue,
                KeyCode::Esc => Action::NormalMode,
                _ => return None,
            },
        })
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::AppendMode => {
                let success = self.insert_color_at(Color::default(), self.cursor() + 1);
                match success {
                    Ok(_) => {
                        self.cursor = (self.cursor + 1).min(self.grid.len() - 1);
                        self.insert_mode();
                    }
                    Err(_) => {}
                }
            }

            Action::ColorMode => {
                self.mode = Mode::Color;
            }

            Action::Delete => {
                self.register = self.delete_color_at(self.cursor()).ok();
                self.cursor = self.cursor().min(self.grid.len() - 1)
            }

            Action::DecreaseMultiplier => {
                self.multiplier = self.multiplier.saturating_div(4).max(1);
            }

            Action::InsertAtEnd => {
                let success = self.insert_color_at(Color::default(), self.grid.len());
                match success {
                    Ok(_) => {
                        self.cursor = self.grid.len() - 1;
                        self.insert_mode();
                    }
                    Err(_) => {}
                }
            }

            Action::InsertAtStart => {
                let success = self.insert_color_at(Color::default(), 0);
                match success {
                    Ok(_) => {
                        self.cursor = 0;
                        self.insert_mode();
                    }
                    Err(_) => {}
                }
            }

            Action::InsertMode => {
                self.insert_mode();
            }

            Action::MoveDown => {
                self.cursor = self
                    .cursor
                    .saturating_add_signed(0 + 1 * self.cols as isize)
                    .min(self.grid.len() - 1);
            }

            Action::MoveLeft => {
                let x = -1;
                self.cursor = self
                    .cursor
                    .saturating_add_signed(x + 0 * self.cols as isize)
                    .min(self.grid.len() - 1);
            }

            Action::MoveRight => {
                self.cursor = self
                    .cursor
                    .saturating_add_signed(1 + 0 * self.cols as isize)
                    .min(self.grid.len() - 1);
            }

            Action::MoveUp => {
                let y = -1;
                self.cursor = self
                    .cursor
                    .saturating_add_signed(0 + y * self.cols as isize)
                    .min(self.grid.len() - 1);
            }

            Action::Noop => {}

            Action::NormalMode => {
                self.mode = Mode::Normal;
            }

            Action::IncreaseMultiplier => {
                self.multiplier = self.multiplier.saturating_mul(4).min(64);
            }

            Action::InsertConfirm => {
                if let Mode::Insert(ref contents) = self.mode {
                    if let Ok(color) = Color::try_from_hex_str(&contents) {
                        let _ = self.set_color_at(color, self.cursor());
                    }
                    self.mode = Mode::Normal;
                }
            }

            Action::InsertAppendChar(c) => {
                if let Mode::Insert(ref mut contents) = self.mode {
                    if contents.len() < 6 {
                        contents.push(c);
                    }
                }
            }

            Action::InsertDeleteChar => {
                if let Mode::Insert(ref mut contents) = self.mode {
                    contents.pop();
                }
            }

            Action::InsertClear => {
                if let Mode::Insert(ref mut contents) = self.mode {
                    contents.clear();
                }
            }

            Action::PasteAfter => {
                if let Some(color) = self.register {
                    let _ = self.insert_color_at(color, self.cursor() + 1);
                }
            }

            Action::PasteBefore => {
                if let Some(color) = self.register {
                    let _ = self.insert_color_at(color, self.cursor());
                }
            }

            Action::PasteClipboardAfter => {
                if let Ok(color) = try_color_from_clipboard() {
                    let _ = self.insert_color_at(color, self.cursor() + 1);
                }
            }

            Action::PasteClipboardBefore => {
                if let Ok(color) = try_color_from_clipboard() {
                    let _ = self.insert_color_at(color, self.cursor());
                }
            }

            Action::Quit => {
                self.running = false;
            }

            Action::Replace => {
                if let Some(color) = self.register {
                    let _ = self.set_color_at(color, self.cursor());
                }
            }

            Action::ReplaceClipboard => {
                if let Ok(color) = try_color_from_clipboard() {
                    let _ = self.set_color_at(color, self.cursor());
                }
            }

            Action::SpaceLeaderMode => {
                self.leader_mode = Some(LeaderMode::Space);
            }

            Action::Yank => self.register = self.color_at(self.cursor()).ok(),
            Action::YankToClipboard => {
                if let Ok(color) = self.color_at(self.cursor()) {
                    todo!("Yanked {} to clipboard", color.hex());
                }
            }

            Action::ColorAddRed => self.operate_on_color(|color, m| color.change_red(m)),
            Action::ColorAddGreen => self.operate_on_color(|color, m| color.change_green(m)),
            Action::ColorAddBlue => self.operate_on_color(|color, m| color.change_blue(m)),
            Action::ColorAddHue => self.operate_on_color(|color, m| color.adjust_hue(m)),
            Action::ColorAddLightness => {
                self.operate_on_color(|color, m| color.adjust_lightness(m))
            }
            Action::ColorRemoveRed => self.operate_on_color(|color, m| color.change_red(-m)),
            Action::ColorRemoveGreen => self.operate_on_color(|color, m| color.change_green(-m)),
            Action::ColorRemoveBlue => self.operate_on_color(|color, m| color.change_blue(-m)),
            Action::ColorRemoveHue => self.operate_on_color(|color, m| color.adjust_hue(-m)),
            Action::ColorRemoveLightness => {
                self.operate_on_color(|color, m| color.adjust_lightness(-m))
            }
        }
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

    pub fn leader_mode(&self) -> &Option<LeaderMode> {
        &self.leader_mode
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

    fn insert_mode(&mut self) {
        if let Ok(color) = self.color_at(self.cursor()) {
            self.mode = Mode::Insert(color.hex());
        }
    }

    fn operate_on_color<F>(&mut self, f: F)
    where
        F: Fn(&mut Color, i32),
    {
        let m = self.multiplier;
        if let Ok(color) = self.mut_color_at(self.cursor()) {
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
