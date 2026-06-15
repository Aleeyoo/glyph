//! Buffer management: switch-to-buffer, kill-buffer, list-buffers, etc.

use crate::editor::{Editor, CmdResult, Flags};

/// switch-to-buffer (C-x b)
pub fn switch_to_buffer(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    // Placeholder: picks first buffer
    if ed.buffers.len() > 1 {
        ed.cur_buffer = 1;
    }
    Ok(())
}

/// kill-buffer (C-x k)
pub fn kill_buffer(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    if ed.buffers.len() > 1 {
        ed.buffers.remove(ed.cur_buffer);
        if ed.cur_buffer >= ed.buffers.len() {
            ed.cur_buffer = 0;
        }
    }
    Ok(())
}

/// list-buffers (C-x C-b)
pub fn list_buffers(ed: &mut Editor, _f: Flags, _n: i32) -> CmdResult {
    let mut s = String::new();
    for buf in &ed.buffers {
        s.push_str(&format!(" {}  {}\n", if buf.modified { "*" } else { " " }, buf.name));
    }
    ed.echo_line = s.trim().to_string();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_editor() -> Editor {
        Editor::new(24, 80)
    }

    #[test]
    fn switch_to_buffer_changes_cur() {
        let mut ed = test_editor();
        // Add a second buffer
        let buf2 = crate::buffer::buffer::Buffer::new(1, "test.txt");
        ed.buffers.push(buf2);
        let before = ed.cur_buffer;
        switch_to_buffer(&mut ed, Flags::default(), 1).unwrap();
        assert_ne!(ed.cur_buffer, before);
    }

    #[test]
    fn kill_buffer_removes_one() {
        let mut ed = test_editor();
        let buf2 = crate::buffer::buffer::Buffer::new(1, "test.txt");
        ed.buffers.push(buf2);
        let before = ed.buffers.len();
        kill_buffer(&mut ed, Flags::default(), 1).unwrap();
        assert_eq!(ed.buffers.len(), before - 1);
    }

    #[test]
    fn list_buffers_output() {
        let mut ed = test_editor();
        list_buffers(&mut ed, Flags::default(), 1).unwrap();
        assert!(ed.echo_line.contains("*scratch*"));
    }
}
