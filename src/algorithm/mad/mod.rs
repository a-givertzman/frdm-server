//!
//! In this module realises algorithm's that search outliners in the sample
//! Realized algorithm's:
//! - `MAD` (Mediana Absolute Deviation)
mod bend;
mod mad_ctx;
mod mad;

pub use bend::*;
pub use mad_ctx::*;
pub use mad::*;
