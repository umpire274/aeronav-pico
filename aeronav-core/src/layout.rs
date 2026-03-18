/// English RustDoc comment.
/// Represents UI layout constraints for the viewer.
#[derive(Debug, Clone, Copy)]
pub struct ViewerLayout {
    pub header_lines: usize,
    pub footer_lines: usize,
}

impl ViewerLayout {
    /// English RustDoc comment.
    /// Creates a new layout configuration.
    pub const fn new(header_lines: usize, footer_lines: usize) -> Self {
        Self {
            header_lines,
            footer_lines,
        }
    }

    /// English RustDoc comment.
    /// Computes the effective page height based on total height.
    pub fn compute_page_height(&self, total_height: usize) -> usize {
        let reserved = self.header_lines + self.footer_lines;
        total_height.saturating_sub(reserved)
    }
}
