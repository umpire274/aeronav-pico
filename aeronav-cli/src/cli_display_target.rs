use std::io::{self, Write};

use aeronav_core::display_target::DisplayTarget;

/// English RustDoc comment.
/// Represents a terminal display target for CLI rendering.
pub struct CliDisplayTarget;

impl CliDisplayTarget {
    /// English RustDoc comment.
    /// Creates a new CLI display target.
    pub fn new() -> Self {
        Self
    }
}

impl DisplayTarget for CliDisplayTarget {
    fn clear(&mut self) {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().expect("failed to flush stdout");
    }

    fn render_lines(&mut self, lines: &[String]) {
        for line in lines {
            println!("{line}");
        }
    }
}
