//! Minibuffer: echo line reading and writing. Corresponds to mg's echo.c.

use crate::editor::{Editor, CmdResult, Flags};
use crate::extend::engine::{self, Env, populate_builtins};

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

/// Evaluate a Lisp expression (M-:).
///
/// Reads an S-expression from the minibuffer, parses it with
/// `extend::engine::read()`, evaluates it with `extend::engine::eval()`,
/// and displays the result in the echo area.
pub fn eval_expression(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    // Read a line from the minibuffer (simplified: use search_pattern as input)
    let mut buf = String::new();
    ed.echo_line = "Eval: ".to_string();
    loop {
        let kc = crate::input::getkey::getkey();
        if kc == 13 || kc == 10 {
            // RET — finish
            break;
        }
        if kc == 3 || kc == 7 {
            // C-c or C-g — abort
            ed.echo_line = "Quit".to_string();
            return Ok(());
        }
        if kc == 0x7f {
            // Backspace
            buf.pop();
        } else if kc >= 32 && kc <= 126 {
            buf.push(char::from_u32(kc as u32).unwrap_or('?'));
        }
    }

    let trimmed = buf.trim().to_string();
    if trimmed.is_empty() {
        return Ok(());
    }

    // Create the Env on first use, then reuse it so defines persist.
    if ed.lisp_env.is_none() {
        let mut env = Env::new();
        populate_builtins(&mut env);
        ed.lisp_env = Some(env);
    }
    let env = ed.lisp_env.as_mut().unwrap();

    // Parse the input as an S-expression
    let expr = match engine::read(&trimmed) {
        Ok(e) => e,
        Err(e) => {
            ed.echo_line = format!("Eval error: {}", e);
            return Ok(());
        }
    };

    // Evaluate the expression
    match engine::eval(env, &expr) {
        Ok(val) => {
            let display = format!("{}", val);
            let display = &display[..display.len().min(ECHO_LEN)];
            ed.echo_line = display.to_string();
        }
        Err(e) => {
            ed.echo_line = format!("Eval error: {}", e);
        }
    }

    Ok(())
}
