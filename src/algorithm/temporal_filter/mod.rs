//! # Temporal Filter
//! 
//! Highlighting / Hiding pixels depending on those changing speed
//! 
//! Useful for detecting moving objects
//! 
mod filter_high_pass;
mod temporal_filter_conf;
mod temporal_filter;

pub use filter_high_pass::*;
pub use temporal_filter_conf::*;
pub use temporal_filter::*;