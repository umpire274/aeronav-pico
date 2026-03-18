use std::env;
use std::io::{self, Write};

use aeronav_core::viewer::{ViewerConfig, WeatherViewer};
use metar_taf_parser::Language;

/// English RustDoc comment.
/// Entry point for the AeroNav CLI preview application.
fn main() {
    let (language, raw) = parse_cli_input();

    let raw = raw.unwrap_or_else(|| {
        "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG".to_string()
    });

    let config = ViewerConfig::cli_default();

    match WeatherViewer::new(&raw, language, config) {
        Ok(mut viewer) => {
            if viewer.is_empty() {
                println!("No content to display.");
                return;
            }

            loop {
                clear_screen();

                let (current, total) = viewer.page_indicator();

                for line in viewer.current_lines() {
                    println!("{line}");
                }

                println!();
                print!(
                    "[n] next  [p] previous  [q] quit   ({}/{}) > ",
                    current, total
                );
                io::stdout().flush().expect("failed to flush stdout");

                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() {
                    eprintln!("Failed to read input.");
                    break;
                }

                match input.trim().to_ascii_lowercase().as_str() {
                    "n" => viewer.next_page(),
                    "p" => viewer.previous_page(),
                    "q" => break,
                    _ => {}
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

/// English RustDoc comment.
/// Parses CLI arguments and returns the selected language plus the optional report text.
///
/// Supported syntax:
/// - `aeronav-cli --lang en METAR ...`
/// - `aeronav-cli METAR ...`
fn parse_cli_input() -> (Language, Option<String>) {
    let mut args = env::args().skip(1).peekable();
    let mut language = Language::En;
    let mut report_parts: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        if arg == "--lang" {
            if let Some(value) = args.next() {
                language = parse_language(&value);
            }
        } else {
            report_parts.push(arg);
        }
    }

    let report = if report_parts.is_empty() {
        None
    } else {
        Some(report_parts.join(" "))
    };

    (language, report)
}

/// English RustDoc comment.
/// Parses a language code and falls back to English for unsupported values.
fn parse_language(value: &str) -> Language {
    match value.to_ascii_lowercase().as_str() {
        "en" => Language::En,
        _ => Language::En,
    }
}

/// English RustDoc comment.
/// Clears the terminal screen using ANSI escape codes.
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().expect("failed to flush stdout");
}
