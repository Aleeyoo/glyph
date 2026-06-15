//! Window management: split, other-window, delete-window, etc.

use crate::editor::{Editor, CmdResult, Flags};

/// split-window-vertically (C-x 2)
pub fn split_window(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let h = ed.active_window().height / 2;
    let new_h = h.max(3);
    if ed.frame.windows.len() < 8 {
        let id = ed.frame.windows.len();
        let mut new_win = crate::display::window::Window::new(id, ed.cur_buffer, new_h);
        new_win.dot = ed.active_window().dot;
        ed.frame.windows.push(new_win);
        ed.frame.active_window = id;
        ed.cur_window = id;
    }
    Ok(())
}

/// delete-window (C-x 0)
pub fn delete_window(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    if ed.frame.windows.len() > 1 {
        ed.frame.windows.remove(ed.cur_window);
        if ed.cur_window >= ed.frame.windows.len() {
            ed.cur_window = ed.frame.windows.len() - 1;
        }
    }
    Ok(())
}

/// other-window (C-x o)
pub fn other_window(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let n = ed.frame.windows.len();
    if n > 1 {
        ed.cur_window = (ed.cur_window + 1) % n;
    }
    Ok(())
}

/// enlarge-window (C-x ^)
pub fn enlarge_window(ed: &mut Editor, _f: Flags, n: i32) -> CmdResult {
    let h = ed.active_window().height;
    let new_h = (h as i32 + n).max(3) as usize;
    ed.frame.windows[ed.cur_window].height = new_h;
    Ok(())
}
