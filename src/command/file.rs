//! File I/O commands: find-file, save-buffer, write-file, etc.

use std::fs;
use std::io::Read;
use std::time::SystemTime;

use crate::editor::{Editor, CmdResult, Flags};

/// Stat the file associated with a buffer and compare its mtime to the
/// last-recorded mtime. Returns `true` if the file has been modified on disk
/// since we loaded or last saved it.
fn fchecktime(ed: &mut Editor) -> bool {
    let (filename, recorded) = {
        let buf = ed.active_buffer();
        (buf.filename.clone(), buf.mtime)
    };
    if filename.is_empty() || recorded.is_none() {
        return false;
    }
    match fs::metadata(&filename) {
        Ok(meta) => {
            let current = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_secs());
            // If we can't get mtime, be conservative: assume no change
            match current {
                Some(secs) => secs != recorded.unwrap(),
                None => false,
            }
        }
        Err(_) => false,
    }
}

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
            // Record file modification time so we can detect external edits
            buf.mtime = fs::metadata(path)
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_secs());
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
    // Warn if the file was modified on disk since we loaded/last-saved it.
    if fchecktime(ed) {
        ed.echo_line = "Warning: file has changed on disk since last read/save".to_string();
    }

    let path = {
        let buf = ed.active_buffer();
        buf.filename.clone()
    };
    if path.is_empty() {
        return Ok(());
    }
    let content = {
        let buf = ed.active_buffer();
        buf.text.to_string()
    };
    fs::write(&path, &content)?;
    {
        let buf = ed.active_buffer_mut();
        buf.modified = false;
        buf.mtime = fs::metadata(&path)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
    }
    ed.set_dirty(false);
    Ok(())
}

/// write-file — save buffer to a specific file
pub fn write_file(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    save_buffer(ed, Flags::default(), 1)
}
