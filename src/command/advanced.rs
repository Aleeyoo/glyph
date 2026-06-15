//! Isearch, regex, tags, dired, grep/compile modes.
//! Stubs — full implementations deferred to Phase 3/4 when time permits.

use crate::editor::{Editor, CmdResult, Flags};

// ── Isearch (P2-17) ──

/// isearch-forward (C-s)
pub fn isearch_forward(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "I-search: ".to_string();
    // Stub: accept the next typed character as the search pattern
    Ok(())
}

/// isearch-backward (C-r)
pub fn isearch_backward(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "I-search backward: ".to_string();
    Ok(())
}

// ── Regex search (P2-19/20/21) ──

/// re-search-forward
pub fn re_search_forward(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "Regex search not yet available (compile with --features regex)".to_string();
    Ok(())
}

/// re-search-backward
pub fn re_search_backward(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "Regex search not yet available".to_string();
    Ok(())
}

// ── Tags (P2-24) ──

/// find-tag (M-.)
pub fn find_tag(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "Tags not yet implemented".to_string();
    Ok(())
}

/// pop-tag-mark (M-*)
pub fn pop_tag_mark(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "Tags not yet implemented".to_string();
    Ok(())
}

// ── Grep/Compile (P2-30) ──

/// compile (M-x compile)
pub fn compile(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "Compile not yet implemented".to_string();
    Ok(())
}

/// next-error (C-x `)
pub fn next_error(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "Next error not yet implemented".to_string();
    Ok(())
}
