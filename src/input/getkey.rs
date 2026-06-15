//! Raw key reading via crossterm.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

/// Glyph internal keycode.
/// 0x0000-0x00FF: ASCII + control chars
/// 0x0100+: Function keys and specials
pub type KCode = u16;

pub const K_CTRL_C: KCode = 3;
pub const K_CTRL_G: KCode = 7;
pub const K_CTRL_X: KCode = 24;
pub const K_CTRL_A: KCode = 1;
pub const K_CTRL_B: KCode = 2;
pub const K_CTRL_D: KCode = 4;
pub const K_CTRL_E: KCode = 5;
pub const K_CTRL_F: KCode = 6;
pub const K_CTRL_K: KCode = 11;
pub const K_CTRL_N: KCode = 14;
pub const K_CTRL_P: KCode = 16;
pub const K_CTRL_R: KCode = 18;
pub const K_CTRL_S: KCode = 19;
pub const K_CTRL_V: KCode = 22;
pub const K_CTRL_SPACE: KCode = 0;
pub const K_ESC: KCode = 27;
pub const K_LEFT: KCode = 0x0100;
pub const K_RIGHT: KCode = 0x0101;
pub const K_UP: KCode = 0x0102;
pub const K_DOWN: KCode = 0x0103;
pub const K_HOME: KCode = 0x0104;
pub const K_END: KCode = 0x0105;
pub const K_PGUP: KCode = 0x0106;
pub const K_PGDN: KCode = 0x0107;
pub const K_DEL: KCode = 0x0108;

/// Read a single key from the terminal, translating to an internal keycode.
pub fn getkey() -> KCode {
    loop {
        match event::read() {
            Ok(Event::Key(ke)) if ke.kind == KeyEventKind::Press => {
                return translate_key(ke.code, ke.modifiers);
            }
            Ok(Event::Resize(_w, _h)) => {
                // Resize handled via signal; skip the synthetic code for now.
                continue;
            }
            _ => continue,
        }
    }
}

fn translate_key(code: KeyCode, mods: event::KeyModifiers) -> KCode {
    let ctrl = mods.contains(event::KeyModifiers::CONTROL);
    match code {
        KeyCode::Char(c) => {
            if ctrl && c >= 'a' && c <= 'z' {
                (c as u8 - b'a' + 1) as u16 // C-a = 1, C-b = 2, ...
            } else if ctrl && c >= 'A' && c <= 'Z' {
                (c as u8 - b'A' + 1) as u16
            } else {
                c as u16
            }
        }
        KeyCode::Esc => K_ESC,
        KeyCode::Enter => b'\n' as u16,
        KeyCode::Backspace => 0x7f,
        KeyCode::Tab => 9,
        KeyCode::Left => K_LEFT,
        KeyCode::Right => K_RIGHT,
        KeyCode::Up => K_UP,
        KeyCode::Down => K_DOWN,
        KeyCode::Home => K_HOME,
        KeyCode::End => K_END,
        KeyCode::PageUp => K_PGUP,
        KeyCode::PageDown => K_PGDN,
        KeyCode::Delete => K_DEL,
        _ => 0,
    }
}
