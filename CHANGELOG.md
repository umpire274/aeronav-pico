# Changelog

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