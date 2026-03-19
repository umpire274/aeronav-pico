//! Core library for AeroNav Pico.
//!
//! This crate provides a thin application layer around the
//! metar-taf-parser crate so that desktop and Picocalc frontends
//! can share the same logic.

pub mod app;
pub mod display_target;
pub mod error;
pub mod input;
pub mod picocalc_display_target;
pub mod runtime;
pub mod ui;
pub mod weather;
