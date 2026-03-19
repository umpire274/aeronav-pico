use crate::frame::{FrameOptions, UiFrame};
use crate::viewer::WeatherViewer;

/// English RustDoc comment.
/// Represents the currently active application screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Menu,
    Viewer,
    Help,
    About,
}

/// English RustDoc comment.
/// Represents a selectable entry in the main menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuEntry {
    OpenViewer,
    Help,
    About,
    Quit,
}

impl MenuEntry {
    /// English RustDoc comment.
    /// Returns all menu entries in display order.
    fn all() -> [Self; 4] {
        [Self::OpenViewer, Self::Help, Self::About, Self::Quit]
    }

    /// English RustDoc comment.
    /// Returns the label shown in the menu.
    fn label(self) -> &'static str {
        match self {
            Self::OpenViewer => "Open viewer",
            Self::Help => "Help",
            Self::About => "About",
            Self::Quit => "Quit",
        }
    }
}

/// English RustDoc comment.
/// Represents a high-level application command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCommand {
    Next,
    Previous,
    Select,
    ShowHelp,
    Back,
    Quit,
    NoOp,
}

impl AppCommand {}

/// English RustDoc comment.
/// Represents the top-level application state.
#[derive(Debug)]
pub struct AppState {
    viewer: WeatherViewer,
    current_screen: AppScreen,
    previous_screen: Option<AppScreen>,
    menu_index: usize,
    running: bool,
}

impl AppState {
    /// English RustDoc comment.
    /// Creates a new application state from a weather viewer.
    pub fn new(viewer: WeatherViewer) -> Self {
        Self {
            viewer,
            current_screen: AppScreen::Menu,
            previous_screen: None,
            menu_index: 0,
            running: true,
        }
    }

    /// English RustDoc comment.
    /// Returns true while the application should continue running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// English RustDoc comment.
    /// Returns the currently active screen.
    pub fn current_screen(&self) -> AppScreen {
        self.current_screen
    }

    /// English RustDoc comment.
    /// Applies an application command to the current state.
    pub fn apply_command(&mut self, command: AppCommand) {
        match self.current_screen {
            AppScreen::Menu => self.apply_menu_command(command),
            AppScreen::Viewer => self.apply_viewer_command(command),
            AppScreen::Help => self.apply_help_command(command),
            AppScreen::About => self.apply_about_command(command),
        }
    }

    /// English RustDoc comment.
    /// Renders the current application frame.
    pub fn render(&self, options: &FrameOptions) -> UiFrame {
        match self.current_screen {
            AppScreen::Menu => self.render_menu_frame(options),
            AppScreen::Viewer => self.viewer.render_frame(options),
            AppScreen::Help => self.render_help_frame(options),
            AppScreen::About => self.render_about_frame(options),
        }
    }

    /// English RustDoc comment.
    /// Applies a command while the menu screen is active.
    fn apply_menu_command(&mut self, command: AppCommand) {
        let entries = MenuEntry::all();

        match command {
            AppCommand::Next => {
                if self.menu_index + 1 < entries.len() {
                    self.menu_index += 1;
                }
            }
            AppCommand::Previous => {
                self.menu_index = self.menu_index.saturating_sub(1);
            }
            AppCommand::Select => match entries[self.menu_index] {
                MenuEntry::OpenViewer => {
                    self.current_screen = AppScreen::Viewer;
                    self.previous_screen = Some(AppScreen::Menu);
                }
                MenuEntry::Help => {
                    self.previous_screen = Some(AppScreen::Menu);
                    self.current_screen = AppScreen::Help;
                }
                MenuEntry::About => {
                    self.previous_screen = Some(AppScreen::Menu);
                    self.current_screen = AppScreen::About;
                }
                MenuEntry::Quit => {
                    self.running = false;
                }
            },
            AppCommand::ShowHelp => {
                self.previous_screen = Some(AppScreen::Menu);
                self.current_screen = AppScreen::Help;
            }
            AppCommand::Back => {}
            AppCommand::Quit => {
                self.running = false;
            }
            AppCommand::NoOp => {}
        }
    }

    /// English RustDoc comment.
    /// Applies a command while the viewer screen is active.
    fn apply_viewer_command(&mut self, command: AppCommand) {
        match command {
            AppCommand::Next => self.viewer.next_page(),
            AppCommand::Previous => self.viewer.previous_page(),
            AppCommand::ShowHelp => {
                self.previous_screen = Some(AppScreen::Viewer);
                self.current_screen = AppScreen::Help;
            }
            AppCommand::Back => {
                self.current_screen = AppScreen::Menu;
                self.previous_screen = Some(AppScreen::Viewer);
            }
            AppCommand::Select => {}
            AppCommand::Quit => self.running = false,
            AppCommand::NoOp => {}
        }
    }

    /// English RustDoc comment.
    /// Applies a command while the help screen is active.
    fn apply_help_command(&mut self, command: AppCommand) {
        match command {
            AppCommand::Back => {
                self.current_screen = self.previous_screen.unwrap_or(AppScreen::Menu);
                self.previous_screen = None;
            }
            AppCommand::Quit => self.running = false,
            AppCommand::ShowHelp => {}
            AppCommand::Next => {}
            AppCommand::Previous => {}
            AppCommand::Select => {}
            AppCommand::NoOp => {}
        }
    }

    /// English RustDoc comment.
    /// Applies a command while the about screen is active.
    fn apply_about_command(&mut self, command: AppCommand) {
        match command {
            AppCommand::Back => {
                self.current_screen = self.previous_screen.unwrap_or(AppScreen::Menu);
                self.previous_screen = None;
            }
            AppCommand::Quit => self.running = false,
            AppCommand::ShowHelp => {
                self.previous_screen = Some(AppScreen::About);
                self.current_screen = AppScreen::Help;
            }
            AppCommand::Next => {}
            AppCommand::Previous => {}
            AppCommand::Select => {}
            AppCommand::NoOp => {}
        }
    }

    /// English RustDoc comment.
    /// Renders the main menu frame.
    fn render_menu_frame(&self, options: &FrameOptions) -> UiFrame {
        let mut header = Vec::new();

        if options.show_header {
            if let Some(title) = &options.header_title {
                header.push(title.clone());
            }
            header.push("Main Menu".to_string());
            header.push(String::new());
        }

        let content = MenuEntry::all()
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                if index == self.menu_index {
                    format!("> {}", entry.label())
                } else {
                    format!("  {}", entry.label())
                }
            })
            .collect();

        let footer = vec!["[n] next  [p] prev  [e] select  [q] quit".to_string()];

        UiFrame::new(header, content, footer)
    }

    /// English RustDoc comment.
    /// Renders the help screen frame.
    fn render_help_frame(&self, options: &FrameOptions) -> UiFrame {
        let mut header = Vec::new();

        if options.show_header {
            if let Some(title) = &options.header_title {
                header.push(title.clone());
            }
            header.push("Help".to_string());
            header.push(String::new());
        }

        let content = vec![
            "AeroNav controls:".to_string(),
            String::new(),
            "n  next page / next item".to_string(),
            "p  previous page / item".to_string(),
            "e  select menu item".to_string(),
            "h  open help".to_string(),
            "b  back".to_string(),
            "q  quit".to_string(),
        ];

        let footer = vec!["[b] back  [q] quit".to_string()];

        UiFrame::new(header, content, footer)
    }

    /// English RustDoc comment.
    /// Renders the about screen frame.
    fn render_about_frame(&self, options: &FrameOptions) -> UiFrame {
        let mut header = Vec::new();

        if options.show_header {
            if let Some(title) = &options.header_title {
                header.push(title.clone());
            }
            header.push("About".to_string());
            header.push(String::new());
        }

        let content = vec![
            "AeroNav Pico".to_string(),
            format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            String::new(),
            "A lightweight Rust-based".to_string(),
            "weather report viewer for".to_string(),
            "CLI and embedded targets.".to_string(),
        ];

        let footer = vec!["[b] back  [q] quit".to_string()];

        UiFrame::new(header, content, footer)
    }
}

#[cfg(test)]
mod tests {
    use super::{AppCommand, AppScreen, AppState};
    use crate::frame::FrameOptions;
    use crate::viewer::{ViewerConfig, WeatherViewer};
    use metar_taf_parser::Language;

    fn sample_viewer() -> WeatherViewer {
        WeatherViewer::new(
            "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG",
            Language::En,
            ViewerConfig::default(),
        )
        .unwrap()
    }

    /// English RustDoc comment.
    /// Verifies that a new application state is running.
    #[test]
    fn app_state_starts_running() {
        let app = AppState::new(sample_viewer());
        assert!(app.is_running());
    }

    /// English RustDoc comment.
    /// Verifies that the initial screen is the menu screen.
    #[test]
    fn app_state_starts_on_menu_screen() {
        let app = AppState::new(sample_viewer());
        assert_eq!(app.current_screen(), AppScreen::Menu);
    }

    /// English RustDoc comment.
    /// Verifies that quit stops the application.
    #[test]
    fn quit_command_stops_application() {
        let mut app = AppState::new(sample_viewer());

        app.apply_command(AppCommand::Quit);

        assert!(!app.is_running());
    }

    /// English RustDoc comment.
    /// Verifies that rendering returns a non-empty frame.
    #[test]
    fn render_returns_frame() {
        let app = AppState::new(sample_viewer());
        let frame = app.render(&FrameOptions::cli_default());

        assert!(!frame.is_empty());
    }

    /// English RustDoc comment.
    /// Verifies that help can be opened from the menu.
    #[test]
    fn menu_help_switches_screen() {
        let mut app = AppState::new(sample_viewer());

        app.apply_command(AppCommand::ShowHelp);

        assert_eq!(app.current_screen(), AppScreen::Help);
    }

    /// English RustDoc comment.
    /// Verifies that back returns from help to menu.
    #[test]
    fn help_back_returns_to_menu() {
        let mut app = AppState::new(sample_viewer());

        app.apply_command(AppCommand::ShowHelp);
        app.apply_command(AppCommand::Back);

        assert_eq!(app.current_screen(), AppScreen::Menu);
    }

    /// English RustDoc comment.
    /// Verifies that selecting the first menu item opens the viewer.
    #[test]
    fn select_menu_opens_viewer() {
        let mut app = AppState::new(sample_viewer());

        app.apply_command(AppCommand::Select);

        assert_eq!(app.current_screen(), AppScreen::Viewer);
    }
}
