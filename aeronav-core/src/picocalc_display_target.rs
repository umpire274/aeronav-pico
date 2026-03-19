use crate::display_target::DisplayTarget;

/// English RustDoc comment.
/// Represents a placeholder display target for the future Picocalc screen.
///
/// This stub stores the last rendered lines in memory.
#[derive(Debug, Default, Clone)]
pub struct PicocalcDisplayTarget {
    lines: Vec<String>,
}

impl PicocalcDisplayTarget {
    /// English RustDoc comment.
    /// Creates a new Picocalc display target stub.
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    /// English RustDoc comment.
    /// Returns the last rendered lines.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}

impl DisplayTarget for PicocalcDisplayTarget {
    fn clear(&mut self) {
        self.lines.clear();
    }

    fn render_lines(&mut self, lines: &[String]) {
        self.lines.clear();
        self.lines.extend(lines.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::PicocalcDisplayTarget;
    use crate::display_target::DisplayTarget;

    /// English RustDoc comment.
    /// Verifies that the display target stores rendered lines.
    #[test]
    fn stores_rendered_lines() {
        let mut display = PicocalcDisplayTarget::new();

        display.render_lines(&["Line 1".to_string(), "Line 2".to_string()]);

        assert_eq!(
            display.lines(),
            &["Line 1".to_string(), "Line 2".to_string()]
        );
    }

    /// English RustDoc comment.
    /// Verifies that clear removes stored lines.
    #[test]
    fn clear_removes_lines() {
        let mut display = PicocalcDisplayTarget::new();

        display.render_lines(&["Line 1".to_string()]);
        display.clear();

        assert!(display.lines().is_empty());
    }
}
