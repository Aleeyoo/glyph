//! Async I/O stub. Full implementation deferred — needs threading/callback architecture.
//! Phase 3 deferred: requires thread pool or tokio integration.

#![allow(dead_code)]

use crate::Editor;

/// Read a file in the background. Stub — reads synchronously for now.
pub fn read_file_async(ed: &mut Editor, path: &str) {
    // Deferred: spawn thread, read file, post completion to editor queue
    // For now, just set echo_line
    ed.echo_line = format!("Async read not yet available (would load: {})", path);
}
