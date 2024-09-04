mod app;
mod ui;

use std::{error::Error, io};

use app::{App, CurrentScreen};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    //create app
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Tab => match app.current_screen {
                        CurrentScreen::Prompt => app.current_screen = CurrentScreen::Main,
                        CurrentScreen::Main => app.current_screen = CurrentScreen::Prompt,
                    },
                    _ => {}
                },
                CurrentScreen::Prompt => match key.code {
                    KeyCode::Char('y') => {}
                    KeyCode::Char('n') => {}
                    KeyCode::Tab => match app.current_screen {
                        CurrentScreen::Prompt => app.current_screen = CurrentScreen::Main,
                        CurrentScreen::Main => app.current_screen = CurrentScreen::Prompt,
                    },
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
            }
        }
    }
}
