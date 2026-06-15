//! Shell commands, dired, help, bell.

use crate::editor::{Editor, CmdResult, Flags};
use std::process::Command;

/// shell-command (M-!)
pub fn shell_command(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let cmd = &ed.shell_cmd;
    if cmd.is_empty() { return Ok(()); }
    let output = Command::new("sh").arg("-c").arg(cmd).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    ed.echo_line = stdout.trim().to_string();
    Ok(())
}

/// shell-command-on-region (M-|)
pub fn shell_command_on_region(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let cmd = &ed.shell_cmd;
    if cmd.is_empty() { return Ok(()); }
    let text = ed.active_buffer().text.to_string();
    let output = Command::new("sh").arg("-c").arg(cmd)
        .arg(text)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    ed.active_buffer_mut().text = crate::buffer::text::GapBuffer::from_text(&stdout);
    ed.set_dirty(true);
    Ok(())
}

/// describe-key-briefly (C-h c)
pub fn describe_key(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    ed.echo_line = "Describe-key is not fully implemented yet.".to_string();
    Ok(())
}

/// describe-bindings (C-h b)
pub fn describe_bindings(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let names = ed.command_registry.names();
    ed.echo_line = format!("{} commands registered", names.len());
    Ok(())
}

/// apropos (C-h a)
pub fn apropos(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let pat = &ed.search_pattern;
    let names = ed.command_registry.find_prefix(pat);
    ed.echo_line = names.join(", ");
    Ok(())
}

/// help-help (C-h ?)
pub fn help_help(_ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    println!("glyph help: C-h c describe-key, C-h b bindings, C-h a apropos, C-x C-c quit");
    Ok(())
}
