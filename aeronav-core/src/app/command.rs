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
