//! Core library for AeroNav Pico.
//!
//! This crate provides a thin application layer around the
//! metar-taf-parser crate so that desktop and Picocalc frontends
//! can share the same logic.

pub mod app;
pub mod display;
pub mod error;
pub mod frame;
pub mod input;
pub mod key_source;
pub mod keys;
pub mod layout;
pub mod pager;
pub mod picocalc_key_source;
pub mod renderer;
pub mod viewer;
pub mod weather;
