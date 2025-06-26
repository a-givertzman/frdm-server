//!
//! In this module realises algorithm's that search outliners in the sample
//! Realized algorithm's:
//! - `MAD` (Mediana Absolute Deviation)
mod bond;
mod mad_ctx;
mod mad;

pub(crate) use bond::*;
pub(crate) use mad_ctx::*;
pub(crate) use mad::*;
