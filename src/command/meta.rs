//! Minibuffer: echo line reading and writing. Corresponds to mg's echo.c.

use crate::editor::{Editor, CmdResult, Flags};

const ECHO_LEN: usize = 256;

/// Read input from the echo area. Stub — returns empty string.
pub fn eread(_ed: &mut Editor, _prompt: &str) -> String {
    String::new()
}

/// Ask a yes/no question. Returns true for yes.
pub fn eyorn(ed: &mut Editor, question: &str) -> bool {
    ed.echo_line = format!("{}? (y or n) ", question);
    // Non-interactive: assume yes
    true
}

/// Print a message in the echo area.
pub fn ewprintf(ed: &mut Editor, msg: &str) {
    let msg = &msg[..msg.len().min(ECHO_LEN)];
    ed.echo_line = msg.to_string();
}

/// Execute-extended-command (M-x) — list and call commands by name.
pub fn execute_extended_command(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let names: Vec<&str> = ed.command_registry.names();
    if !names.is_empty() {
        ed.echo_line = format!("M-x {}", names[0]);
    }
    Ok(())
}
