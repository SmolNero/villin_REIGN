// src/lib.rs
// villin_REIGN - Military-grade IoT security
pub mod core;

pub mod security;

pub mod api;

pub mod monitoring;

pub use core::REIGN;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
