# AeroNav Pico

![CI](https://github.com/umpire274/aeronav-pico/actions/workflows/rust.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Rust](https://img.shields.io/badge/Rust-2024-orange)
![Status](https://img.shields.io/badge/status-active-success)
![Version](https://img.shields.io/github/v/release/umpire274/aeronav-pico)
![Issues](https://img.shields.io/github/issues/umpire274/aeronav-pico)
![PRs](https://img.shields.io/github/issues-pr/umpire274/aeronav-pico)

---

## ✨ Overview

**AeroNav Pico** is a lightweight, modular Rust application designed to parse and display METAR and TAF weather reports
in a structured and readable format.

Built with portability in mind, it targets both:

* 🖥️ Desktop CLI environments
* 📟 Embedded devices (e.g. Picocalc)

---

## 🧩 Core Architecture

The project is built around a clean separation of concerns:

* **Parsing** → delegated to `metar-taf-parser`
* **Formatting** → natural language output
* **Layout** → wrapping and alignment
* **Paging** → multi-page navigation
* **Viewer** → state + navigation
* **Frame System** → UI abstraction layer

---

## 🆕 v0.4.0 Highlights

### 🧱 UI Frame System

Introduced a new rendering abstraction:

* `UiFrame` → full screen (header, content, footer)
* `FrameOptions` → configurable rendering
* `render_frame()` → produces ready-to-render output

---

### 🖥️ CLI Refactor

* Rendering logic moved to `aeronav-core`
* CLI now only handles input and display loop
* Cleaner and more maintainable structure

---

### ⚙️ Flexible Rendering

Configurable options:

* Header on/off
* Custom title
* Page indicator
* Footer prompt

Profiles:

* `FrameOptions::cli_default()`
* `FrameOptions::picocalc_default()`

---

### 📦 Automated Releases

GitHub Actions now:

* Builds for Linux, Windows, macOS (Intel + ARM)
* Packages binaries with docs and licenses
* Generates SHA256 checksums
* Publishes assets automatically

---

## 🚀 Usage

```bash
aeronav-cli "METAR LIRF 121250Z 18010KT 9999 FEW030 -RA 18/12 Q1015 NOSIG"
```

---

## 🎮 Controls

* `n` → next page
* `p` → previous page
* `q` → quit

---

## 🛠️ Build

```bash
cargo build --release
```

---

## 📦 Packaging Status

The project is structured as a Rust workspace with:

- `aeronav-core` for reusable application logic
- `aeronav-cli` for the command-line frontend

Package metadata has been prepared for future crates.io publication.

---

## 📜 License

Dual licensed under:

* MIT
* Apache-2.0

---

## 🚧 Roadmap

* Input from file / stdin
* Embedded display integration (Picocalc)
* Advanced navigation
* Multi-language support expansion
