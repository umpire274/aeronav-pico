use crate::app::AppCommand;

/// English RustDoc comment.
/// Represents an abstract key coming from a keyboard-like input source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PicoKey {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Back,
    Help,
    Quit,
    Unknown,
}

impl PicoKey {
    /// English RustDoc comment.
    /// Converts a CLI input string into an abstract key.
    pub fn from_cli_input(input: &str) -> Self {
        match input.trim().to_ascii_lowercase().as_str() {
            "n" => Self::Down,
            "p" => Self::Up,
            "e" => Self::Enter,
            "b" => Self::Back,
            "h" => Self::Help,
            "q" => Self::Quit,
            _ => Self::Unknown,
        }
    }

    /// English RustDoc comment.
    /// Maps an abstract key to an application command.
    pub fn to_app_command(self) -> AppCommand {
        match self {
            Self::Up => AppCommand::Previous,
            Self::Down => AppCommand::Next,
            Self::Left => AppCommand::Back,
            Self::Right => AppCommand::Select,
            Self::Enter => AppCommand::Select,
            Self::Back => AppCommand::Back,
            Self::Help => AppCommand::ShowHelp,
            Self::Quit => AppCommand::Quit,
            Self::Unknown => AppCommand::NoOp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PicoKey;
    use crate::app::AppCommand;

    /// English RustDoc comment.
    /// Verifies CLI input mapping to abstract keys.
    #[test]
    fn cli_input_maps_to_keys() {
        assert_eq!(PicoKey::from_cli_input("n"), PicoKey::Down);
        assert_eq!(PicoKey::from_cli_input("p"), PicoKey::Up);
        assert_eq!(PicoKey::from_cli_input("e"), PicoKey::Enter);
        assert_eq!(PicoKey::from_cli_input("b"), PicoKey::Back);
        assert_eq!(PicoKey::from_cli_input("h"), PicoKey::Help);
        assert_eq!(PicoKey::from_cli_input("q"), PicoKey::Quit);
        assert_eq!(PicoKey::from_cli_input("x"), PicoKey::Unknown);
    }

    /// English RustDoc comment.
    /// Verifies abstract key mapping to application commands.
    #[test]
    fn keys_map_to_app_commands() {
        assert_eq!(PicoKey::Up.to_app_command(), AppCommand::Previous);
        assert_eq!(PicoKey::Down.to_app_command(), AppCommand::Next);
        assert_eq!(PicoKey::Enter.to_app_command(), AppCommand::Select);
        assert_eq!(PicoKey::Back.to_app_command(), AppCommand::Back);
        assert_eq!(PicoKey::Help.to_app_command(), AppCommand::ShowHelp);
        assert_eq!(PicoKey::Quit.to_app_command(), AppCommand::Quit);
        assert_eq!(PicoKey::Unknown.to_app_command(), AppCommand::NoOp);
    }
}
