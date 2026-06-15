//! File I/O commands: find-file, save-buffer, write-file, etc.

use std::fs;
use std::io::Read;

use crate::editor::{Editor, CmdResult, Flags};

/// find-file — open a file in a buffer
pub fn find_file(ed: &mut Editor, _f: Flags, name: i32) -> CmdResult {
    // Simple: take filename from prefix arg as path stub
    // Full version uses minibuffer
    let path = "/tmp/glyph.txt";
    match fs::read_to_string(path) {
        Ok(content) => {
            let (_, buf) = ed.active_window_and_buffer_mut();
            buf.text = crate::buffer::text::GapBuffer::from_text(&content);
            buf.filename = path.to_string();
            buf.name = path.rsplit('/').next().unwrap_or(path).to_string();
            ed.set_dirty(false);
            Ok(())
        }
        Err(e) => {
            ed.echo_line = format!("Error reading {}: {}", path, e);
            Ok(())
        }
    }
}

/// save-buffer — save current buffer to its file
pub fn save_buffer(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let buf = ed.active_buffer();
    let path = &buf.filename;
    let content = buf.text.to_string();
    drop(buf);
    if !path.is_empty() {
        fs::write(path, &content)?;
        ed.active_buffer_mut().modified = false;
        ed.set_dirty(false);
    }
    Ok(())
}

/// write-file — save buffer to a specific file
pub fn write_file(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    save_buffer(ed, Flags::default(), 1)
}
