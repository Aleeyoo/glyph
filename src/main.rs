mod types;
mod buffer;
mod display;
mod input;
mod editor;

use editor::Editor;
use display::tui;
use input::getkey;
use crossterm::terminal::size;

fn main() -> tui::TuiResult<()> {
    let (cols, rows) = size()?;
    let mut terminal = tui::init()?;
    let mut editor = Editor::new(rows as usize, cols as usize);

    terminal.draw(|f| {
        let _ = f.size();
    })?;

    while editor.running {
        let kc = getkey::getkey();
        if kc == getkey::K_CTRL_C || kc == getkey::K_ESC {
            editor.running = false;
        }
        terminal.draw(|f| {
            let _ = f.size();
        })?;
    }

    tui::cleanup()?;
    Ok(())
}
