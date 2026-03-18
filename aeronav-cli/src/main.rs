use std::io::{self, Write};

use aeronav_core::display::{paginate_lines, wrap_labeled_text};
use aeronav_core::pager::DocumentPager;
use aeronav_core::weather::service::decode_weather_report;
use metar_taf_parser::Language;

/// English RustDoc comment.
/// Entry point for the AeroNav CLI preview application.
fn main() {
    let raw = "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG";

    match decode_weather_report(raw, Language::En) {
        Ok(document) => {
            let lines = wrap_labeled_text(&document.formatted_text, 36, 15);
            let pages = paginate_lines(&lines, 6);
            let mut pager = DocumentPager::new(pages);

            if pager.is_empty() {
                println!("No content to display.");
                return;
            }

            loop {
                clear_screen();

                let (current, total) = pager.indicator();

                println!("Kind: {}", document.report_kind());
                println!("Station: {}", document.station());
                println!();

                for line in pager.current_page() {
                    println!("{line}");
                }

                println!();
                print!("[n] next  [p] prev  [q] quit   ({}/{}) > ", current, total);
                io::stdout().flush().expect("failed to flush stdout");

                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() {
                    eprintln!("Failed to read input.");
                    break;
                }

                match input.trim().to_ascii_lowercase().as_str() {
                    "n" => pager.next(),
                    "p" => pager.previous(),
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
/// Clears the terminal screen using ANSI escape codes.
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().expect("failed to flush stdout");
}
