/// English RustDoc comment.
/// Represents a navigation command for the weather viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerCommand {
    NextPage,
    PreviousPage,
    Quit,
    NoOp,
}

impl ViewerCommand {
    /// English RustDoc comment.
    /// Parses a viewer command from user input.
    pub fn from_input(input: &str) -> Self {
        match input.trim().to_ascii_lowercase().as_str() {
            "n" => Self::NextPage,
            "p" => Self::PreviousPage,
            "q" => Self::Quit,
            _ => Self::NoOp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ViewerCommand;

    /// English RustDoc comment.
    /// Verifies parsing of the next-page command.
    #[test]
    fn parse_next_command() {
        assert_eq!(ViewerCommand::from_input("n"), ViewerCommand::NextPage);
        assert_eq!(ViewerCommand::from_input("N"), ViewerCommand::NextPage);
    }

    /// English RustDoc comment.
    /// Verifies parsing of the previous-page command.
    #[test]
    fn parse_previous_command() {
        assert_eq!(ViewerCommand::from_input("p"), ViewerCommand::PreviousPage);
        assert_eq!(ViewerCommand::from_input("P"), ViewerCommand::PreviousPage);
    }

    /// English RustDoc comment.
    /// Verifies parsing of the quit command.
    #[test]
    fn parse_quit_command() {
        assert_eq!(ViewerCommand::from_input("q"), ViewerCommand::Quit);
        assert_eq!(ViewerCommand::from_input("Q"), ViewerCommand::Quit);
    }

    /// English RustDoc comment.
    /// Verifies fallback to NoOp for unsupported input.
    #[test]
    fn parse_unknown_command() {
        assert_eq!(ViewerCommand::from_input("x"), ViewerCommand::NoOp);
        assert_eq!(ViewerCommand::from_input(""), ViewerCommand::NoOp);
        assert_eq!(ViewerCommand::from_input("next"), ViewerCommand::NoOp);
    }
}
