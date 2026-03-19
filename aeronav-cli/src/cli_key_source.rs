use aeronav_core::input::{KeySource, PicoKey};
use std::io;

/// English RustDoc comment.
/// Represents a CLI-based key source backed by stdin.
pub struct CliKeySource;

impl CliKeySource {
    /// English RustDoc comment.
    /// Creates a new CLI key source.
    pub fn new() -> Self {
        Self
    }
}

impl KeySource for CliKeySource {
    fn poll_key(&mut self) -> PicoKey {
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            return PicoKey::Unknown;
        }

        PicoKey::from_cli_input(&input)
    }
}
