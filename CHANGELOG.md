# Changelog

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