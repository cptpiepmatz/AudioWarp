use ratatui::widgets::Paragraph;
use crossterm::{event, execute};
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use ratatui::backend::CrosstermBackend;
use crate::Terminal;

fn setup_terminal() -> anyhow::Result<Terminal> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World");
            frame.render_widget(greeting, frame.size());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }

    Ok(())
}
