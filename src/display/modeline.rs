//! Mode line rendering — shows buffer name, modes, modified status, position.

use crate::Editor;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Draw the mode line for the given editor state.
pub fn draw(ed: &Editor, f: &mut Frame, area: Rect) {
    let buf = ed.active_buffer();
    let win = ed.active_window();

    let modified = if buf.modified { "**" } else { "--" };
    let mode_name = "Fundamental"; // placeholder; modes TBD
    let pos = format!("L{}", win.dot.line);

    let text = Line::from(format!("{} Mg: {} ({})  {}", modified, buf.name, mode_name, pos));
    let style = Style::default().bg(Color::Cyan).fg(Color::Black);

    let block = Block::default()
        .borders(Borders::TOP)
        .style(style);
    let paragraph = Paragraph::new(text)
        .block(block);
    f.render_widget(paragraph, area);
}
