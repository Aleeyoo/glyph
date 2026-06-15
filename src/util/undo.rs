//! Undo tree: record changes, undo, redo.

use crate::editor::{Editor, CmdResult, Flags};

#[derive(Clone)]
struct UndoEntry {
    pos: usize,
    len: usize,
    text: Vec<u8>,
}

pub struct UndoTree {
    nodes: Vec<Vec<UndoEntry>>,
    current: usize,
    enabled: bool,
}

impl UndoTree {
    pub fn new() -> Self {
        Self { nodes: vec![vec![]], current: 0, enabled: true }
    }

    pub fn record(&mut self, pos: usize, old_text: Vec<u8>) {
        if !self.enabled { return; }
        // Truncate future branch
        self.nodes.truncate(self.current + 1);
        self.nodes.push(vec![UndoEntry { pos, len: old_text.len(), text: old_text }]);
        self.current = self.nodes.len() - 1;
    }

    pub fn undo(&mut self) -> Option<&[UndoEntry]> {
        if self.current == 0 { return None; }
        self.current -= 1;
        Some(&self.nodes[self.current])
    }

    pub fn redo(&mut self) -> Option<&[UndoEntry]> {
        if self.current + 1 >= self.nodes.len() { return None; }
        self.current += 1;
        Some(&self.nodes[self.current])
    }
}

/// undo (C-z) — undo last change
pub fn undo(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let buf = ed.active_buffer_mut();
    if let Some(entries) = buf.undo.undo() {
        for entry in entries {
            buf.text.delete_at(entry.pos, entry.len);
        }
    }
    Ok(())
}

/// undo-boundary — add an undo boundary
pub fn undo_boundary(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.active_buffer_mut().undo.record(0, vec![]);
    Ok(())
}
