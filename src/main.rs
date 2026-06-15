mod types;
mod buffer;
mod display;
mod input;
mod command;
mod util;
mod editor;

use editor::Editor;
use display::tui;
use input::getkey::{self, KCode};
use crossterm::terminal::size;
use ratatui::layout::Rect;

fn register_commands(ed: &mut Editor) {
    let _ = ed;
}

fn main() -> tui::TuiResult<()> {
    let (cols, rows) = size()?;
    let mut terminal = tui::init()?;
    let mut editor = Editor::new(rows as usize, cols as usize);
    register_commands(&mut editor);

    terminal.draw(|f| {
        let rect = f.size();
        let modeline_area = Rect {
            x: 0,
            y: rect.height.saturating_sub(1),
            width: rect.width,
            height: 1,
        };
        display::modeline::draw(&editor, f, modeline_area);
    })?;

    while editor.running {
        let kc = getkey::getkey();

        match kc {
            getkey::K_CTRL_X => {
                let _chord = getkey::getkey();
                editor.running = false;
            }
            getkey::K_ESC => {
                editor.running = false;
            }
            c if c >= 32 && c <= 126 => {
                let _ = command::edit::self_insert(&mut editor, Default::default(), c as i32);
            }
            10 | 13 => {
                let _ = command::edit::newline(&mut editor, Default::default(), 1);
            }
            0x7f => {
                let _ = command::edit::delete_backward_char(&mut editor, Default::default(), 1);
            }
            9 => {
                let _ = command::edit::self_insert(&mut editor, Default::default(), ' ' as i32);
                let _ = command::edit::self_insert(&mut editor, Default::default(), ' ' as i32);
            }
            _ => {}
        }

        terminal.draw(|f| {
            let rect = f.size();
            let modeline_area = Rect {
                x: 0,
                y: rect.height.saturating_sub(1),
                width: rect.width,
                height: 1,
            };
            display::modeline::draw(&editor, f, modeline_area);
        })?;
    }

    tui::cleanup()?;
    Ok(())
}
