//!
//! Contour detection algorithms optimized for speed, tradeoff in result quality
//! 
//! - Convert into gray scale
//! - Apply autogamma
//! - First way
//!    - Find contours based on the sharpness (sopel gradient or laplacian)
//! - Second way
//!    - Find contours based on the moving objhect (diff of same pixel betwee current and previouse frame)
//! - Union contours of two ways using bitwise operation
//! 
mod fast_union;
mod fast_scan;

pub use fast_union::*;
pub use fast_scan::*;
