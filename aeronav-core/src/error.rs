use thiserror::Error;

/// English RustDoc comment.
/// Represents the errors returned by the AeroNav core library.
#[derive(Debug, Error)]
pub enum AeroNavError {
    /// Returned when a weather report cannot be parsed.
    #[error("failed to parse weather report: {0}")]
    Parse(String),
}
