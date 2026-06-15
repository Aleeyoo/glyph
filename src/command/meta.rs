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

/// Execute-extended-command (M-x) — read a command name and execute it.
pub fn execute_extended_command(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    // Inline minibuffer read: collect chars until RET
    let mut buf = String::new();
    ed.echo_line = "M-x ".to_string();
    loop {
        // Refresh echo_line with current input
        // In a real TUI this would block for input;
        // for now we simulate with a single getkey
        let kc = crate::input::getkey::getkey();
        if kc == 13 || kc == 10 { // RET
            break;
        }
        if kc == 3 || kc == 7 { // C-c or C-g
            ed.echo_line = "Quit".to_string();
            return Ok(());
        }
        if kc == 0x7f { // Backspace
            buf.pop();
        } else if kc >= 32 && kc <= 126 {
            buf.push(char::from_u32(kc as u32).unwrap_or('?'));
        }
    }

    let trimmed = buf.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    match ed.command_registry.lookup(trimmed) {
        Some(func) => {
            ed.echo_line.clear();
            func(ed, Flags::default(), 1)?;
        }
        None => {
            ed.echo_line = format!("No command: {}", trimmed);
        }
    }
    Ok(())
}
