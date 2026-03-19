use crate::app::{AppRunner, AppState};
use crate::display_target::DisplayTarget;
use crate::error::AeroNavError;
use crate::input::PicocalcKeySource;
use crate::ui::{DisplayRenderer, FrameOptions, ViewerConfig, ViewerLayout, WeatherViewer};
use metar_taf_parser::Language;

/// English RustDoc comment.
/// Represents a stub runtime configuration for Picocalc-oriented execution.
#[derive(Debug)]
pub struct PicocalcRuntime {
    runner: AppRunner<PicocalcKeySource>,
}

impl PicocalcRuntime {
    /// English RustDoc comment.
    /// Creates a new Picocalc runtime from a raw weather report.
    pub fn new(raw: &str, language: Language) -> Result<Self, AeroNavError> {
        let base_config = ViewerConfig::picocalc_default();
        let layout = ViewerLayout::new(0, 1);

        let viewer = WeatherViewer::new_with_layout(raw, language, base_config, layout)?;
        let app = AppState::new(viewer);
        let key_source = PicocalcKeySource::new();
        let frame_options = FrameOptions::picocalc_default();
        let renderer = DisplayRenderer::picocalc_default();

        let runner = AppRunner::new(app, key_source, frame_options, renderer);

        Ok(Self { runner })
    }

    /// English RustDoc comment.
    /// Returns true while the runtime should continue running.
    pub fn is_running(&self) -> bool {
        self.runner.is_running()
    }

    /// English RustDoc comment.
    /// Renders the current state into display-ready lines.
    pub fn render_lines(&self) -> Vec<String> {
        self.runner.render_current().lines
    }

    /// English RustDoc comment.
    /// Executes a single runtime step.
    pub fn step(&mut self) -> bool {
        self.runner.step().running
    }

    /// English RustDoc comment.
    /// Runs the Picocalc runtime using the provided display target.
    pub fn run<D: DisplayTarget>(&mut self, display: &mut D) {
        self.runner.run(display);
    }

    /// English RustDoc comment.
    /// Returns a shared reference to the underlying runner.
    pub fn runner(&self) -> &AppRunner<PicocalcKeySource> {
        &self.runner
    }

    /// English RustDoc comment.
    /// Returns a mutable reference to the underlying runner.
    pub fn runner_mut(&mut self) -> &mut AppRunner<PicocalcKeySource> {
        &mut self.runner
    }
}

#[cfg(test)]
mod tests {
    use super::PicocalcRuntime;
    use metar_taf_parser::Language;

    /// English RustDoc comment.
    /// Verifies that the Picocalc runtime can be created successfully.
    #[test]
    fn create_picocalc_runtime() {
        let runtime = PicocalcRuntime::new(
            "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG",
            Language::En,
        )
        .unwrap();

        assert!(runtime.is_running());
    }

    /// English RustDoc comment.
    /// Verifies that the Picocalc runtime renders display lines.
    #[test]
    fn picocalc_runtime_renders_lines() {
        let runtime = PicocalcRuntime::new(
            "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG",
            Language::En,
        )
        .unwrap();

        let lines = runtime.render_lines();

        assert!(!lines.is_empty());
        assert!(lines.len() <= 8);
    }

    /// English RustDoc comment.
    /// Verifies that a runtime step keeps running with the current stub key source.
    #[test]
    fn picocalc_runtime_step_keeps_running() {
        let mut runtime = PicocalcRuntime::new(
            "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG",
            Language::En,
        )
        .unwrap();

        let running = runtime.step();

        assert!(running);
        assert!(runtime.is_running());
    }

    /// English RustDoc comment.
    /// Verifies that the runtime can drive a display target.
    #[test]
    fn picocalc_runtime_render_lines_produces_output() {
        let runtime = PicocalcRuntime::new(
            "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG",
            Language::En,
        )
        .unwrap();

        let lines = runtime.render_lines();

        assert!(!lines.is_empty());
    }
}
