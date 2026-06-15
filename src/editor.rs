use crate::buffer::buffer::Buffer;
use crate::display::frame::Frame;
use crate::display::window::Window;
use crate::types::BufferId;
use crate::command::CommandRegistry;

pub type CmdResult = crate::command::CmdResult;
pub type Flags = crate::command::Flags;

/// Central editor state. Single object, no globals.
pub struct Editor {
    pub buffers: Vec<Buffer>,
    pub frame: Frame,
    pub cur_buffer: BufferId,
    pub cur_window: usize,
    pub running: bool,
    pub dirty: bool,
    pub prefix_arg: usize,
    pub prefix_negative: bool,
    pub echo_line: String,
    pub command_registry: CommandRegistry,
}

impl Editor {
    pub fn new(rows: usize, cols: usize) -> Self {
        let scratch = Buffer::new(0, "*scratch*");
        let mut frame = Frame::new(0, rows, cols);
        let win = Window::new(0, 0, rows - 2);
        frame.windows.push(win);
        frame.active_window = 0;

        Self {
            buffers: vec![scratch],
            frame,
            cur_buffer: 0,
            cur_window: 0,
            running: true,
            dirty: false,
            prefix_arg: 0,
            prefix_negative: false,
            echo_line: String::new(),
            command_registry: CommandRegistry::new(),
        }
    }

    pub fn active_buffer(&self) -> &Buffer {
        &self.buffers[self.cur_buffer]
    }

    pub fn active_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.cur_buffer]
    }

    pub fn active_window(&self) -> &Window {
        &self.frame.windows[self.cur_window]
    }

    /// Combined mutable access to active window and buffer, avoiding double-borrow.
    pub fn active_window_and_buffer_mut(&mut self) -> (&mut Window, &mut Buffer) {
        let win = &mut self.frame.windows[self.cur_window];
        let buf = &mut self.buffers[self.cur_buffer];
        (win, buf)
    }

    pub fn set_dirty(&mut self, val: bool) {
        self.dirty = val;
        if val {
            self.buffers[self.cur_buffer].modified = true;
        }
    }
}
