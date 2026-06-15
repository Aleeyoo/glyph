//! Bracket/quote matching — static grammar (not tree-sitter).
//! Includes blink-and-insert / showmatch.

use crate::editor::{Editor, CmdResult, Flags};

/// Matching pairs: opening → closing.
const PAIRS: &[(u8, u8)] = &[
    (b'(', b')'), (b'[', b']'), (b'{', b'}'),
    (b'"', b'"'), (b'\'', b'\''),
];

/// showmatch / blink-and-insert
/// After inserting a closing bracket/quote, briefly flash the matching opener.
pub fn blink_and_insert(ed: &mut Editor, f: Flags, c: i32) -> CmdResult {
    let b = c as u8;
    // Find if this is a closing character
    if let Some(&(open, _)) = PAIRS.iter().find(|&&(_, close)| close == b) {
        let pos = ed.active_window().dot.pos;
        if pos > 0 {
            let text = ed.active_buffer().text.to_vec();
            let mut depth = 1;
            let mut i = pos.saturating_sub(1);
            loop {
                if text[i] == open { depth -= 1; }
                else if text[i] == b { depth += 1; }
                if depth == 0 {
                    ed.echo_line = format!("Matches {}", open as char);
                    break;
                }
                if i == 0 { break; }
                i -= 1;
            }
        }
    }
    // Then insert the character normally
    crate::command::edit::self_insert(ed, f, c)
}

/// Match the bracket at point (showmatch).
pub fn showmatch(ed: &mut Editor, f: Flags, n: i32) -> CmdResult {
    blink_and_insert(ed, f, n)
}
