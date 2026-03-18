/// English RustDoc comment.
/// Wraps a block of text into fixed-width lines for small displays.
pub fn wrap_for_display(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }

    let mut lines = Vec::new();

    for paragraph in text.lines() {
        if paragraph.trim().is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut current = String::new();

        for word in paragraph.split_whitespace() {
            if current.is_empty() {
                if word.len() <= width {
                    current.push_str(word);
                } else {
                    lines.extend(split_long_word(word, width));
                }
                continue;
            }

            let candidate_len = current.len() + 1 + word.len();

            if candidate_len <= width {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current);
                current = String::new();

                if word.len() <= width {
                    current.push_str(word);
                } else {
                    lines.extend(split_long_word(word, width));
                }
            }
        }

        if !current.is_empty() {
            lines.push(current);
        }
    }

    lines
}

/// English RustDoc comment.
/// Splits a single word into fixed-width chunks.
fn split_long_word(word: &str, width: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < word.len() {
        let end = (start + width).min(word.len());
        chunks.push(word[start..end].to_string());
        start = end;
    }

    chunks
}

/// English RustDoc comment.
/// Wraps formatted weather text while preserving `Label: value` alignment.
///
/// Lines without a colon are wrapped as plain text.
pub fn wrap_labeled_text(text: &str, width: usize, label_width: usize) -> Vec<String> {
    if width == 0 {
        return Vec::new();
    }

    let mut output = Vec::new();

    for line in text.lines() {
        let normalized_line = line.trim_start();

        if normalized_line.trim().is_empty() {
            output.push(String::new());
            continue;
        }

        if let Some((label, value)) = normalized_line.split_once(':') {
            let label_text = format!("{label}:");
            let value_text = value.trim();

            let prefix = format!("{label_text:<label_width$}");
            let continuation_prefix = " ".repeat(label_width);

            let available_width = width.saturating_sub(label_width).max(1);
            let wrapped_value = wrap_for_display(value_text, available_width);

            if wrapped_value.is_empty() {
                output.push(prefix);
                continue;
            }

            for (index, chunk) in wrapped_value.iter().enumerate() {
                if index == 0 {
                    output.push(format!("{prefix}{chunk}"));
                } else {
                    output.push(format!("{continuation_prefix}{chunk}"));
                }
            }
        } else {
            output.extend(wrap_for_display(normalized_line, width));
        }
    }

    output
}

/// English RustDoc comment.
/// Splits a list of lines into pages of fixed height.
pub fn paginate_lines(lines: &[String], page_height: usize) -> Vec<Vec<String>> {
    if page_height == 0 {
        return Vec::new();
    }

    let mut pages = Vec::new();
    let mut current_page = Vec::new();

    for line in lines {
        current_page.push(line.clone());

        if current_page.len() == page_height {
            pages.push(current_page);
            current_page = Vec::new();
        }
    }

    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
}

#[cfg(test)]
mod tests {
    use super::{wrap_for_display, wrap_labeled_text};

    /// English RustDoc comment.
    /// Verifies pagination splits lines correctly.
    #[test]
    fn paginate_basic() {
        let lines = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];

        let pages = super::paginate_lines(&lines, 2);

        assert_eq!(pages.len(), 3);
        assert_eq!(pages[0], vec!["A", "B"]);
        assert_eq!(pages[1], vec!["C", "D"]);
        assert_eq!(pages[2], vec!["E"]);
    }

    /// English RustDoc comment.
    /// Verifies pagination with exact multiple of page height.
    #[test]
    fn paginate_exact_multiple() {
        let lines = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
        ];

        let pages = super::paginate_lines(&lines, 2);

        assert_eq!(pages.len(), 2);
    }

    /// English RustDoc comment.
    /// Verifies empty input produces no pages.
    #[test]
    fn paginate_empty() {
        let lines: Vec<String> = Vec::new();

        let pages = super::paginate_lines(&lines, 3);

        assert!(pages.is_empty());
    }

    /// English RustDoc comment.
    /// Verifies that short text remains on a single line.
    #[test]
    fn wrap_short_text() {
        let lines = wrap_for_display("METAR LIRF", 20);
        assert_eq!(lines, vec!["METAR LIRF"]);
    }

    /// English RustDoc comment.
    /// Verifies that long text is wrapped across multiple lines.
    #[test]
    fn wrap_long_text() {
        let lines = wrap_for_display("Visibility greater than 10 km", 12);

        assert_eq!(lines, vec!["Visibility", "greater than", "10 km"]);
    }

    /// English RustDoc comment.
    /// Verifies that empty lines are preserved.
    #[test]
    fn preserve_empty_lines() {
        let lines = wrap_for_display("Line one\n\nLine two", 20);

        assert_eq!(lines, vec!["Line one", "", "Line two"]);
    }

    /// English RustDoc comment.
    /// Verifies that labeled text keeps alignment across wrapped lines.
    #[test]
    fn wrap_labeled_text_preserves_alignment() {
        let lines = wrap_labeled_text("Wind: wind from 180° at 10 kt", 28, 13);

        assert_eq!(
            lines,
            vec!["Wind:        wind from 180°", "             at 10 kt"]
        );
    }

    /// English RustDoc comment.
    /// Verifies that plain text lines are still wrapped normally.
    #[test]
    fn wrap_labeled_text_handles_plain_lines() {
        let lines = wrap_labeled_text("METAR LIRF", 20, 10);

        assert_eq!(lines, vec!["METAR LIRF"]);
    }

    /// English RustDoc comment.
    /// Verifies that leading indentation is ignored before wrapping labeled text.
    #[test]
    fn wrap_labeled_text_trims_leading_indent() {
        let lines = wrap_labeled_text("  Wind: wind from 180° at 10 kt", 28, 13);

        assert_eq!(
            lines,
            vec!["Wind:        wind from 180°", "             at 10 kt",]
        );
    }
}
