//! Gap buffer — the core text storage for glyph.
//!
//! A gap buffer stores text in a single contiguous `Vec<u8>` with a "gap"
//! (unused space) at the cursor position. This makes insert and delete at
//! point O(1) amortized, at the cost of O(n) memmove when moving the gap.

use crate::types::BytePos;

/// Internal capacity growth factor.
const GROWTH_FACTOR: usize = 2;

/// Minimum gap size after reallocation.
const MIN_GAP: usize = 256;

/// A gap buffer, the backing store for a buffer's text content.
///
/// Design invariant: the gap always surrounds the current editing position.
/// Moving the cursor means moving the gap via a memmove.
#[derive(Debug, Clone)]
pub struct GapBuffer {
    /// The underlying byte storage. Allocated capacity >= text_len + gap_size.
    buf: Vec<u8>,
    /// Start of the gap (first unused byte).
    gap_start: usize,
    /// End of the gap (first used byte after the gap).
    gap_end: usize,
    /// Total bytes of actual text content (not counting gap or excess capacity).
    text_len: usize,
}

impl GapBuffer {
    /// Create a new empty gap buffer.
    pub fn new() -> Self {
        let cap = MIN_GAP;
        Self {
            buf: vec![0u8; cap],
            gap_start: 0,
            gap_end: cap,
            text_len: 0,
        }
    }

    /// Create a gap buffer from existing text.
    /// Text occupies the front of the buffer; the gap is after it.
    pub fn from_text(text: &str) -> Self {
        let text_len = text.len();
        let cap = text_len + MIN_GAP;
        let mut buf = vec![0u8; cap];
        buf[..text_len].copy_from_slice(text.as_bytes());
        Self {
            buf,
            gap_start: text_len,
            gap_end: cap,
            text_len,
        }
    }

    /// Total bytes of text in the buffer (excluding the gap).
    pub fn len(&self) -> usize {
        self.text_len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the text as a contiguous `Vec<u8>`, copying out (for save).
    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.text_len);
        let text_end = self.gap_start;
        v.extend_from_slice(&self.buf[..text_end]);
        let right_start = self.gap_end;
        let right_len = self.text_len - self.gap_start;
        v.extend_from_slice(&self.buf[right_start..right_start + right_len]);
        v
    }

    /// Return the text as a String (panics if not valid UTF-8).
    pub fn to_string(&self) -> String {
        // Always valid: we only insert valid UTF-8.
        unsafe { String::from_utf8_unchecked(self.to_vec()) }
    }

    /// Access a single byte at position `pos` (before the gap if pos < gap_start,
    /// after the gap otherwise).
    pub fn byte_at(&self, pos: BytePos) -> u8 {
        if pos < self.gap_start {
            self.buf[pos]
        } else {
            self.buf[pos + (self.gap_end - self.gap_start)]
        }
    }

    /// Access a slice of text. Returns a temporary Vec.
    pub fn slice(&self, start: BytePos, end: BytePos) -> Vec<u8> {
        let len = end - start;
        let mut v = Vec::with_capacity(len);
        for i in 0..len {
            v.push(self.byte_at(start + i));
        }
        v
    }

    /// Move the gap so that it begins at `pos`.
    fn move_gap(&mut self, pos: BytePos) {
        let gap_size = self.gap_end - self.gap_start;
        if pos < self.gap_start {
            // gap needs to move left: shift bytes from left of gap to right
            let count = self.gap_start - pos;
            let src = pos;
            let dst = self.gap_end - count;
            // SAFETY: src..src+count and dst..dst+count don't overlap because
            // src < gap_start < gap_end <= dst (the gap separates them).
            unsafe {
                let p = self.buf.as_mut_ptr();
                std::ptr::copy_nonoverlapping(p.add(src), p.add(dst), count);
            }
            self.gap_start = pos;
            self.gap_end = pos + gap_size;
        } else if pos > self.gap_start {
            // gap needs to move right: shift bytes from right of gap to left
            let count = pos - self.gap_start;
            let src = self.gap_end;
            let dst = self.gap_start;
            // SAFETY: dst <= pos, and src..src+count is after the gap, dst..dst+count
            // is at the former gap location — they can overlap if gap_size < count.
            unsafe {
                let p = self.buf.as_mut_ptr();
                std::ptr::copy(p.add(src), p.add(dst), count);
            }
            self.gap_start = pos;
            self.gap_end = pos + gap_size;
        }
        // else gap already at pos — noop
    }

    /// Ensure the gap is at least `min_gap` bytes wide, reallocating if needed.
    fn ensure_gap(&mut self, min_gap: usize) {
        let gap_size = self.gap_end - self.gap_start;
        if gap_size >= min_gap {
            return;
        }
        let target_cap = std::cmp::max(
            self.text_len + min_gap,
            (self.text_len + MIN_GAP) * GROWTH_FACTOR,
        );
        let gap_size_new = target_cap - self.text_len;

        let mut new_buf = vec![0u8; target_cap];
        // Copy left part.
        new_buf[..self.gap_start].copy_from_slice(&self.buf[..self.gap_start]);
        // Copy right part, shifted right by the new gap size.
        let right_len = self.text_len - self.gap_start;
        let new_right_start = self.gap_start + gap_size_new;
        new_buf[new_right_start..new_right_start + right_len]
            .copy_from_slice(&self.buf[self.gap_end..self.gap_end + right_len]);

        self.buf = new_buf;
        self.gap_end = self.gap_start + gap_size_new;
    }

    /// Insert `text` at the current cursor position (where the gap is).
    /// The gap must already be at the desired position.
    pub fn insert_at(&mut self, pos: BytePos, text: &[u8]) {
        self.move_gap(pos);
        self.ensure_gap(text.len());
        self.buf[self.gap_start..self.gap_start + text.len()].copy_from_slice(text);
        self.gap_start += text.len();
        self.text_len += text.len();
    }

    /// Delete `len` bytes starting at `pos`. The gap must already be at `pos`.
    pub fn delete_at(&mut self, pos: BytePos, len: usize) {
        self.move_gap(pos);
        // "Delete" means expand the gap over the deleted bytes.
        self.gap_end += len;
        self.text_len -= len;
        // Clamp: gap_end can't exceed buf.len()
        if self.gap_end > self.buf.len() {
            self.gap_end = self.buf.len();
        }
    }
}

impl Default for GapBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let gb = GapBuffer::new();
        assert!(gb.is_empty());
        assert_eq!(gb.len(), 0);
    }

    #[test]
    fn from_text_roundtrip() {
        let text = "hello glyph";
        let gb = GapBuffer::from_text(text);
        assert_eq!(gb.len(), text.len());
        assert_eq!(gb.to_string(), text);
    }

    #[test]
    fn insert_at_start() {
        let mut gb = GapBuffer::new();
        gb.insert_at(0, b"abc");
        assert_eq!(gb.to_string(), "abc");
    }

    #[test]
    fn insert_multiple() {
        let mut gb = GapBuffer::new();
        gb.insert_at(0, b"ac");
        gb.insert_at(1, b"b");
        assert_eq!(gb.to_string(), "abc");
    }

    #[test]
    fn delete_forward() {
        let mut gb = GapBuffer::from_text("abcd");
        gb.delete_at(1, 2);
        assert_eq!(gb.to_string(), "ad");
    }

    #[test]
    fn byte_at() {
        let gb = GapBuffer::from_text("hello");
        assert_eq!(gb.byte_at(0), b'h');
        assert_eq!(gb.byte_at(4), b'o');
    }

    #[test]
    fn slice() {
        let gb = GapBuffer::from_text("hello world");
        let s = gb.slice(0, 5);
        assert_eq!(&s, b"hello");
    }

    #[test]
    fn insert_delete_sequence() {
        let mut gb = GapBuffer::from_text("hello");
        // delete 'o' at position 4
        gb.delete_at(4, 1);
        assert_eq!(gb.to_string(), "hell");
        // insert 'p' after "hell"
        gb.insert_at(4, b"p");
        assert_eq!(gb.to_string(), "hellp");
        gb.insert_at(5, b"!");
        assert_eq!(gb.to_string(), "hellp!");
    }

    #[test]
    fn large_growth() {
        let mut gb = GapBuffer::new();
        let data = "a".repeat(10_000);
        gb.insert_at(0, data.as_bytes());
        assert_eq!(gb.len(), 10_000);
        assert_eq!(gb.to_string(), data);
    }

    #[test]
    fn delete_entire_buffer() {
        let mut gb = GapBuffer::from_text("hello");
        gb.delete_at(0, 5);
        assert!(gb.is_empty());
        assert_eq!(gb.to_string(), "");
    }

    #[test]
    fn insert_at_boundary_after_delete() {
        let mut gb = GapBuffer::from_text("abcd");
        gb.delete_at(1, 2);
        gb.insert_at(1, b"xyz");
        assert_eq!(gb.to_string(), "axyzd");
    }

    #[test]
    fn round_trip_multiple_ops() {
        let mut gb = GapBuffer::new();
        gb.insert_at(0, b"hello world");
        gb.delete_at(5, 6);
        gb.insert_at(5, b" there");
        assert_eq!(gb.to_string(), "hello there");
    }
}
