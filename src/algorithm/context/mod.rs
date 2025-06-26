//! # Module where to store initial data and result's of all algorithm's
//!
//! This module constist of feild, that give all info about programm
//! and their processing in the system.
//!
//! ## Example of using
//! ```rust
//! use crate::{algorithm::context::{context::Context, ctx_result::Result}
//! use crate::kernel::initial_ctx::initial_ctx::InitialCtx
//! let path = #"....";
//! let context = Contex::new(InitialCtx::new(Storage::new(path))).eval();
//! println!("Initial data: {}", context.initial);
//! ```
mod context_access;
mod context;
mod initial;
///
/// TODO: To be moved to the better place
mod testing_ctx;

pub(crate) use context_access::*;
pub(crate) use context::*;
pub(crate) use initial::*;
pub(crate) use testing_ctx::*;

use crate::domain::Error;

pub type EvalResult = Result<Context, Error>;