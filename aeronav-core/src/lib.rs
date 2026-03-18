//! Core library for AeroNav Pico.
//!
//! This crate provides a thin application layer around the
//! metar-taf-parser crate so that desktop and Picocalc frontends
//! can share the same logic.

pub mod display;
pub mod error;
pub mod pager;
pub mod viewer;
pub mod weather;
