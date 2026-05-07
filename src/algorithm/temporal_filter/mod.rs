//! # Temporal Filter
//! 
//! Highlighting / Hiding pixels depending on those changing speed
//! 
//! Useful for detecting moving objects
//! 
mod temporal_filter_conf;
mod temporal_filter_ctx;
mod temporal_filter;

pub use temporal_filter_conf::*;
pub use temporal_filter_ctx::*;
pub use temporal_filter::*;