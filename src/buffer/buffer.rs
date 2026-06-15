use crate::types::BufferId;

/// A buffer: holds text content, metadata, and editing state.
///
/// In mg2, buffers are the primary editing unit. Every file being edited
/// lives in a buffer. The buffer owns the text (via GapBuffer), tracks
/// file association, and holds mode and undo state.
pub struct Buffer {
    pub id: BufferId,
    pub name: String,
    pub filename: String,
    pub text: super::text::GapBuffer,
    pub modified: bool,
}

impl Buffer {
    pub fn new(id: BufferId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            filename: String::new(),
            text: super::text::GapBuffer::new(),
            modified: false,
        }
    }
}
