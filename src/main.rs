mod snippets;
mod app;
mod ui;

use std::io;
use std::panic;
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{App, Screen};
use ui::draw;

fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
}

fn main() -> io::Result<()> {
    // Restore terminal even on panic
    panic::set_hook(Box::new(|info| {
        restore_terminal();
        eprintln!("{}", info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Note: no EnableMouseCapture — we don't need it and it causes
    // garbage characters to appear when hovering after exit
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run(&mut terminal, &mut app);

    restore_terminal();
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Global quit
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(());
                }

                match app.screen {
                    Screen::Menu => handle_menu(app, key.code),
                    Screen::Typing => handle_typing(app, key.code),
                    Screen::Results => handle_results(app, key.code),
                }
                if app.should_quit {
                    return Ok(());
                }
            }
        }

        // Tick for WPM updates while typing
        if app.screen == Screen::Typing {
            app.tick();
        }
    }
}

fn handle_menu(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up | KeyCode::Char('k') => app.menu_prev(),
        KeyCode::Down | KeyCode::Char('j') => app.menu_next(),
        KeyCode::Enter | KeyCode::Char(' ') => app.start_session(),
        KeyCode::Char('q') => app.quit(),
        _ => {}
    }
}

fn handle_typing(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.go_to_menu(),
        KeyCode::Backspace => app.backspace(),
        KeyCode::Char(c) => app.type_char(c),
        _ => {}
    }
}

fn handle_results(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Char('r') => app.restart_session(),
        KeyCode::Char('n') => app.new_snippet(),
        KeyCode::Esc | KeyCode::Char('q') => app.go_to_menu(),
        _ => {}
    }
}
