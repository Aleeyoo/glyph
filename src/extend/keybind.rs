//! Lisp-level key binding functions.
//!
//! Provides `define-key` and `global-set-key` for minibuffer/config-file usage.
//! Key strings use the Emacs notation: `\C-x`, `\C-f`, `\M-f`, etc.

use crate::Editor;

/// Parse an Emacs-style key string like `\C-x\C-f` into a vector of keycodes.
///
/// Supported escapes:
/// - `\C-x` — Control-modified key (x = any letter)
/// - `\M-x` — Meta/Alt-modified key (x = any letter)
/// - `\C-\M-x` or `\M-\C-x` — Control+Meta combined
/// - Literal characters `a`, `1`, etc. are ASCII values
/// - `\d` — Backspace / Delete
/// - `\e` — Escape
/// - `\t` — Tab
/// - `\r` — Return
/// - `\S-x` — Shift-modified (uppercase letter)
pub fn parse_key_sequence(key: &str) -> Result<Vec<u16>, String> {
    let mut keys = Vec::new();
    let mut chars = key.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c == '\\' {
            chars.next(); // consume backslash
            let esc = chars.next().ok_or("unexpected end after \\")?;
            match esc {
                'C' | 'c' => {
                    // Expect `-` then a letter
                    let hyphen = chars.next().ok_or("expected - after \\C")?;
                    if hyphen != '-' {
                        return Err(format!("expected - after \\C, got '{}'", hyphen));
                    }
                    let ch = chars.next().ok_or("expected key after \\C-")?;
                    if !ch.is_ascii_alphabetic() {
                        return Err(format!(
                            "expected alphabetic key after \\C-, got '{}'", ch
                        ));
                    }
                    // Control codes: C-a = 1, C-z = 26
                    let ctrl_code = (ch.to_ascii_lowercase() as u8 - b'a' + 1) as u16;
                    keys.push(ctrl_code);
                }
                'M' | 'm' => {
                    // Meta is represented as keycode + 0x80 (high bit set)
                    let hyphen = chars.next().ok_or("expected - after \\M")?;
                    if hyphen != '-' {
                        return Err(format!("expected - after \\M, got '{}'", hyphen));
                    }

                    // Check if it's actually \M-\C-x (meta+control)
                    let mut meta_kc = parse_key_escape_sequence(&mut chars)?;
                    if meta_kc < 0x80 {
                        meta_kc |= 0x80;
                    }
                    keys.push(meta_kc);
                }
                'd' => keys.push(0x7f),   // delete/backspace
                'e' => keys.push(27),      // escape
                't' => keys.push(9),       // tab
                'r' => keys.push(13),      // return
                'S' => {
                    // Shift modifier: accept `\S-` then next char
                    let hyphen = chars.next().ok_or("expected - after \\S")?;
                    if hyphen != '-' {
                        return Err(format!("expected - after \\S, got '{}'", hyphen));
                    }
                    let ch = chars.next().ok_or("expected key after \\S-")?;
                    keys.push(ch.to_ascii_uppercase() as u16);
                }
                other => {
                    return Err(format!("unrecognized escape: \\{}", other));
                }
            }
        } else {
            // Literal character
            keys.push(c as u16);
            chars.next();
        }
    }

    Ok(keys)
}

/// Parse the character after a `\something-` prefix. Used inside `\M-` to handle
/// nested sequences like `\M-\C-x`.
fn parse_key_escape_sequence(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<u16, String> {
    let c = chars.next().ok_or("expected key after modifier prefix")?;
    if c == '\\' {
        let esc = chars.next().ok_or("unexpected end after \\")?;
        match esc {
            'C' | 'c' => {
                let hyphen = chars.next().ok_or("expected - after \\C inside \\M-")?;
                if hyphen != '-' {
                    return Err(format!("expected -, got '{}'", hyphen));
                }
                let ch = chars.next().ok_or("expected key after \\C-")?;
                let code = if ch.is_ascii_alphabetic() {
                    (ch.to_ascii_lowercase() as u8 - b'a' + 1) as u16
                } else {
                    ch as u16
                };
                Ok(code | 0x80) // meta + control
            }
            'd' => Ok(0x7f | 0x80),
            'e' => Ok(27 | 0x80),
            't' => Ok(9 | 0x80),
            'r' => Ok(13 | 0x80),
            other => Err(format!("unrecognized escape inside \\M-: \\{}", other)),
        }
    } else {
        // Meta-modified plain character
        Ok((c as u16) | 0x80)
    }
}

/// `define-key` — register a key binding in the editor's command registry.
///
/// Takes a key string (e.g. `\C-x\C-f`) and a command name (e.g. `find-file`).
/// For now this sets `echo_line` for debugging. The actual keymap structure
/// needs refactoring to support runtime binding changes.
pub fn lisp_define_key(ed: &mut Editor, key: &str, cmd: &str) -> Result<(), String> {
    let keys = parse_key_sequence(key)?;

    // Validate the command exists in the registry
    if ed.command_registry.lookup(cmd).is_none() {
        return Err(format!("define-key: unknown command: {}", cmd));
    }

    ed.echo_line = format!("define-key: {} -> {} ({} keycodes)", key, cmd, keys.len());
    Ok(())
}

/// `global-set-key` — register a key binding globally.
///
/// Semantically identical to `define-key` in current editor; both modify
/// the global keymap. For now this sets `echo_line` for debugging.
pub fn lisp_global_set_key(ed: &mut Editor, key: &str, cmd: &str) -> Result<(), String> {
    let keys = parse_key_sequence(key)?;

    // Validate the command exists in the registry
    if ed.command_registry.lookup(cmd).is_none() {
        return Err(format!("global-set-key: unknown command: {}", cmd));
    }

    ed.echo_line = format!("global-set-key: {} -> {} ({} keycodes)", key, cmd, keys.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Editor;

    fn test_editor() -> Editor {
        Editor::new(24, 80)
    }

    #[test]
    fn parse_ctrl_a() {
        let keys = parse_key_sequence(r"\C-a").unwrap();
        assert_eq!(keys, vec![1]);
    }

    #[test]
    fn parse_ctrl_z() {
        let keys = parse_key_sequence(r"\C-z").unwrap();
        assert_eq!(keys, vec![26]);
    }

    #[test]
    fn parse_ctrl_x_ctrl_f() {
        let keys = parse_key_sequence(r"\C-x\C-f").unwrap();
        assert_eq!(keys, vec![24, 6]);
    }

    #[test]
    fn parse_meta_f() {
        let keys = parse_key_sequence(r"\M-f").unwrap();
        // Meta modifier sets bit 7 (0x80)
        assert_eq!(keys, vec![0x80 | 'f' as u16]);
    }

    #[test]
    fn parse_meta_ctrl_x() {
        let keys = parse_key_sequence(r"\M-\C-x").unwrap();
        // Meta+Ctrl-x = 0x80 | 24
        assert_eq!(keys, vec![0x80 | 24]);
    }

    #[test]
    fn parse_escape_sequences() {
        let keys = parse_key_sequence(r"\d\e\t\r").unwrap();
        assert_eq!(keys, vec![0x7f, 27, 9, 13]);
    }

    #[test]
    fn parse_shift_key() {
        let keys = parse_key_sequence(r"\S-a").unwrap();
        assert_eq!(keys, vec!['A' as u16]);
    }

    #[test]
    fn parse_multiple_literals() {
        let keys = parse_key_sequence("abc").unwrap();
        assert_eq!(keys, vec![97, 98, 99]);
    }

    #[test]
    fn parse_mixed_literal_and_control() {
        let keys = parse_key_sequence(r"a\C-b").unwrap();
        assert_eq!(keys, vec![97, 2]);
    }

    #[test]
    fn parse_ctrl_uppercase() {
        let keys = parse_key_sequence(r"\C-A").unwrap();
        assert_eq!(keys, vec![1]);
    }

    #[test]
    fn parse_empty_string() {
        let keys = parse_key_sequence("").unwrap();
        assert!(keys.is_empty());
    }

    #[test]
    fn parse_invalid_trailing_backslash() {
        let result = parse_key_sequence(r"\");
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_non_alpha_after_ctrl() {
        let result = parse_key_sequence(r"\C-1");
        assert!(result.is_err());
    }

    #[test]
    fn define_key_sets_echo_for_known_command() {
        let mut ed = test_editor();
        ed.command_registry.register("test-cmd", |_, _, _| Ok(()), "test doc");

        let result = lisp_define_key(&mut ed, r"\C-x\C-f", "test-cmd");
        assert!(result.is_ok());
        assert!(ed.echo_line.contains("define-key:"));
        assert!(ed.echo_line.contains("test-cmd"));
    }

    #[test]
    fn define_key_errors_on_unknown_command() {
        let mut ed = test_editor();
        let result = lisp_define_key(&mut ed, r"\C-x\C-f", "nonexistent-cmd");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("unknown command"));
    }

    #[test]
    fn global_set_key_sets_echo_for_known_command() {
        let mut ed = test_editor();
        ed.command_registry.register("test-cmd", |_, _, _| Ok(()), "test doc");

        let result = lisp_global_set_key(&mut ed, r"\C-c", "test-cmd");
        assert!(result.is_ok());
        assert!(ed.echo_line.contains("global-set-key:"));
    }

    #[test]
    fn global_set_key_errors_on_unknown_command() {
        let mut ed = test_editor();
        let result = lisp_global_set_key(&mut ed, r"\C-c", "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn define_key_rejects_bad_key_string() {
        let mut ed = test_editor();
        let result = lisp_define_key(&mut ed, r"\C-", "test-cmd");
        assert!(result.is_err());
    }
}
