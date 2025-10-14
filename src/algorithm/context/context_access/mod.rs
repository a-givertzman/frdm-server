mod common_access;
mod fast_scan_access;
mod fine_scan_access;
mod normalized_access;

pub use common_access::*;
pub use fast_scan_access::*;
pub use fine_scan_access::*;
pub use normalized_access::*;

use sal_core::error::Error;
use crate::Context;

///
/// Provides restricted write access to the [Context] members
pub trait ContextWrite<Branch, T> {
    fn write(self, value: T) -> Result<Context, Error>;
}
///
/// Provides simple read access to the [Context] members
pub trait ContextRead<Branch, T> {
    fn read(&self) -> &T;
}
