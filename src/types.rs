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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_new() {
        let p = Point::new(10, 3, 5);
        assert_eq!(p.pos, 10);
        assert_eq!(p.line, 3);
        assert_eq!(p.col, 5);
    }

    #[test]
    fn region_forward_len() {
        let start = Point::new(0, 1, 0);
        let end = Point::new(10, 2, 0);
        let r = Region { start, end };
        assert_eq!(r.len(), 10);
        assert!(!r.is_empty());
    }

    #[test]
    fn region_reverse_len() {
        let start = Point::new(10, 2, 0);
        let end = Point::new(0, 1, 0);
        let r = Region { start, end };
        assert_eq!(r.len(), 0);
        assert!(r.is_empty());
    }

    #[test]
    fn region_equal_len() {
        let p = Point::new(5, 1, 5);
        let r = Region { start: p, end: p };
        assert_eq!(r.len(), 0);
        assert!(r.is_empty());
    }
}
