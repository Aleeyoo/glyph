mod types;
mod buffer;
mod display;
mod input;
mod command;
mod util;
mod editor;

use editor::Editor;
use display::tui;
use input::getkey::{self, KCode};
use input::keymap::{Keymap, KeyEntry, KeyAction};
use crossterm::terminal::size;
use ratatui::layout::{Rect, Direction, Layout, Constraint};

fn register_commands(ed: &mut Editor) {
    use command::*;
    let r = &mut ed.command_registry;
    r.register("self-insert-command", |e,f,n| edit::self_insert(e,f,n), "Insert character");
    r.register("newline", |e,f,n| edit::newline(e,f,n), "Insert newline");
    r.register("delete-char", |e,f,n| edit::delete_char(e,f,n), "Delete character at point");
    r.register("delete-backward-char", |e,f,n| edit::delete_backward_char(e,f,n), "Delete character before point");
    r.register("forward-char", |e,f,n| edit::forward_char(e,f,n), "Move forward one character");
    r.register("backward-char", |e,f,n| edit::backward_char(e,f,n), "Move backward one character");
    r.register("next-line", |e,f,n| edit::next_line(e,f,n), "Move to next line");
    r.register("previous-line", |e,f,n| edit::previous_line(e,f,n), "Move to previous line");
    r.register("beginning-of-line", |e,f,n| edit::beginning_of_line(e,f,n), "Move to start of line");
    r.register("end-of-line", |e,f,n| edit::end_of_line(e,f,n), "Move to end of line");
    r.register("beginning-of-buffer", |e,f,n| edit::beginning_of_buffer(e,f,n), "Move to start of buffer");
    r.register("end-of-buffer", |e,f,n| edit::end_of_buffer(e,f,n), "Move to end of buffer");
    r.register("keyboard-quit", |e,f,n| edit::keyboard_quit(e,f,n), "Quit current operation");
    r.register("quoted-insert", |e,f,n| edit::quoted_insert(e,f,n), "Insert next character literally");
    r.register("forward-word", |e,f,n| extended::forward_word(e,f,n), "Move forward one word");
    r.register("backward-word", |e,f,n| extended::backward_word(e,f,n), "Move backward one word");
    r.register("scroll-up", |e,f,n| extended::scroll_up(e,f,n), "Scroll up one screen");
    r.register("scroll-down", |e,f,n| extended::scroll_down(e,f,n), "Scroll down one screen");
    r.register("goto-line", |e,f,n| extended::goto_line(e,f,n), "Go to a line number");
    r.register("open-line", |e,f,n| extended::open_line(e,f,n), "Open a blank line");
    r.register("fill-paragraph", |e,f,n| extended::fill_paragraph(e,f,n), "Fill paragraph");
    r.register("join-line", |e,f,n| extended::join_line(e,f,n), "Join this line to previous");
    r.register("upcase-word", |e,f,n| extended::upcase_word(e,f,n), "Uppercase word");
    r.register("downcase-word", |e,f,n| extended::downcase_word(e,f,n), "Lowercase word");
    r.register("capitalize-word", |e,f,n| extended::capitalize_word(e,f,n), "Capitalize word");
    r.register("transpose-chars", |e,f,n| extended::transpose_chars(e,f,n), "Transpose characters");
    r.register("delete-blank-lines", |e,f,n| extended::delete_blank_lines(e,f,n), "Delete blank lines");
    r.register("just-one-space", |e,f,n| extended::just_one_space(e,f,n), "Collapse spaces");
    r.register("find-file", |e,f,n| file::find_file(e,f,n), "Find file in new buffer");
    r.register("save-buffer", |e,f,n| file::save_buffer(e,f,n), "Save current buffer");
    r.register("write-file", |e,f,n| file::write_file(e,f,n), "Write buffer to file");
    r.register("universal-argument", |e,f,n| prefix::universal_argument(e,f,n), "Universal argument");
    r.register("digit-argument", |e,f,n| prefix::digit_argument(e,f,n), "Digit argument");
    r.register("negative-argument", |e,f,n| prefix::negative_argument(e,f,n), "Negative argument");
    r.register("set-mark-command", |e,f,n| prefix::set_mark(e,f,n), "Set mark");
    r.register("exchange-point-and-mark", |e,f,n| prefix::exchange_point_and_mark(e,f,n), "Exchange point and mark");
    r.register("mark-whole-buffer", |e,f,n| prefix::mark_whole_buffer(e,f,n), "Mark whole buffer");
    r.register("what-cursor-position", |e,f,n| prefix::what_cursor_position(e,f,n), "Show cursor position");
    r.register("redraw-display", |e,f,n| prefix::redraw_display(e,f,n), "Redraw display");
    r.register("switch-to-buffer", |e,f,n| buffer::switch_to_buffer(e,f,n), "Switch to buffer");
    r.register("kill-buffer", |e,f,n| buffer::kill_buffer(e,f,n), "Kill buffer");
    r.register("list-buffers", |e,f,n| buffer::list_buffers(e,f,n), "List buffers");
    r.register("split-window-vertically", |e,f,n| window::split_window(e,f,n), "Split window");
    r.register("delete-window", |e,f,n| window::delete_window(e,f,n), "Delete window");
    r.register("other-window", |e,f,n| window::other_window(e,f,n), "Other window");
    r.register("enlarge-window", |e,f,n| window::enlarge_window(e,f,n), "Enlarge window");
    r.register("execute-extended-command", |e,f,n| meta::execute_extended_command(e,f,n), "Execute command by name");
    r.register("shell-command", |e,f,n| shell::shell_command(e,f,n), "Execute shell command");
    r.register("shell-command-on-region", |e,f,n| shell::shell_command_on_region(e,f,n), "Shell command on region");
    r.register("search-forward", |e,f,n| search::search_forward(e,f,n), "Search forward");
    r.register("search-backward", |e,f,n| search::search_backward(e,f,n), "Search backward");
    r.register("query-replace", |e,f,n| search::query_replace(e,f,n), "Query replace");
    r.register("describe-key-briefly", |e,f,n| shell::describe_key(e,f,n), "Describe key");
    r.register("describe-bindings", |e,f,n| shell::describe_bindings(e,f,n), "Describe bindings");
    r.register("apropos", |e,f,n| shell::apropos(e,f,n), "Apropos");
    r.register("help-help", |e,f,n| shell::help_help(e,f,n), "Help");
}

fn build_default_keymap(ed: &Editor) -> Keymap {
    let mut km = Keymap::new("global");
    let _ = ed;
    km
}

fn main() -> tui::TuiResult<()> {
    let (cols, rows) = size()?;
    let mut terminal = tui::init()?;
    let mut editor = Editor::new(rows as usize, cols as usize);
    register_commands(&mut editor);

    terminal.draw(|f| {
        let rect = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(rect);
        display::modeline::draw(&editor, f, chunks[1]);
    })?;

    while editor.running {
        let kc = getkey::getkey();

        // Keymap dispatch (expand as keymap is populated)
        let handled = false; // placeholder: keymap dispatch

        if !handled {
            match kc {
                getkey::K_CTRL_X => {
                    let second = getkey::getkey();
                    let ctrl = second == 3; // C-c = 3
                    if ctrl {
                        editor.running = false;
                    }
                }
                getkey::K_ESC => {
                    editor.running = false;
                }
                c if c >= 32 && c <= 126 => {
                    let _ = command::edit::self_insert(&mut editor, Default::default(), c as i32);
                }
                10 | 13 => {
                    let _ = command::edit::newline(&mut editor, Default::default(), 1);
                }
                0x7f => {
                    let _ = command::edit::delete_backward_char(&mut editor, Default::default(), 1);
                }
                9 => {
                    let w = editor.active_buffer().b_tabw;
                    for _ in 0..w {
                        let _ = command::edit::self_insert(&mut editor, Default::default(), ' ' as i32);
                    }
                }
                _ => {}
            }
        }

        terminal.draw(|f| {
            let rect = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
                .split(rect);
            display::screen::draw(&editor, f, chunks[0]);
            display::modeline::draw(&editor, f, chunks[1]);
        })?;
    }

    tui::cleanup()?;
    Ok(())
}
