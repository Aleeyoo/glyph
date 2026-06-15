//! Core types used throughout glyph.
//!
//! Text positions are tracked as byte offsets from the start of the buffer.
//! This keeps indexing O(1) and works naturally with Rust's UTF-8 strings
//! (which index by byte, not codepoint).

/// A byte position within a buffer. Always valid UTF-8 byte offset.
pub type BytePos = usize;

/// A unique identifier for a buffer.
pub type BufferId = usize;

/// A unique identifier for a window.
pub type WindowId = usize;

/// A unique identifier for a frame.
pub type FrameId = usize;

/// A line number (1-based, as shown in the mode line).
pub type LineNo = usize;

/// A point in the buffer: position + line tracking for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    /// Byte offset from start of buffer.
    pub pos: BytePos,
    /// 1-based line number (for display, may be stale).
    pub line: LineNo,
    /// Byte offset within the line (for display).
    pub col: BytePos,
}

impl Point {
    pub const fn new(pos: BytePos, line: LineNo, col: BytePos) -> Self {
        Self { pos, line, col }
    }
}

/// A region between two points (point and mark).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    pub start: Point,
    pub end: Point,
}

impl Region {
    pub fn len(&self) -> BytePos {
        if self.end.pos > self.start.pos {
            self.end.pos - self.start.pos
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
