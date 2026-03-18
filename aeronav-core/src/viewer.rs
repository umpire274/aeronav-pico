use crate::display::{paginate_lines, wrap_labeled_text};
use crate::error::AeroNavError;
use crate::frame::{FrameOptions, UiFrame};
use crate::input::ViewerCommand;
use crate::layout::ViewerLayout;
use crate::pager::DocumentPager;
use crate::weather::model::WeatherDocument;
use crate::weather::service::decode_weather_report;
use metar_taf_parser::Language;

/// English RustDoc comment.
/// Represents the display configuration used by a weather viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewerConfig {
    pub width: usize,
    pub label_width: usize,
    pub page_height: usize,
}

impl ViewerConfig {
    /// English RustDoc comment.
    /// Creates a new viewer configuration.
    pub const fn new(width: usize, label_width: usize, page_height: usize) -> Self {
        Self {
            width,
            label_width,
            page_height,
        }
    }

    /// English RustDoc comment.
    /// Returns the default configuration for CLI usage.
    pub const fn cli_default() -> Self {
        Self {
            width: 36,
            label_width: 15,
            page_height: 8,
        }
    }

    /// English RustDoc comment.
    /// Returns the default configuration for Picocalc display.
    ///
    /// NOTE: values are provisional and may be adjusted
    /// once real hardware constraints are confirmed.
    pub const fn picocalc_default() -> Self {
        Self {
            width: 32,
            label_width: 14,
            page_height: 6,
        }
    }
}

impl Default for ViewerConfig {
    fn default() -> Self {
        // Keep Default aligned with CLI for now
        Self::cli_default()
    }
}

/// English RustDoc comment.
/// Represents a ready-to-display weather report viewer.
///
/// This type owns the decoded weather document and prepares its
/// formatted content for small displays using wrapping and pagination.
#[derive(Debug)]
pub struct WeatherViewer {
    document: WeatherDocument,
    pager: DocumentPager,
    config: ViewerConfig,
}

impl WeatherViewer {
    /// English RustDoc comment.
    /// Creates a new weather viewer from a raw METAR or TAF string.
    pub fn new(raw: &str, language: Language, config: ViewerConfig) -> Result<Self, AeroNavError> {
        let document = decode_weather_report(raw, language)?;
        let lines = wrap_labeled_text(&document.formatted_text, config.width, config.label_width);
        let pages = paginate_lines(&lines, config.page_height);
        let pager = DocumentPager::new(pages);

        Ok(Self {
            document,
            pager,
            config,
        })
    }

    /// English RustDoc comment.
    /// Creates a viewer using a layout-aware configuration.
    pub fn new_with_layout(
        raw: &str,
        language: Language,
        base_config: ViewerConfig,
        layout: ViewerLayout,
    ) -> Result<Self, AeroNavError> {
        let page_height = layout.compute_page_height(base_config.page_height);

        let config = ViewerConfig {
            width: base_config.width,
            label_width: base_config.label_width,
            page_height,
        };

        Self::new(raw, language, config)
    }

    /// English RustDoc comment.
    /// Applies a viewer command and returns `false` when the caller should quit.
    pub fn apply_command(&mut self, command: ViewerCommand) -> bool {
        match command {
            ViewerCommand::NextPage => {
                self.next_page();
                true
            }
            ViewerCommand::PreviousPage => {
                self.previous_page();
                true
            }
            ViewerCommand::Quit => false,
            ViewerCommand::NoOp => true,
        }
    }

    /// English RustDoc comment.
    /// Renders the current viewer state into a UI frame.
    pub fn render_frame(&self, options: &FrameOptions) -> UiFrame {
        let mut header = Vec::new();

        if options.show_header {
            if let Some(title) = &options.header_title {
                header.push(title.clone());
            }

            header.push(format!(
                "{} v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ));
            header.push(String::new());
        }

        let content = self.current_lines().to_vec();

        let mut footer = Vec::new();

        let (current, total) = self.page_indicator();

        match (&options.footer_prompt, options.show_page_indicator) {
            (Some(prompt), true) => {
                footer.push(format!("{prompt}   ({current}/{total})"));
            }
            (Some(prompt), false) => {
                footer.push(prompt.clone());
            }
            (None, true) => {
                footer.push(format!("({current}/{total})"));
            }
            (None, false) => {}
        }

        UiFrame::new(header, content, footer)
    }

    /// English RustDoc comment.
    /// Returns the station identifier.
    pub fn station(&self) -> &str {
        self.document.station()
    }

    /// English RustDoc comment.
    /// Returns the report kind.
    pub fn report_kind(&self) -> &'static str {
        self.document.report_kind()
    }

    /// English RustDoc comment.
    /// Returns the current page lines.
    pub fn current_lines(&self) -> &[String] {
        self.pager.current_page()
    }

    /// English RustDoc comment.
    /// Moves to the next page if possible.
    pub fn next_page(&mut self) {
        self.pager.next();
    }

    /// English RustDoc comment.
    /// Moves to the previous page if possible.
    pub fn previous_page(&mut self) {
        self.pager.previous();
    }

    /// English RustDoc comment.
    /// Returns the current page indicator as `(current, total)`.
    pub fn page_indicator(&self) -> (usize, usize) {
        self.pager.indicator()
    }

    /// English RustDoc comment.
    /// Returns true if the viewer contains no displayable pages.
    pub fn is_empty(&self) -> bool {
        self.pager.is_empty()
    }

    /// English RustDoc comment.
    /// Returns the viewer configuration.
    pub fn config(&self) -> ViewerConfig {
        self.config
    }

    /// English RustDoc comment.
    /// Returns the original formatted text.
    pub fn formatted_text(&self) -> &str {
        &self.document.formatted_text
    }
}

#[cfg(test)]
mod tests {
    use super::{ViewerConfig, WeatherViewer};
    use crate::frame::FrameOptions;
    use crate::input::ViewerCommand;
    use metar_taf_parser::Language;

    /// English RustDoc comment.
    /// Verifies that the default configuration matches the CLI default.
    #[test]
    fn default_config_values() {
        let default_config = ViewerConfig::default();
        let cli_config = ViewerConfig::cli_default();

        assert_eq!(default_config, cli_config);
    }

    /// English RustDoc comment.
    /// Verifies the provisional Picocalc configuration values.
    #[test]
    fn picocalc_config_values() {
        let config = ViewerConfig::picocalc_default();

        assert_eq!(config.width, 32);
        assert_eq!(config.label_width, 14);
        assert_eq!(config.page_height, 6);
    }

    /// English RustDoc comment.
    /// Verifies that a METAR viewer can be created successfully.
    #[test]
    fn create_metar_viewer() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

        let viewer = WeatherViewer::new(raw, Language::En, ViewerConfig::default()).unwrap();

        assert_eq!(viewer.station(), "LIRF");
        assert_eq!(viewer.report_kind(), "METAR");
        assert!(!viewer.is_empty());
        assert_eq!(viewer.page_indicator(), (1, 2));
    }

    /// English RustDoc comment.
    /// Verifies that page navigation works correctly.
    #[test]
    fn viewer_navigation() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

        let mut viewer = WeatherViewer::new(raw, Language::En, ViewerConfig::default()).unwrap();

        assert_eq!(viewer.page_indicator(), (1, 2));

        viewer.next_page();
        assert_eq!(viewer.page_indicator(), (2, 2));

        viewer.previous_page();
        assert_eq!(viewer.page_indicator(), (1, 2));
    }

    /// English RustDoc comment.
    /// Verifies that the viewer exposes non-empty current lines.
    #[test]
    fn viewer_current_lines_are_available() {
        let raw = "TAF LIRF 121100Z 1212/1318 18010KT 9999 SCT020";

        let viewer = WeatherViewer::new(raw, Language::En, ViewerConfig::default()).unwrap();

        assert_eq!(viewer.report_kind(), "TAF");
        assert!(!viewer.current_lines().is_empty());
    }

    /// English RustDoc comment.
    /// Verifies that a custom configuration is preserved.
    #[test]
    fn viewer_preserves_custom_config() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";
        let config = ViewerConfig::new(40, 16, 8);

        let viewer = WeatherViewer::new(raw, Language::En, config).unwrap();

        assert_eq!(viewer.config(), config);
    }

    /// English RustDoc comment.
    /// Verifies that apply_command advances to the next page.
    #[test]
    fn apply_next_command() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

        let mut viewer = WeatherViewer::new(raw, Language::En, ViewerConfig::default()).unwrap();

        assert_eq!(viewer.page_indicator(), (1, 2));

        let keep_running = viewer.apply_command(ViewerCommand::NextPage);

        assert!(keep_running);
        assert_eq!(viewer.page_indicator(), (2, 2));
    }

    /// English RustDoc comment.
    /// Verifies that apply_command handles quit requests.
    #[test]
    fn apply_quit_command() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

        let mut viewer = WeatherViewer::new(raw, Language::En, ViewerConfig::default()).unwrap();

        let keep_running = viewer.apply_command(ViewerCommand::Quit);

        assert!(!keep_running);
    }

    /// English RustDoc comment.
    /// Verifies frame rendering with CLI options.
    #[test]
    fn render_frame_with_cli_options() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

        let viewer = WeatherViewer::new(raw, Language::En, ViewerConfig::default()).unwrap();
        let frame = viewer.render_frame(&FrameOptions::cli_default());

        assert!(!frame.header.is_empty());
        assert!(!frame.content.is_empty());
        assert_eq!(frame.footer.len(), 1);
    }

    /// English RustDoc comment.
    /// Verifies frame rendering with Picocalc-style options.
    #[test]
    fn render_frame_with_picocalc_options() {
        let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

        let viewer = WeatherViewer::new(raw, Language::En, ViewerConfig::default()).unwrap();
        let frame = viewer.render_frame(&FrameOptions::picocalc_default());

        assert!(frame.header.is_empty());
        assert!(!frame.content.is_empty());
        assert_eq!(frame.footer.len(), 1);
    }
}
