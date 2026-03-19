use crate::app::AppState;
use crate::display_target::DisplayTarget;
use crate::input::KeySource;
use crate::ui::{DisplayRenderer, FrameOptions, UiFrame};

/// English RustDoc comment.
/// Represents the result of a single application loop step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppStep {
    pub frame: UiFrame,
    pub lines: Vec<String>,
    pub running: bool,
}

/// English RustDoc comment.
/// Represents an application runner that coordinates input,
/// state updates and frame rendering.
#[derive(Debug)]
pub struct AppRunner<K: KeySource> {
    app: AppState,
    key_source: K,
    frame_options: FrameOptions,
    renderer: DisplayRenderer,
}

impl<K: KeySource> AppRunner<K> {
    /// English RustDoc comment.
    /// Creates a new application runner.
    pub fn new(
        app: AppState,
        key_source: K,
        frame_options: FrameOptions,
        renderer: DisplayRenderer,
    ) -> Self {
        Self {
            app,
            key_source,
            frame_options,
            renderer,
        }
    }

    /// English RustDoc comment.
    /// Returns true while the application should continue running.
    pub fn is_running(&self) -> bool {
        self.app.is_running()
    }

    /// English RustDoc comment.
    /// Renders the current application state without consuming input.
    pub fn render_current(&self) -> AppStep {
        let frame = self.app.render(&self.frame_options);
        let lines = self.renderer.render_lines(&frame);

        AppStep {
            frame,
            lines,
            running: self.app.is_running(),
        }
    }

    /// English RustDoc comment.
    /// Executes a single application loop step:
    /// poll key, map command, apply command, render updated state.
    pub fn step(&mut self) -> AppStep {
        let key = self.key_source.poll_key();
        let command = key.to_app_command();

        self.app.apply_command(command);

        let frame = self.app.render(&self.frame_options);
        let lines = self.renderer.render_lines(&frame);

        AppStep {
            frame,
            lines,
            running: self.app.is_running(),
        }
    }

    /// English RustDoc comment.
    /// Runs the application loop using the provided display target.
    pub fn run<D: DisplayTarget>(&mut self, display: &mut D) {
        while self.is_running() {
            display.clear();

            let current = self.render_current();
            display.render_lines(&current.lines);

            let step = self.step();

            if !step.running {
                break;
            }
        }
    }

    /// English RustDoc comment.
    /// Returns a shared reference to the application state.
    pub fn app(&self) -> &AppState {
        &self.app
    }

    /// English RustDoc comment.
    /// Returns a mutable reference to the application state.
    pub fn app_mut(&mut self) -> &mut AppState {
        &mut self.app
    }
}

#[cfg(test)]
mod tests {
    use super::AppRunner;
    use crate::app::AppState;
    use crate::display_target::DisplayTarget;
    use crate::input::{KeySource, PicoKey};
    use crate::ui::{DisplayRenderer, FrameOptions, ViewerConfig, WeatherViewer};
    use metar_taf_parser::Language;

    #[derive(Debug)]
    struct FixedKeySource {
        keys: Vec<PicoKey>,
        index: usize,
    }

    impl FixedKeySource {
        fn new(keys: Vec<PicoKey>) -> Self {
            Self { keys, index: 0 }
        }
    }

    impl KeySource for FixedKeySource {
        fn poll_key(&mut self) -> PicoKey {
            if self.index >= self.keys.len() {
                PicoKey::Unknown
            } else {
                let key = self.keys[self.index];
                self.index += 1;
                key
            }
        }
    }

    #[derive(Debug, Default)]
    struct TestDisplayTarget {
        rendered_frames: Vec<Vec<String>>,
        clear_count: usize,
    }

    impl DisplayTarget for TestDisplayTarget {
        fn clear(&mut self) {
            self.clear_count += 1;
        }

        fn render_lines(&mut self, lines: &[String]) {
            self.rendered_frames.push(lines.to_vec());
        }
    }

    fn sample_app() -> AppState {
        let viewer = WeatherViewer::new(
            "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG",
            Language::En,
            ViewerConfig::default(),
        )
        .unwrap();

        AppState::new(viewer)
    }

    /// English RustDoc comment.
    /// Verifies that the runner can render the current state.
    #[test]
    fn render_current_returns_lines() {
        let app = sample_app();
        let key_source = FixedKeySource::new(vec![]);
        let runner = AppRunner::new(
            app,
            key_source,
            FrameOptions::cli_default(),
            DisplayRenderer::cli_default(),
        );

        let step = runner.render_current();

        assert!(!step.lines.is_empty());
        assert!(step.running);
    }

    /// English RustDoc comment.
    /// Verifies that a quit key stops the application.
    #[test]
    fn quit_key_stops_runner() {
        let app = sample_app();
        let key_source = FixedKeySource::new(vec![PicoKey::Quit]);
        let mut runner = AppRunner::new(
            app,
            key_source,
            FrameOptions::cli_default(),
            DisplayRenderer::cli_default(),
        );

        let step = runner.step();

        assert!(!step.running);
        assert!(!runner.is_running());
    }

    /// English RustDoc comment.
    /// Verifies that the run loop drives the display target.
    #[test]
    fn run_renders_to_display_target() {
        let app = sample_app();
        let key_source = FixedKeySource::new(vec![PicoKey::Quit]);
        let mut runner = AppRunner::new(
            app,
            key_source,
            FrameOptions::cli_default(),
            DisplayRenderer::cli_default(),
        );
        let mut display = TestDisplayTarget::default();

        runner.run(&mut display);

        assert!(display.clear_count >= 1);
        assert!(!display.rendered_frames.is_empty());
    }
}
