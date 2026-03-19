use crate::frame::UiFrame;

/// English RustDoc comment.
/// Represents a renderer that transforms UI frames into display-ready lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayRenderer {
    pub max_lines: usize,
}

impl DisplayRenderer {
    /// English RustDoc comment.
    /// Creates a new display renderer with a maximum number of lines.
    pub const fn new(max_lines: usize) -> Self {
        Self { max_lines }
    }

    /// English RustDoc comment.
    /// Returns a default renderer suitable for CLI preview usage.
    pub const fn cli_default() -> Self {
        Self { max_lines: 32 }
    }

    /// English RustDoc comment.
    /// Returns a provisional renderer suitable for Picocalc-like displays.
    pub const fn picocalc_default() -> Self {
        Self { max_lines: 8 }
    }

    /// English RustDoc comment.
    /// Renders a frame into a bounded list of display-ready lines.
    pub fn render_lines(&self, frame: &UiFrame) -> Vec<String> {
        let mut lines = frame.lines();

        if lines.len() > self.max_lines {
            lines.truncate(self.max_lines);
        }

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::DisplayRenderer;
    use crate::frame::UiFrame;

    /// English RustDoc comment.
    /// Verifies that rendering preserves all lines when under the limit.
    #[test]
    fn render_lines_under_limit() {
        let renderer = DisplayRenderer::new(10);
        let frame = UiFrame::new(
            vec!["H".to_string()],
            vec!["C1".to_string(), "C2".to_string()],
            vec!["F".to_string()],
        );

        let lines = renderer.render_lines(&frame);

        assert_eq!(
            lines,
            vec![
                "H".to_string(),
                "C1".to_string(),
                "C2".to_string(),
                "F".to_string()
            ]
        );
    }

    /// English RustDoc comment.
    /// Verifies that rendering truncates lines above the limit.
    #[test]
    fn render_lines_truncate() {
        let renderer = DisplayRenderer::new(2);
        let frame = UiFrame::new(
            vec!["H".to_string()],
            vec!["C1".to_string(), "C2".to_string()],
            vec!["F".to_string()],
        );

        let lines = renderer.render_lines(&frame);

        assert_eq!(lines, vec!["H".to_string(), "C1".to_string()]);
    }

    /// English RustDoc comment.
    /// Verifies Picocalc default renderer line capacity.
    #[test]
    fn picocalc_default_renderer() {
        let renderer = DisplayRenderer::picocalc_default();
        assert_eq!(renderer.max_lines, 8);
    }
}
