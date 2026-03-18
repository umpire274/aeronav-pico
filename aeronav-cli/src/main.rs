use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use aeronav_core::input::ViewerCommand;
use aeronav_core::layout::ViewerLayout;
use aeronav_core::viewer::{ViewerConfig, WeatherViewer};
use metar_taf_parser::Language;

/// English RustDoc comment.
/// Represents parsed CLI options for the preview application.
#[derive(Debug)]
struct CliOptions {
    language: Language,
    file_path: Option<String>,
    report_text: Option<String>,
    show_help: bool,
    show_version: bool,
    show_header: bool,
}

/// English RustDoc comment.
/// Entry point for the AeroNav CLI preview application.
fn main() {
    let options = match parse_cli_options() {
        Ok(options) => options,
        Err(err) => {
            eprintln!("Error: {err}");
            eprintln!();
            print_help();
            return;
        }
    };

    if options.show_help {
        print_help();
        return;
    }

    if options.show_version {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }

    let raw = match resolve_input_report(&options) {
        Ok(Some(report)) => report,
        Ok(None) => "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG".to_string(),
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    let base_config = ViewerConfig::default();

    let layout = if options.show_header {
        ViewerLayout::new(3, 1) // header + footer
    } else {
        ViewerLayout::new(0, 1)
    };

    match WeatherViewer::new_with_layout(&raw, options.language, base_config, layout) {
        Ok(mut viewer) => {
            if viewer.is_empty() {
                println!("No content to display.");
                return;
            }

            loop {
                clear_screen();

                if options.show_header {
                    print_header();
                    println!();
                }

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

                let command = ViewerCommand::from_input(&input);

                if !viewer.apply_command(command) {
                    break;
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }
}

/// English RustDoc comment.
/// Parses command-line options.
fn parse_cli_options() -> Result<CliOptions, String> {
    let mut args = env::args().skip(1).peekable();
    let mut options = CliOptions {
        language: Language::En,
        file_path: None,
        report_text: None,
        show_help: false,
        show_version: false,
        show_header: true,
    };

    let mut report_parts: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                options.show_help = true;
            }
            "-v" | "--version" => {
                options.show_version = true;
            }
            "--lang" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --lang".to_string())?;
                options.language = parse_language(&value);
            }
            "--file" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value for --file".to_string())?;
                options.file_path = Some(value);
            }
            "--no-header" => {
                options.show_header = false;
            }
            _ => report_parts.push(arg),
        }
    }

    if !report_parts.is_empty() {
        options.report_text = Some(report_parts.join(" "));
    }

    Ok(options)
}

/// English RustDoc comment.
/// Resolves the input weather report from CLI options.
fn resolve_input_report(options: &CliOptions) -> Result<Option<String>, String> {
    if let Some(path) = &options.file_path {
        let report = read_report_from_file(path)?;
        return Ok(Some(report));
    }

    Ok(options.report_text.clone())
}

/// English RustDoc comment.
/// Reads a weather report from a text file.
fn read_report_from_file(path: &str) -> Result<String, String> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        return Err(format!("file not found: {}", file_path.display()));
    }

    let content = fs::read_to_string(file_path)
        .map_err(|err| format!("failed to read '{}': {err}", file_path.display()))?;

    let trimmed = content.trim();

    if trimmed.is_empty() {
        return Err(format!("file is empty: {}", file_path.display()));
    }

    Ok(trimmed.to_string())
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
/// Prints the CLI help message.
fn print_help() {
    println!(
        "\
AeroNav Pico CLI Preview

USAGE:
    aeronav-cli [OPTIONS] [REPORT...]

OPTIONS:
    -h, --help           Show this help message
    -v, --version        Show application version
        --lang <LANG>    Select output language (currently: en)
        --file <PATH>    Read METAR/TAF report from file
        --no-header      Disable the CLI header

ARGS:
    [REPORT...]          Weather report passed directly as CLI arguments

EXAMPLES:
    aeronav-cli
    aeronav-cli -- METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG
    aeronav-cli --file report.txt
"
    );
}

/// English RustDoc comment.
/// Prints the optional CLI header.
fn print_header() {
    println!("AeroNav CLI Preview");
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

/// English RustDoc comment.
/// Clears the terminal screen using ANSI escape codes.
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().expect("failed to flush stdout");
}
