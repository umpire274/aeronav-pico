use crate::key_source::KeySource;
use crate::keys::PicoKey;

/// English RustDoc comment.
/// Represents a placeholder key source for the future Picocalc hardware keyboard.
///
/// This is currently a stub and always returns `PicoKey::Unknown`.
#[derive(Debug, Default)]
pub struct PicocalcKeySource;

impl PicocalcKeySource {
    /// English RustDoc comment.
    /// Creates a new Picocalc key source stub.
    pub fn new() -> Self {
        Self
    }
}

impl KeySource for PicocalcKeySource {
    fn poll_key(&mut self) -> PicoKey {
        PicoKey::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::PicocalcKeySource;
    use crate::key_source::KeySource;
    use crate::keys::PicoKey;

    /// English RustDoc comment.
    /// Verifies that the current Picocalc key source stub returns Unknown.
    #[test]
    fn stub_returns_unknown() {
        let mut source = PicocalcKeySource::new();
        assert_eq!(source.poll_key(), PicoKey::Unknown);
    }
}
