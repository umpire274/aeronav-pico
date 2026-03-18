# AeroNav Pico

![CI](https://github.com/umpire274/aeronav-pico/actions/workflows/rust.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Rust](https://img.shields.io/badge/Rust-2024-orange)
![Status](https://img.shields.io/badge/status-active-success)
![Version](https://img.shields.io/github/v/release/umpire274/aeronav-pico)
![Issues](https://img.shields.io/github/issues/umpire274/aeronav-pico)
![PRs](https://img.shields.io/github/issues-pr/umpire274/aeronav-pico)

AeroNav Pico is a lightweight Rust-based CLI designed to parse and display METAR and TAF weather reports in a readable,
paginated format.

It is built as a foundation for future embedded usage on devices like the Picocalc.

---

## ✨ Features

* Parse METAR and TAF reports using `metar-taf-parser`
* Natural language formatted output
* Interactive paginated viewer
* File input support (`--file`)
* Direct CLI input support
* Optional header (`--no-header`)
* Built-in help and version flags
* Layout-aware rendering via core engine

---

## 🚀 Usage

### Basic (demo)

```bash
aeronav-cli
```

### Direct input

```bash
aeronav-cli -- METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG
```

### From file

```bash
aeronav-cli --file report.txt
```

### Disable header

```bash
aeronav-cli --no-header
```

### Help

```bash
aeronav-cli --help
```

### Version

```bash
aeronav-cli --version
```

---

## 🧠 Architecture

The project is split into:

### `aeronav-core`

* Parsing integration
* Viewer logic
* Pagination engine
* Layout system (`ViewerLayout`)
* Command handling (`ViewerCommand`)

### `aeronav-cli`

* CLI interface
* Input parsing
* Terminal rendering

---

## 🧩 Design Goals

* Minimal dependencies
* Embedded-friendly architecture
* Clear separation of concerns
* Extensible UI model

---

## 🔮 Future Plans

* Picocalc integration
* Hardware key mapping
* Frame-based UI renderer
* Multi-language support
* Network data fetching (METAR/TAF online)

---

## 📄 License

Licensed under either of:

* MIT License
* Apache License 2.0

at your option.

---

## 👤 Author

Developed by Alessandro (umpire274)
