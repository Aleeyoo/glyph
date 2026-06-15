use crate::buffer::buffer::Buffer;
use crate::display::frame::Frame;
use crate::types::BufferId;

/// Central editor state. Single object, no globals.
///
/// Manages all buffers, the active frame, and input/display state.
pub struct Editor {
    /// All open buffers.
    pub buffers: Vec<Buffer>,
    /// The active frame (manages windows).
    pub frame: Frame,
    /// Currently active buffer ID.
    pub cur_buffer: BufferId,
    /// Currently active window index (into frame.windows).
    pub cur_window: usize,
    pub running: bool,
}

impl Editor {
    pub fn new(rows: usize, cols: usize) -> Self {
        let scratch = Buffer::new(0, "*scratch*");
        let mut frame = Frame::new(0, rows, cols);
        let win = crate::display::window::Window::new(0, 0, rows - 2);
        frame.windows.push(win);
        frame.active_window = 0;

        Self {
            buffers: vec![scratch],
            frame,
            cur_buffer: 0,
            cur_window: 0,
            running: true,
        }
    }
}
