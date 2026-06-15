//! Basic editing commands: self-insert, newline, delete, cursor movement.

use crate::editor::{Editor, CmdResult, Flags};

/// selfinsert-command / self-insert-char / insert
pub fn self_insert(ed: &mut Editor, _f: Flags, c: i32) -> CmdResult {
    if c > 0 && c < 256 {
        let b = c as u8;
        let (win, buf) = ed.active_window_and_buffer_mut();
        buf.text.insert_at(win.dot.pos, &[b]);
        win.dot.pos += 1;
        win.dot.col += 1;
        ed.set_dirty(true);
    }
    Ok(())
}

/// newline
pub fn newline(ed: &mut Editor, f: Flags, n: i32) -> CmdResult {
    let count = if f.has_arg { n.max(1) } else { 1 };
    for _ in 0..count {
        let (win, buf) = ed.active_window_and_buffer_mut();
        buf.text.insert_at(win.dot.pos, b"\n");
        win.dot.pos += 1;
        win.dot.line += 1;
        win.dot.col = 0;
        ed.set_dirty(true);
    }
    Ok(())
}

/// delete-char (C-d)
pub fn delete_char(ed: &mut Editor, f: Flags, n: i32) -> CmdResult {
    let count = if f.has_arg { n.max(1) } else { 1 };
    let (win, buf) = ed.active_window_and_buffer_mut();
    let text_len = buf.text.len();
    let to_del = count.min((text_len - win.dot.pos) as i32) as usize;
    if to_del > 0 {
        buf.text.delete_at(win.dot.pos, to_del);
        ed.set_dirty(true);
    }
    Ok(())
}

/// delete-backward-char (Backspace/DEL)
pub fn delete_backward_char(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let pos = ed.active_window().dot.pos;
    if pos > 0 {
        ed.active_buffer_mut().text.delete_at(pos - 1, 1);
        ed.active_window_and_buffer_mut().0.dot.pos -= 1;
        ed.set_dirty(true);
    }
    Ok(())
}

/// forward-char
pub fn forward_char(ed: &mut Editor, f: Flags, n: i32) -> CmdResult {
    let count = if f.has_arg { n.max(1) } else { 1 };
    let buf_len = ed.active_buffer().text.len();
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.pos = (win.dot.pos + count as usize).min(buf_len);
    win.dot.col = win.dot.pos;
    Ok(())
}

/// backward-char
pub fn backward_char(ed: &mut Editor, f: Flags, n: i32) -> CmdResult {
    let count = if f.has_arg { n.max(1) } else { 1 };
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.pos = win.dot.pos.saturating_sub(count as usize);
    win.dot.col = win.dot.pos;
    Ok(())
}

/// next-line
pub fn next_line(ed: &mut Editor, f: Flags, n: i32) -> CmdResult {
    let count = if f.has_arg { n.max(1) } else { 1 } as usize;
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.line = win.dot.line.saturating_add(count);
    Ok(())
}

/// previous-line
pub fn previous_line(ed: &mut Editor, f: Flags, n: i32) -> CmdResult {
    let count = if f.has_arg { n.max(1) } else { 1 } as usize;
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.line = win.dot.line.saturating_sub(count);
    Ok(())
}

/// beginning-of-line
pub fn beginning_of_line(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let pos = ed.active_window().dot.pos;
    let text = ed.active_buffer().text.to_vec();
    let bol = if pos == 0 { 0 } else {
        let mut i = pos - 1;
        loop {
            if text[i] == b'\n' { break i + 1; }
            if i == 0 { break 0; }
            i -= 1;
        }
    };
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.pos = bol;
    win.dot.col = 0;
    Ok(())
}

/// end-of-line
pub fn end_of_line(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let pos = ed.active_window().dot.pos;
    let text = ed.active_buffer().text.to_vec();
    let len = text.len();
    let eol = if pos >= len { len } else {
        let mut i = pos;
        while i < len && text[i] != b'\n' { i += 1; }
        i
    };
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.pos = eol;
    Ok(())
}

/// beginning-of-buffer
pub fn beginning_of_buffer(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.pos = 0;
    win.dot.line = 1;
    win.dot.col = 0;
    Ok(())
}

/// end-of-buffer
pub fn end_of_buffer(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let buf_len = ed.active_buffer().text.len();
    let nl_count = ed.active_buffer().text.to_vec().iter().filter(|&&b| b == b'\n').count();
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.pos = buf_len;
    win.dot.line = nl_count + 1;
    win.dot.col = 0;
    Ok(())
}

/// quoted-insert (C-q)
pub fn quoted_insert(ed: &mut Editor, _f: Flags, c: i32) -> CmdResult {
    self_insert(ed, Flags::default(), c)
}

/// keyboard-quit (C-g)
pub fn keyboard_quit(_ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    Err("Quit".into())
}
