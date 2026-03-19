/// English RustDoc comment.
/// Represents the currently active application screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Menu,
    Viewer,
    Help,
    About,
}
