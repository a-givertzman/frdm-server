//! # Module of struct that store initial data
//!
//! This module constist structure of initial data
//! [documentation to initial data](design\docs\algorithm\part01\initial_data.md)
//!
//! ## Example of using
//! ```rust
//! use crate::kernel::initial_data::initial_data::InitialData;
//! let path = "....";
//! let initial_ctx = InitialCtx::new(&mut Storage::new(path);
//! println!("Steady state lifiting speed: {}", initial_ctx);
//! ```
mod initial_ctx;

pub(crate) use initial_ctx::*;
