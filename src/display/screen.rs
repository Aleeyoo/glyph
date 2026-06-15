//! Screen refresh — buffer content rendering with cursor.
//!
//! Renders visible lines from the active buffer's GapBuffer to the ratatui frame.

use crate::Editor;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

/// Draw the buffer content into the given area.
pub fn draw(ed: &Editor, f: &mut Frame, area: Rect) {
    if area.is_empty() { return; }

    let text = ed.active_buffer().text.to_vec();
    let _dot_pos = ed.active_window().dot.pos;
    let height = area.height as usize;

    // Empty buffer: show ~ lines (Emacs style)
    if text.is_empty() {
        let lines: Vec<Line> = (0..height)
            .map(|_| Line::from(Span::styled("~", Style::default().fg(Color::DarkGray))))
            .collect();
        f.render_widget(Paragraph::new(lines), area);
        return;
    }

    // Compute line starts by scanning for newlines
    let mut line_starts: Vec<usize> = vec![0];
    for (i, &b) in text.iter().enumerate() {
        if b == b'\n' {
            line_starts.push(i + 1);
        }
    }

    let total_lines = line_starts.len();
    let dot_line_num = ed.active_window().dot.line.saturating_sub(1);
    let top_line = ed.active_window().top_line.saturating_sub(1);

    // Find which display row the dot line is on
    let mut display_line;
    if dot_line_num >= total_lines {
        display_line = total_lines.saturating_sub(1).saturating_sub(top_line);
    } else {
        display_line = dot_line_num.saturating_sub(top_line);
    }

    // Find the first visible line index
    let start_idx = if top_line >= total_lines {
        total_lines.saturating_sub(1)
    } else {
        top_line
    };

    let mut lines: Vec<Line> = Vec::with_capacity(height);

    for row in 0..height {
        let line_idx = start_idx + row;

        if line_idx < total_lines {
            let line_start = line_starts[line_idx];
            let line_end = if line_idx + 1 < total_lines {
                line_starts[line_idx + 1].saturating_sub(1)
            } else {
                text.len()
            };

            let line_slice = if line_end > line_start {
                &text[line_start..line_end]
            } else {
                &[]
            };

            let is_cursor_line = row == display_line;

            // Convert to display string with tab expansion
            let display_text = String::from_utf8_lossy(line_slice)
                .replace('\t', "        ");

            let style = if is_cursor_line {
                Style::default().fg(Color::White).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White).bg(Color::Black)
            };

            lines.push(Line::from(Span::styled(display_text, style)));
        } else {
            // Empty line past end of buffer
            lines.push(Line::from(Span::styled("~", Style::default().fg(Color::DarkGray))));
        }
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}
