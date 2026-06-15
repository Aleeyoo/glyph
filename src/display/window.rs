use crate::types::{BytePos, LineNo, Point, WindowId};

/// A window into a buffer. Each window has its own cursor (dot) and mark.
pub struct Window {
    pub id: WindowId,
    pub buffer_id: usize,
    /// Cursor position (dot).
    pub dot: Point,
    /// Mark position (for region selection).
    pub mark: Option<Point>,
    /// Top line displayed in the window.
    pub top_line: usize,
    /// Number of visible rows.
    pub height: usize,
    /// Cursor column (0-based on screen).
    pub cur_col: usize,
}

impl Window {
    pub fn new(id: WindowId, buffer_id: usize, height: usize) -> Self {
        Self {
            id,
            buffer_id,
            dot: Point::new(0, 1, 0),
            mark: None,
            top_line: 1,
            height,
            cur_col: 0,
        }
    }
}
