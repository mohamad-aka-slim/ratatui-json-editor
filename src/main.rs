use std::{error::Error, io, panic};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
            KeyModifiers,
        },
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod ui;

use app::{App, CurrentScreen};
use ui::ui;

type Tui = Terminal<CrosstermBackend<io::Stderr>>;

fn main() -> Result<(), Box<dyn Error>> {
    install_panic_hook();

    let mut terminal = init_terminal()?;
    let mut app = App::new();

    let print_json = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    // `run_app` only reports an error after the terminal has been restored.
    if print_json? {
        app.print_json()?;
    }

    Ok(())
}

/// Puts the terminal into raw mode and switches to the alternate screen.
///
/// The UI is drawn to stderr so that stdout stays reserved for the JSON output.
fn init_terminal() -> io::Result<Tui> {
    enable_raw_mode()?;
    execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    Terminal::new(CrosstermBackend::new(io::stderr()))
}

/// Restores the terminal to its original state.
fn restore_terminal(terminal: &mut Tui) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()
}

/// Restores the terminal before the default panic message is printed, otherwise
/// a panic would leave the shell in raw mode with the alternate screen active.
fn install_panic_hook() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture);
        default_hook(info);
    }));
}

/// Runs the event loop and returns whether the collected JSON should be printed.
fn run_app(terminal: &mut Tui, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        // Crossterm reports both press and release events on some terminals.
        if key.kind == KeyEventKind::Release {
            continue;
        }

        if handle_key(app, key) {
            return Ok(true);
        }
    }
}

/// Applies a key press to the app state. Returns `true` when the user confirmed
/// quitting and the collected JSON should be printed.
fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match app.screen() {
        CurrentScreen::Main => match key.code {
            KeyCode::Char('e') => app.begin_editing(),
            KeyCode::Char('q') => app.request_exit(),
            KeyCode::Char('c') if ctrl => app.request_exit(),
            _ => {}
        },
        CurrentScreen::Editing => match key.code {
            KeyCode::Enter => app.submit_editing(),
            KeyCode::Tab => app.toggle_editing(),
            KeyCode::Esc => app.cancel_editing(),
            KeyCode::Backspace => app.delete_char(),
            KeyCode::Char(_) if ctrl => app.cancel_editing(),
            KeyCode::Char(character) => app.push_char(character),
            _ => {}
        },
        CurrentScreen::Exiting => match key.code {
            KeyCode::Char('y' | 'Y') => return true,
            KeyCode::Char('n' | 'N') | KeyCode::Esc => app.cancel_exit(),
            _ => {}
        },
    }

    false
}
