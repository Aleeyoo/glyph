//! Prefix argument, universal-argument, digit-argument, negative-argument.

use crate::editor::{Editor, CmdResult, Flags};

/// universal-argument (C-u)
/// Start or repeat a prefix argument.
pub fn universal_argument(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.prefix_arg = 4;
    Ok(())
}

/// digit-argument (M-0..M-9)
/// Numeric argument prefix.
pub fn digit_argument(ed: &mut Editor, _f: Flags, n: i32) -> CmdResult {
    if ed.prefix_arg == 0 {
        ed.prefix_arg = n as usize;
    } else {
        ed.prefix_arg = ed.prefix_arg * 10 + n as usize;
    }
    Ok(())
}

/// negative-argument (M--)
pub fn negative_argument(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.prefix_arg = 0;
    ed.prefix_negative = true;
    Ok(())
}

/// keyboard-quit (C-g) — abort current operation
pub fn keyboard_quit(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.prefix_arg = 0;
    ed.prefix_negative = false;
    Err("Quit".into())
}

/// set-mark-command (C-SPC)
pub fn set_mark(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let dot = ed.active_window().dot;
    ed.active_window_and_buffer_mut().0.mark = Some(dot);
    Ok(())
}

/// exchange-point-and-mark (C-x C-x)
pub fn exchange_point_and_mark(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, _) = ed.active_window_and_buffer_mut();
    if let Some(mark) = win.mark {
        let dot = win.dot;
        win.dot = mark;
        win.mark = Some(dot);
    }
    Ok(())
}

/// mark-whole-buffer (C-x h)
pub fn mark_whole_buffer(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let buf_len = ed.active_buffer().text.len();
    let (win, _) = ed.active_window_and_buffer_mut();
    win.mark = Some(win.dot);
    win.dot.pos = buf_len;
    win.dot.col = 0;
    Ok(())
}

/// what-cursor-position (C-x =)
pub fn what_cursor_position(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let pos = ed.active_window().dot.pos;
    let buf_len = ed.active_buffer().text.len();
    ed.echo_line = format!("pos: {} of {}, {}%", pos, buf_len,
        if buf_len > 0 { pos * 100 / buf_len } else { 0 });
    Ok(())
}

/// redraw-display (C-l)
pub fn redraw_display(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, _) = ed.active_window_and_buffer_mut();
    win.top_line = win.dot.line.saturating_sub(win.height / 2);
    Ok(())
}
