/// English RustDoc comment.
/// Represents a target capable of displaying a list of rendered lines.
pub trait DisplayTarget {
    /// English RustDoc comment.
    /// Clears the target display.
    fn clear(&mut self);

    /// English RustDoc comment.
    /// Renders the provided lines on the target display.
    fn render_lines(&mut self, lines: &[String]);
}
