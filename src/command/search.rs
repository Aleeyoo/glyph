//! Search commands: plain search, isearch, replace.

use crate::editor::{Editor, CmdResult, Flags};

/// search-forward (C-s)
pub fn search_forward(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    // Simplified: use last pattern from ed
    let pat = &ed.search_pattern;
    if pat.is_empty() { return Ok(()); }
    let text = ed.active_buffer().text.to_string();
    let pos = ed.active_window().dot.pos;
    if let Some(found) = text[pos..].find(pat) {
        let new_pos = pos + found;
        ed.active_window_and_buffer_mut().0.dot.pos = new_pos;
    }
    Ok(())
}

/// search-backward (C-r)
pub fn search_backward(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let pat = &ed.search_pattern;
    if pat.is_empty() { return Ok(()); }
    let text = ed.active_buffer().text.to_string();
    let pos = ed.active_window().dot.pos;
    if let Some(found) = text[..pos].rfind(pat) {
        ed.active_window_and_buffer_mut().0.dot.pos = found;
    }
    Ok(())
}

/// query-replace (M-%)
pub fn query_replace(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let from = &ed.search_pattern;
    let to = &ed.replace_pattern;
    if from.is_empty() { return Ok(()); }
    let text = ed.active_buffer().text.to_string();
    let replaced = text.replace(from, to);
    ed.active_buffer_mut().text = crate::buffer::text::GapBuffer::from_text(&replaced);
    ed.set_dirty(true);
    Ok(())
}
