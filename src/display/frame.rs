use crate::types::FrameId;
use crate::display::window::Window;

/// A frame manages one full screen and its windows.
pub struct Frame {
    pub id: FrameId,
    pub windows: Vec<Window>,
    pub active_window: usize,
    pub rows: usize,
    pub cols: usize,
}

impl Frame {
    pub fn new(id: FrameId, rows: usize, cols: usize) -> Self {
        Self {
            id,
            windows: Vec::new(),
            active_window: 0,
            rows,
            cols,
        }
    }
}
