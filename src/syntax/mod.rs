//! Syntax highlighting engine — tree-sitter integration (stub + loader).
//!
//! Phase 3 deferred: tree-sitter grammars need cargo add tree-sitter.
//! This module provides the structure; actual parsing activated when tree-sitter is available.

#![allow(dead_code)]

use crate::Editor;

/// Placeholder: language detected by file extension.
pub fn detect_language(_ed: &Editor) -> &'static str {
    // Read buffer filename extension
    // For now: always return "text"
    "text"
}
