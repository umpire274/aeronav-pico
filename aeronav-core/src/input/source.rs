use crate::input::PicoKey;

/// English RustDoc comment.
/// Represents a source of abstract keys for the application.
pub trait KeySource {
    /// English RustDoc comment.
    /// Polls the next available key.
    fn poll_key(&mut self) -> PicoKey;
}
