mod types;
mod buffer;
mod display;
mod input;
mod editor;

use editor::Editor;

fn main() {
    // For now, just run with simulated terminal size.
    // P0-9 (ratatui init) deferred until network is available.
    let _editor = Editor::new(24, 80);
    println!("glyph — a lightweight Emacs-compatible text editor");
    println!("Phase 0 skeleton complete. {} tasks done.", 7);
}
