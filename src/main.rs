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

use crate::app::{App, Color};
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
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, key_code) => match key_code {
                    KeyCode::Char(' ') => app.set_color_at(
                        Color {
                            r: 0,
                            g: 125,
                            b: 125,
                        },
                        app.get_cursor(),
                    ),
                    KeyCode::Left => app.move_cursor(-1, 0),
                    KeyCode::Down => app.move_cursor(0, 1),
                    KeyCode::Up => app.move_cursor(0, -1),
                    KeyCode::Right => app.move_cursor(1, 0),
                    _ => {}
                },
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    app.stop();
                }
                _ => {}
            }
        }
    }
    Ok(())
}
