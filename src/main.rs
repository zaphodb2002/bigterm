use anyhow::{Error, Result};
use crossterm::{
    event::{
        self,
        Event::{self, Key},
        KeyCode::{self, Char},
        KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    style::Stylize,
    widgets::*,
};

use std::io::stdout;

fn main() -> Result<(), Error> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    disable_raw_mode()?;
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    f.render_widget(Paragraph::new(format!("TEST {} TEST", app.text)), f.size());
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App {
        text: "This is from the app!".to_string(),
        should_quit: false,
    };

    loop {
        terminal.draw(|f| {
            ui(&app, f);
        })?;

        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

struct App {
    text: String,
    should_quit: bool,
}
