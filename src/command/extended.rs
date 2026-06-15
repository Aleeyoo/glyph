//! Extended editing commands: word motion, page motion, paragraphs, case, etc.

use crate::editor::{Editor, CmdResult, Flags};

/// forward-word (M-f)
pub fn forward_word(ed: &mut Editor, _f: Flags, n: i32) -> CmdResult {
    let count = n.max(1) as usize;
    let text = ed.active_buffer().text.to_vec();
    let len = text.len();
    let mut pos = ed.active_window().dot.pos;
    for _ in 0..count {
        while pos < len && text[pos].is_ascii_whitespace() { pos += 1; }
        while pos < len && !text[pos].is_ascii_whitespace() { pos += 1; }
    }
    ed.active_window_and_buffer_mut().0.dot.pos = pos;
    Ok(())
}

/// backward-word (M-b)
pub fn backward_word(ed: &mut Editor, _f: Flags, n: i32) -> CmdResult {
    let count = n.max(1) as usize;
    let text = ed.active_buffer().text.to_vec();
    let mut pos = ed.active_window().dot.pos;
    for _ in 0..count {
        if pos == 0 { break; }
        pos -= 1;
        while pos > 0 && text[pos].is_ascii_whitespace() { pos -= 1; }
        while pos > 0 && !text[pos].is_ascii_whitespace() && !text[pos-1].is_ascii_whitespace() { pos -= 1; }
    }
    ed.active_window_and_buffer_mut().0.dot.pos = pos;
    Ok(())
}

/// inword — is the current position inside a word?
pub fn inword(ed: &Editor, _pos: usize) -> bool {
    let text = ed.active_buffer().text.to_vec();
    let pos = ed.active_window().dot.pos;
    pos < text.len() && !text[pos].is_ascii_whitespace()
}

/// scroll-up (C-v)
pub fn scroll_up(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let h = ed.active_window().height.saturating_sub(1);
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.line = win.dot.line.saturating_add(h);
    // Simplified: keep dot visible
    Ok(())
}

/// scroll-down (M-v)
pub fn scroll_down(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let h = ed.active_window().height.saturating_sub(1);
    let (win, _) = ed.active_window_and_buffer_mut();
    win.dot.line = win.dot.line.saturating_sub(h);
    Ok(())
}

/// goto-line (M-g g)
pub fn goto_line(ed: &mut Editor, _f: Flags, n: i32) -> CmdResult {
    let target = n.max(1) as usize;
    // Count newlines to find position
    let text = ed.active_buffer().text.to_vec();
    let mut pos = 0;
    let mut line = 1;
    while line < target && pos < text.len() {
        if text[pos] == b'\n' { line += 1; }
        pos += 1;
    }
    ed.active_window_and_buffer_mut().0.dot.pos = pos;
    ed.active_window_and_buffer_mut().0.dot.line = target;
    Ok(())
}

/// open-line (C-o)
pub fn open_line(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, buf) = ed.active_window_and_buffer_mut();
    buf.text.insert_at(win.dot.pos, b"\n");
    ed.set_dirty(true);
    Ok(())
}

/// fill-paragraph (M-q)
pub fn fill_paragraph(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let text = ed.active_buffer().text.to_string();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut filled = String::new();
    let mut col = 0;
    for w in words {
        if col + w.len() + 1 > 72 && col > 0 {
            filled.push('\n');
            col = 0;
        }
        if col > 0 { filled.push(' '); col += 1; }
        filled.push_str(w);
        col += w.len();
    }
    ed.active_buffer_mut().text = crate::buffer::text::GapBuffer::from_text(&filled);
    ed.set_dirty(true);
    Ok(())
}

/// join-line (M-^)
pub fn join_line(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, buf) = ed.active_window_and_buffer_mut();
    let pos = win.dot.pos;
    let text = buf.text.to_vec();
    if pos < text.len() && text[pos] == b'\n' {
        buf.text.delete_at(pos, 1);
        win.dot.col = 0;
        ed.set_dirty(true);
    }
    Ok(())
}

/// upcase-word (M-u)
pub fn upcase_word(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, buf) = ed.active_window_and_buffer_mut();
    let pos = win.dot.pos;
    let text = buf.text.to_vec();
    // Transform first word at point to uppercase
    let mut end = pos;
    let len = text.len();
    while end < len && !text[end].is_ascii_whitespace() { end += 1; }
    if end > pos {
        let word = &text[pos..end];
        let upper: Vec<u8> = word.iter().map(|b| b.to_ascii_uppercase()).collect();
        buf.text.delete_at(pos, end - pos);
        buf.text.insert_at(pos, &upper);
        win.dot.pos = end;
        ed.set_dirty(true);
    }
    Ok(())
}

/// downcase-word (M-l)
pub fn downcase_word(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, buf) = ed.active_window_and_buffer_mut();
    let pos = win.dot.pos;
    let text = buf.text.to_vec();
    let mut end = pos;
    let len = text.len();
    while end < len && !text[end].is_ascii_whitespace() { end += 1; }
    if end > pos {
        let word = &text[pos..end];
        let lower: Vec<u8> = word.iter().map(|b| b.to_ascii_lowercase()).collect();
        buf.text.delete_at(pos, end - pos);
        buf.text.insert_at(pos, &lower);
        win.dot.pos = end;
        ed.set_dirty(true);
    }
    Ok(())
}

/// capitalize-word (M-c)
pub fn capitalize_word(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, buf) = ed.active_window_and_buffer_mut();
    let pos = win.dot.pos;
    let text = buf.text.to_vec();
    let mut end = pos;
    let len = text.len();
    while end < len && !text[end].is_ascii_whitespace() { end += 1; }
    if end > pos {
        let word = &text[pos..end];
        let mut cap: Vec<u8> = word.to_vec();
        cap[0] = cap[0].to_ascii_uppercase();
        for b in &mut cap[1..] { *b = b.to_ascii_lowercase(); }
        buf.text.delete_at(pos, end - pos);
        buf.text.insert_at(pos, &cap);
        win.dot.pos = end;
        ed.set_dirty(true);
    }
    Ok(())
}

/// transpose-chars (C-t)
pub fn transpose_chars(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, buf) = ed.active_window_and_buffer_mut();
    let pos = win.dot.pos;
    if pos >= 1 && pos < buf.text.len() {
        let c_prev = buf.text.byte_at(pos - 1);
        let c_cur = buf.text.byte_at(pos);
        buf.text.delete_at(pos - 1, 2);
        let swapped = [c_cur, c_prev];
        buf.text.insert_at(pos - 1, &swapped);
        win.dot.pos += 1;
        ed.set_dirty(true);
    }
    Ok(())
}

/// delete-blank-lines (C-x C-o)
pub fn delete_blank_lines(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (_, buf) = ed.active_window_and_buffer_mut();
    let text = buf.text.to_vec();
    let mut new_text = Vec::new();
    let mut blank_count = 0;
    for &b in &text {
        if b == b'\n' {
            blank_count += 1;
            if blank_count <= 1 { new_text.push(b); }
        } else {
            blank_count = 0;
            new_text.push(b);
        }
    }
    buf.text = crate::buffer::text::GapBuffer::from_text(&String::from_utf8_lossy(&new_text));
    ed.set_dirty(true);
    Ok(())
}

/// just-one-space (M-SPC)
pub fn just_one_space(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let (win, buf) = ed.active_window_and_buffer_mut();
    let pos = win.dot.pos;
    let text = buf.text.to_vec();
    let mut end = pos;
    while end < text.len() && text[end] == b' ' { end += 1; }
    if end > pos {
        buf.text.delete_at(pos, end - pos - 1);
        ed.set_dirty(true);
    }
    Ok(())
}
