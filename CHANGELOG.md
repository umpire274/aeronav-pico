# Changelog

# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2026-03-19

### ✨ Added

- Introduced `AppState`, `AppScreen`, and `AppCommand`
- Added multi-screen navigation (Menu, Viewer, Help, About)
- Introduced `PicoKey` abstraction
- Added `KeySource` trait
- Implemented `CliKeySource` and `PicocalcKeySource` (stub)
- Introduced `DisplayRenderer`

### 🔄 Changed

- Refactored CLI to use application state instead of direct viewer access
- Removed direct input → command mapping in favor of `PicoKey`

### 🏗️ Internal

- Established full input → state → render pipeline
- Prepared architecture for embedded hardware execution

---

## [0.4.0] - 2026-03-18

### ✨ Added

* Introduced `UiFrame` abstraction (header, content, footer)
* Introduced `FrameOptions` for configurable rendering
* Added `render_frame()` in `WeatherViewer`
* Added CLI and Picocalc default frame profiles
* Added automated multi-platform release pipeline
* Added SHA256 checksum generation

### 🔄 Changed

* Refactored CLI to use `UiFrame`
* Moved rendering logic from CLI to `aeronav-core`
* Improved separation of concerns across modules
* Simplified main loop and rendering pipeline

### 🧪 Tests

* Added tests for `UiFrame`
* Added tests for `FrameOptions`
* Updated viewer tests for new rendering model

### 🏗️ Internal

* Improved modular architecture
* Prepared project for embedded display targets

---

## [0.3.0] - 2026-03-18

### ✨ Features

- Added file-based input via `--file`
- Added `--help` and `--version` support
- Added optional CLI header with `--no-header`
- Introduced `ViewerCommand` for viewer interaction
- Added `WeatherViewer::apply_command()` to move command handling into the core
- Added `ViewerLayout` to model UI layout constraints
- Added `WeatherViewer::new_with_layout()` for layout-aware viewer creation

### 🖥️ CLI Improvements

- Improved argument parsing without external dependencies
- Added support for direct METAR/TAF input via positional arguments
- Preserved demo fallback behavior
- Improved interactive navigation workflow

### 🧠 Architecture

- Moved layout computation from CLI into `aeronav-core`
- Improved separation between input handling, layout logic and viewer behavior
- Prepared the project for embedded environments (Picocalc)

---

## [0.2.0] - 2026-03-18

### ✨ Features

- Introduced `WeatherViewer` as a high-level display-ready abstraction
- Added `ViewerConfig` to centralize viewer layout settings
- Added configurable defaults for CLI and future Picocalc integration
- Moved wrapping, pagination and navigation pipeline behind a reusable core API

### 🧱 Architecture

- Refactored the application flow so frontends consume `WeatherViewer`
  instead of manually orchestrating parsing, wrapping and paging
- Reduced CLI responsibilities to input/output and user interaction only
- Improved separation between core logic and presentation layer

### 🖥 CLI

- Updated CLI to use `WeatherViewer`
- Simplified interactive page navigation using the new viewer abstraction
- Removed pipe-based input support to avoid conflicts with interactive navigation
- Kept argument-based report input and demo fallback behavior

### 🧪 Testing

- Added unit tests for `WeatherViewer`
- Added tests for viewer configuration and navigation behavior

--

## [0.1.0] - 2026-03-17

### ✨ Features

- Initial implementation of AeroNav Pico core
- Unified METAR/TAF parsing via `metar-taf-parser`
- Natural language weather report formatting
- Display-oriented text wrapping
- Label-aware rendering for small screens
- Pagination system for multi-page display
- Core page navigation via `DocumentPager`

### 🖥 CLI

- Interactive CLI preview application
- Page navigation using keyboard input (`n`, `p`, `q`)
- Screen clearing and display refresh
- Compact page indicator integrated into prompt

### 🧱 Architecture

- Thin wrapper around external parser crate
- Clear separation of concerns:
    - parsing
    - formatting
    - display
    - navigation
- Core logic independent from I/O (embedded-ready)

### 🧪 Testing

- Unit tests for:
    - wrapping logic
    - labeled rendering
    - pagination
    - pager navigation