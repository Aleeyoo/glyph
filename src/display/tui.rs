//! Terminal UI initialization and cleanup via crossterm + ratatui.

use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Result alias for TUI operations.
pub type TuiResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Initialize the terminal for TUI mode: raw mode + alternate screen.
pub fn init() -> TuiResult<Terminal<CrosstermBackend<io::Stdout>>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to normal mode.
pub fn cleanup() -> TuiResult<()> {
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
