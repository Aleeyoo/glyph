//! File encoding detection.
//!
//! Detects encoding by BOM (byte order mark) and falls back to a
//! UTF-8 validation heuristic. No external dependencies.

/// Supported text encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Latin1,
    Utf16Le,
    Utf16Be,
}

/// Detect encoding from raw bytes.
///
/// Strategy (in order):
/// 1. Check for a BOM (byte order mark) at the start of `text`.
/// 2. If the bytes pass UTF-8 validation, return `Utf8`.
/// 3. Otherwise, return `Latin1` (ISO 8859-1, which accepts every
///    possible byte value).
pub fn detect(text: &[u8]) -> Encoding {
    if let Some(enc) = detect_bom(text) {
        return enc;
    }
    if is_valid_utf8(text) {
        Encoding::Utf8
    } else {
        Encoding::Latin1
    }
}

// ── BOM detection ────────────────────────────────────────────────

const BOM_UTF8: &[u8] = &[0xEF, 0xBB, 0xBF];
const BOM_UTF16LE: &[u8] = &[0xFF, 0xFE];
const BOM_UTF16BE: &[u8] = &[0xFE, 0xFF];

/// Returns `Some(encoding)` when a known BOM is found at the start
/// of `text`, otherwise `None`.
fn detect_bom(text: &[u8]) -> Option<Encoding> {
    if text.starts_with(BOM_UTF8) {
        return Some(Encoding::Utf8);
    }
    if text.starts_with(BOM_UTF16LE) {
        return Some(Encoding::Utf16Le);
    }
    if text.starts_with(BOM_UTF16BE) {
        return Some(Encoding::Utf16Be);
    }
    None
}

// ── UTF-8 validation ─────────────────────────────────────────────

/// Returns `true` when `text` is valid UTF-8.
///
/// Based on the well-known algorithm that checks continuation bytes
/// and overlong sequences without allocating.
fn is_valid_utf8(text: &[u8]) -> bool {
    let mut i = 0;
    let len = text.len();
    while i < len {
        let byte = text[i];
        if byte == 0xC0 || byte == 0xC1 {
            // Overlong 2-byte encoding.
            return false;
        }
        if byte >= 0xF5 {
            // Bytes 0xF5–0xFF are never valid in UTF-8.
            return false;
        }
        let (seq_len, check) = if byte < 0x80 {
            i += 1;
            continue;
        } else if byte < 0xC0 {
            // Bare continuation byte.
            return false;
        } else if byte < 0xE0 {
            (2, 1)
        } else if byte < 0xF0 {
            if byte == 0xE0 && i + 2 < len {
                // Overlong 3-byte: must be >= U+0800.
                // 0xE0 0xA0..0xBF is the minimum valid range.
                let b1 = text[i + 1];
                if b1 < 0xA0 || b1 > 0xBF {
                    return false;
                }
            }
            if byte == 0xED && i + 2 < len {
                // Surrogate half (U+D800–U+DFFF) is invalid.
                let b1 = text[i + 1];
                if b1 >= 0xA0 {
                    return false;
                }
            }
            (3, 2)
        } else {
            // byte < 0xF5 (guaranteed by the early return above).
            if byte == 0xF0 && i + 3 < len {
                // Overlong 4-byte: must be >= U+10000.
                let b1 = text[i + 1];
                if b1 < 0x90 || b1 > 0xBF {
                    return false;
                }
            }
            if byte == 0xF4 && i + 3 < len {
                // Beyond U+10FFFF.
                let b1 = text[i + 1];
                if b1 > 0x8F {
                    return false;
                }
            }
            (4, 3)
        };
        if i + seq_len > len {
            return false;
        }
        for j in 1..seq_len {
            let cb = text[i + j];
            if cb < 0x80 || cb > 0xBF {
                return false;
            }
        }
        i += seq_len;
        _ = check;
    }
    true
}

// ── Newline format detection ────────────────────────────────────────

/// Line-ending format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewlineFormat {
    /// Unix / LF (`\n`)
    Lf,
    /// Windows / CRLF (`\r\n`)
    CrLf,
    /// Classic Mac / CR (`\r`)
    Cr,
}

/// Detect the dominant newline format in `text`.
///
/// Scans at most the first 4096 bytes and counts occurrences of `\r\n`,
/// lone `\n`, and lone `\r`. Whichever appears most (ties broken in
/// the order LF > CRLF > CR) is returned.
pub fn detect_newlines(text: &[u8]) -> NewlineFormat {
    const SCAN_LIMIT: usize = 4096;
    let end = text.len().min(SCAN_LIMIT);
    let text = &text[..end];

    let mut crlf = 0usize;
    let mut lf = 0usize;
    let mut cr = 0usize;

    let mut i = 0;
    while i < text.len() {
        match text[i] {
            b'\r' if i + 1 < text.len() && text[i + 1] == b'\n' => {
                crlf += 1;
                i += 2;
            }
            b'\n' => {
                lf += 1;
                i += 1;
            }
            b'\r' => {
                cr += 1;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    // Ties: LF > CRLF > CR.
    if lf >= crlf && lf >= cr {
        NewlineFormat::Lf
    } else if crlf >= cr {
        NewlineFormat::CrLf
    } else {
        NewlineFormat::Cr
    }
}

/// Convert `text` to use `target` line endings.
///
/// Normalises any mix of `\r\n`, `\r`, or `\n` into the target format.
pub fn convert_newlines(text: &str, target: NewlineFormat) -> String {
    let line_ending = match target {
        NewlineFormat::Lf => "\n",
        NewlineFormat::CrLf => "\r\n",
        NewlineFormat::Cr => "\r",
    };

    // First normalise every line-break sequence to `\n`, then replace
    // all `\n` with the target separator.
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if chars.peek() == Some(&'\n') {
                    chars.next(); // consume the `\n`
                }
                result.push('\n');
            }
            '\n' => {
                result.push('\n');
            }
            other => {
                result.push(other);
            }
        }
    }

    // Replace normalised `\n` with the target sequence *in-place*.
    // Just using str::replace on the final string is simpler.
    if line_ending != "\n" {
        let normalised = result;
        result = normalised.replace('\n', line_ending);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── BOM ──

    #[test]
    fn utf8_bom() {
        assert_eq!(detect(&[0xEF, 0xBB, 0xBF, b'a']), Encoding::Utf8);
    }

    #[test]
    fn utf16le_bom() {
        assert_eq!(detect(&[0xFF, 0xFE, b'a', 0x00]), Encoding::Utf16Le);
    }

    #[test]
    fn utf16be_bom() {
        assert_eq!(detect(&[0xFE, 0xFF, 0x00, b'a']), Encoding::Utf16Be);
    }

    // ── Heuristic ──

    #[test]
    fn plain_ascii_is_utf8() {
        assert_eq!(detect(b"hello world"), Encoding::Utf8);
    }

    #[test]
    fn valid_multibyte_is_utf8() {
        // U+00E9 (é) = 0xC3 0xA9, U+2603 (snowman) = 0xE2 0x98 0x83
        assert_eq!(detect(b"caf\xC3\xA9 \xE2\x98\x83"), Encoding::Utf8);
    }

    #[test]
    fn invalid_bytes_fall_back_to_latin1() {
        // 0xFF is never valid UTF-8.
        assert_eq!(detect(&[0xFF, b'a']), Encoding::Latin1);
    }

    #[test]
    fn bare_continuation_byte_is_latin1() {
        assert_eq!(detect(&[0xBF]), Encoding::Latin1);
    }

    #[test]
    fn overlong_2byte_is_invalid() {
        // 0xC0 0x80 would encode U+0000 — overlong, invalid.
        assert_eq!(detect(&[0xC0, 0x80]), Encoding::Latin1);
    }

    #[test]
    fn bytes_above_f4_are_invalid() {
        // 0xF5 is never valid as a leading byte.
        assert_eq!(detect(&[0xF5, 0x80, 0x80, 0x80]), Encoding::Latin1);
    }

    #[test]
    fn empty_is_utf8() {
        assert_eq!(detect(b""), Encoding::Utf8);
    }

    #[test]
    fn surrogate_half_is_invalid() {
        // 0xED 0xA0 0x80 = U+D800 — surrogate, invalid.
        assert_eq!(detect(&[0xED, 0xA0, 0x80]), Encoding::Latin1);
    }

    #[test]
    fn truncated_sequence_is_latin1() {
        // Leading byte expecting 2 continuation bytes, only 1 present.
        assert_eq!(detect(&[0xE0, 0xA0]), Encoding::Latin1);
    }

    // ── Newline detection ──

    #[test]
    fn detect_lf() {
        assert_eq!(detect_newlines(b"hello\nworld\n"), NewlineFormat::Lf);
    }

    #[test]
    fn detect_crlf() {
        assert_eq!(detect_newlines(b"hello\r\nworld\r\n"), NewlineFormat::CrLf);
    }

    #[test]
    fn detect_cr() {
        assert_eq!(detect_newlines(b"hello\rworld\r"), NewlineFormat::Cr);
    }

    #[test]
    fn tie_lf_over_crlf() {
        // One of each — LF tie-break wins.
        assert_eq!(detect_newlines(b"a\nb\r\n"), NewlineFormat::Lf);
    }

    #[test]
    fn tie_crlf_over_cr() {
        // One of each — CRLF tie-break wins over CR.
        assert_eq!(detect_newlines(b"a\r\nb\r"), NewlineFormat::CrLf);
    }

    #[test]
    fn no_newlines_lf() {
        // No line endings at all — defaults to LF.
        assert_eq!(detect_newlines(b"hello world"), NewlineFormat::Lf);
    }

    #[test]
    fn scan_limited_to_4096() {
        // Only the first 4096 bytes are scanned, so a long buffer with
        // CRLF after the limit should still be detected as LF due to
        // LF-newlines within the first 4096 bytes.
        let mut buf = vec![b'\n'; 4096];
        buf.push(b'\r');
        buf.push(b'\n');
        assert_eq!(detect_newlines(&buf), NewlineFormat::Lf);
    }

    // ── Newline conversion ──

    #[test]
    fn convert_to_lf() {
        let input = "a\r\nb\rc";
        assert_eq!(convert_newlines(input, NewlineFormat::Lf), "a\nb\nc");
    }

    #[test]
    fn convert_to_crlf() {
        let input = "a\nb\rc";
        assert_eq!(convert_newlines(input, NewlineFormat::CrLf), "a\r\nb\r\nc");
    }

    #[test]
    fn convert_to_cr() {
        let input = "a\nb\r\nc";
        assert_eq!(convert_newlines(input, NewlineFormat::Cr), "a\rb\rc");
    }

    #[test]
    fn convert_no_newlines() {
        let input = "hello world";
        assert_eq!(convert_newlines(input, NewlineFormat::CrLf), "hello world");
    }

    #[test]
    fn convert_empty() {
        assert_eq!(convert_newlines("", NewlineFormat::CrLf), "");
    }
}
