/// English RustDoc comment.
/// Represents rendering options for a UI frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameOptions {
    pub show_header: bool,
    pub header_title: Option<String>,
    pub show_page_indicator: bool,
    pub footer_prompt: Option<String>,
}

impl FrameOptions {
    /// English RustDoc comment.
    /// Creates a new frame options instance.
    pub fn new(
        show_header: bool,
        header_title: Option<String>,
        show_page_indicator: bool,
        footer_prompt: Option<String>,
    ) -> Self {
        Self {
            show_header,
            header_title,
            show_page_indicator,
            footer_prompt,
        }
    }

    /// English RustDoc comment.
    /// Returns the default frame options for CLI usage.
    pub fn cli_default() -> Self {
        Self {
            show_header: true,
            header_title: Some("AeroNav CLI Preview".to_string()),
            show_page_indicator: true,
            footer_prompt: Some("[n] next  [p] previous  [q] quit".to_string()),
        }
    }

    /// English RustDoc comment.
    /// Returns compact frame options suitable for embedded displays.
    pub fn picocalc_default() -> Self {
        Self {
            show_header: false,
            header_title: None,
            show_page_indicator: true,
            footer_prompt: Some("[n] [p] [q]".to_string()),
        }
    }
}

/// English RustDoc comment.
/// Represents a renderable UI frame composed of header, content and footer sections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiFrame {
    pub header: Vec<String>,
    pub content: Vec<String>,
    pub footer: Vec<String>,
}

impl UiFrame {
    /// English RustDoc comment.
    /// Creates a new UI frame.
    pub fn new(header: Vec<String>, content: Vec<String>, footer: Vec<String>) -> Self {
        Self {
            header,
            content,
            footer,
        }
    }

    /// English RustDoc comment.
    /// Returns all frame lines in display order.
    pub fn lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        lines.extend(self.header.iter().cloned());
        lines.extend(self.content.iter().cloned());
        lines.extend(self.footer.iter().cloned());

        lines
    }

    /// English RustDoc comment.
    /// Returns true when all frame sections are empty.
    pub fn is_empty(&self) -> bool {
        self.header.is_empty() && self.content.is_empty() && self.footer.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{FrameOptions, UiFrame};

    /// English RustDoc comment.
    /// Verifies that frame lines are returned in header-content-footer order.
    #[test]
    fn lines_are_returned_in_order() {
        let frame = UiFrame::new(
            vec!["H1".to_string(), "H2".to_string()],
            vec!["C1".to_string()],
            vec!["F1".to_string()],
        );

        assert_eq!(
            frame.lines(),
            vec![
                "H1".to_string(),
                "H2".to_string(),
                "C1".to_string(),
                "F1".to_string()
            ]
        );
    }

    /// English RustDoc comment.
    /// Verifies empty frame detection.
    #[test]
    fn empty_frame_detection() {
        let frame = UiFrame::new(Vec::new(), Vec::new(), Vec::new());
        assert!(frame.is_empty());
    }

    /// English RustDoc comment.
    /// Verifies CLI default frame options.
    #[test]
    fn cli_default_options() {
        let options = FrameOptions::cli_default();

        assert!(options.show_header);
        assert!(options.header_title.is_some());
        assert!(options.show_page_indicator);
        assert!(options.footer_prompt.is_some());
    }

    /// English RustDoc comment.
    /// Verifies Picocalc default frame options.
    #[test]
    fn picocalc_default_options() {
        let options = FrameOptions::picocalc_default();

        assert!(!options.show_header);
        assert!(options.header_title.is_none());
        assert!(options.show_page_indicator);
        assert!(options.footer_prompt.is_some());
    }
}
