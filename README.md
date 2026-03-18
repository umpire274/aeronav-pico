# AeroNav Pico

A lightweight Rust project for decoding and displaying METAR and TAF weather reports, designed for small devices such as Picocalc.

## ✨ Features

* Unified METAR/TAF parsing via `metar-taf-parser`
* Natural language formatting (multi-language ready)
* Display-oriented text wrapping
* Label-aware rendering for small screens
* Pagination system for multi-page display
* CLI preview with interactive navigation

## 🧱 Architecture

The project is composed of two crates:

* **aeronav-core**

    * Parsing (via external crate)
    * Formatting (natural language)
    * Display wrapping
    * Pagination
    * Page navigation (`DocumentPager`)

* **aeronav-cli**

    * Desktop preview interface
    * Interactive navigation (`n` / `p` / `q`)
    * Rendering simulation for small displays

## 🚀 Example

Run the CLI preview:

```bash
cargo run -p aeronav-cli
```

Example output:

```
METAR LIRF
Time:          Day 12 at 12:50Z
Wind:          wind from 180° at 10
               kt
Visibility:    visibility greater
               than 10 km
Weather:       light rain
Clouds:        few clouds at 3000 ft
Temperature:   temperature 18°C,
               dew point 12°C
Pressure:      QNH 1015 hPa
Trend:         no significant change
```

Navigate pages using:

* `n` → next page
* `p` → previous page
* `q` → quit

## 🎯 Project Goal

This project aims to provide a minimal, efficient and portable weather decoding system suitable for:

* embedded devices (Picocalc)
* low-resolution displays
* keyboard-driven interfaces

## 📦 Status

🚧 **v0.1.0 — Initial working prototype**

* Core pipeline completed
* Display rendering ready
* Navigation implemented
* Ready for embedded integration

## 🔜 Next Steps

* Embedded input integration (hardware keys)
* Real-time METAR/TAF fetching
* UI state abstraction for firmware
* Multi-language expansion
* Performance optimizations for low-power devices