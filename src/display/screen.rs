//! Screen refresh — buffer content rendering.
//!
//! Renders visible lines from the active buffer's GapBuffer to the ratatui frame.

use crate::Editor;
use ratatui::{
    layout::Rect,
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Maximum number of lines to render in one frame.
const MAX_LINES: usize = 4096;

/// Draw the buffer content into the given area.
pub fn draw(ed: &Editor, f: &mut Frame, area: Rect) {
    if area.is_empty() { return; }

    let text = ed.active_buffer().text.to_vec();
    let dot_line = ed.active_window().dot.line.saturating_sub(1); // 0-based
    let height = area.height as usize;
    let start_line = dot_line.saturating_sub(ed.active_window().top_line.saturating_sub(1));
    let start_line = start_line.min(MAX_LINES);
    let mut lines: Vec<Line> = Vec::with_capacity(height);

    // Walk text lines to find the visible range
    let mut line_no: usize = 0;
    let mut pos: usize = 0;
    let text_len = text.len();
    let mut visible_count = 0;

    while pos <= text_len && line_no <= start_line + height {
        // Find end of this line
        let mut end = pos;
        while end < text_len && text[end] != b'\n' { end += 1; }
        let line_slice = &text[pos..end];

        if line_no >= start_line && visible_count < height {
            let default_style = Style::default().fg(Color::White).bg(Color::Black);
            let spans = vec![Span::styled(
                String::from_utf8_lossy(line_slice).to_string(),
                default_style,
            )];
            lines.push(Line::from(spans));
            visible_count += 1;
        }

        // Skip newline
        pos = end + 1;
        line_no += 1;
        if pos > text_len { break; }
    }

    // Fill remaining rows with blank lines
    while lines.len() < height {
        lines.push(Line::from(" "));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}
