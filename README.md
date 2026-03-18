# AeroNav Pico

![CI](https://github.com/umpire274/aeronav-pico/actions/workflows/rust.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Rust](https://img.shields.io/badge/Rust-2024-orange)
![Status](https://img.shields.io/badge/status-active-success)
![Version](https://img.shields.io/github/v/release/umpire274/aeronav-pico)
![Issues](https://img.shields.io/github/issues/umpire274/aeronav-pico)
![PRs](https://img.shields.io/github/issues-pr/umpire274/aeronav-pico)

A lightweight Rust project for decoding and displaying METAR and TAF weather reports, designed for small devices such as Picocalc.

## ✨ Features

* Unified METAR/TAF parsing via `metar-taf-parser`
* Natural language formatting
* Display-oriented text wrapping
* Label-aware rendering for small screens
* Pagination system for multi-page display
* High-level `WeatherViewer` abstraction
* CLI preview with interactive navigation

## 🧱 Architecture

The project is composed of two crates:

* **aeronav-core**

    * Parsing (via external crate)
    * Formatting (natural language)
    * Display wrapping
    * Pagination
    * Page navigation (`DocumentPager`)
    * High-level viewer abstraction (`WeatherViewer`)
    * Viewer layout configuration (`ViewerConfig`)

* **aeronav-cli**

    * Desktop preview interface
    * Interactive navigation (`n` / `p` / `q`)
    * Rendering simulation for small displays
    * Argument-based report input

## 🚀 Example

Run the CLI preview:

```bash
cargo run -p aeronav-cli
```

Pass a custom METAR report:

```bash
cargo run -p aeronav-cli -- METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG
```

## 🧭 Navigation

* `n` → next page
* `p` → previous page
* `q` → quit

## 🎯 Project Goal

This project aims to provide a minimal, efficient and portable weather decoding system suitable for:

* embedded devices (Picocalc)
* low-resolution displays
* keyboard-driven interfaces

## 📦 Status

🚀 **v0.2.0 — Viewer abstraction and architecture stabilization**

* High-level viewer introduced
* Configuration system implemented
* CLI refactored on top of core
* Ready for embedded integration

## 🔜 Next Steps

* Input from file (`--file`)
* Extended CLI argument handling
* Hardware input abstraction (Picocalc keys)
* Multi-language support (when available upstream)

## 📄 License

This project is licensed under either of:

* MIT License ([LICENSE-MIT](LICENSE-MIT))
* Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
