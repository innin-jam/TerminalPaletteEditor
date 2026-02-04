mod app;
mod ui;

use std::io;

use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};

use crate::app::{App, LeaderMode, Mode};
use crate::ui::ui;

fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    while app.is_running() {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if let Some(leader_mode) = app.get_leader_mode() {
                match leader_mode {
                    LeaderMode::Space => {
                        match (key.modifiers, key.code) {
                            (KeyModifiers::SHIFT, KeyCode::Char('P')) => {
                                app.paste_clipboard_before()
                            }
                            (KeyModifiers::SHIFT, KeyCode::Char('R')) => app.replace_clipboard(),
                            (KeyModifiers::NONE, KeyCode::Char('p')) => app.paste_clipboard_after(),
                            (KeyModifiers::NONE, KeyCode::Char('y')) => app.yank_to_clipboard(),
                            _ => {}
                        }
                        app.clear_leader_mode();
                        continue;
                    }
                }
            }
            match app.get_mode() {
                Mode::Insert(_) => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, key_code) => match key_code {
                        KeyCode::Char('w') => app.insert_clear_chars(),
                        _ => {}
                    },
                    (KeyModifiers::NONE, key_code) => match key_code {
                        KeyCode::Enter => app.insert_confirm(),
                        KeyCode::Esc => app.normal_mode(),
                        KeyCode::Char(c) if "012345678293abcdef".contains(c) => {
                            app.insert_append_char(c)
                        }
                        KeyCode::Backspace => app.insert_delete_char(),
                        _ => {}
                    },
                    _ => {}
                },
                Mode::Normal => match (key.modifiers, key.code) {
                    (KeyModifiers::SHIFT, key_code) => match key_code {
                        KeyCode::Char('P') => app.paste_before(),
                        KeyCode::Char('A') => app.insert_at_end(),
                        KeyCode::Char('I') => app.insert_at_start(),
                        KeyCode::Char('R') => app.replace(),
                        _ => {}
                    },
                    (KeyModifiers::NONE, key_code) => match key_code {
                        KeyCode::Char('i') => app.insert_mode(),
                        KeyCode::Char('a') => app.append_mode(),
                        KeyCode::Char('d') => app.delete(),
                        KeyCode::Char('p') => app.paste_after(),
                        KeyCode::Char('y') => app.yank(),
                        KeyCode::Char(' ') => app.space_leader_mode(),
                        KeyCode::Left | KeyCode::Char('h') => app.move_cursor(-1, 0),
                        KeyCode::Down | KeyCode::Char('j') => app.move_cursor(0, 1),
                        KeyCode::Up | KeyCode::Char('k') => app.move_cursor(0, -1),
                        KeyCode::Right | KeyCode::Char('l') => app.move_cursor(1, 0),
                        _ => {}
                    },
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        app.stop();
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
