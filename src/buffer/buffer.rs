use crate::types::BufferId;
use crate::util::undo::UndoTree;

pub struct Buffer {
    pub id: BufferId,
    pub name: String,
    pub filename: String,
    pub text: super::text::GapBuffer,
    pub b_tabw: usize,
    pub modified: bool,
    pub undo: UndoTree,
}

impl Buffer {
    pub fn new(id: BufferId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            filename: String::new(),
            text: super::text::GapBuffer::new(),
            b_tabw: 8,
            modified: false,
            undo: UndoTree::new(),
        }
    }
}
