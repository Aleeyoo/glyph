mod types;
mod buffer;
mod display;
mod input;
mod editor;

use std::io;
use editor::Editor;
use display::tui;
use input::getkey;

fn main() -> tui::TuiResult<()> {
    // Initialize terminal
    let mut terminal = tui::init()?;
    let mut editor = Editor::new(terminal.size()?.height as usize, terminal.size()?.width as usize);

    // Draw initial blank screen
    terminal.draw(|f| {
        let _ = f.size();
    })?;

    // Event loop
    while editor.running {
        let kc = getkey::getkey();

        // C-x C-c quits
        // ESC quits (simple for now; proper prefix key dispatch later)
        if kc == getkey::K_CTRL_C || kc == getkey::K_ESC {
            editor.running = false;
        }

        // Refresh display each frame
        terminal.draw(|f| {
            let _ = f.size();
        })?;
    }

    // Cleanup
    tui::cleanup()?;
    Ok(())
}
