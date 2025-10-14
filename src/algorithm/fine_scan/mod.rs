//!
//! Contour detection algorithms are optimized for best result, at the expence of speed
//! 
//! - Expect gray scale normalised image
//! - Find contours
//!    - First way
//!       - Find contours based on the sharpness (Sobel gradient or Laplacian)
//!       - Merge nerby contours by threshold of distance
//!    - Second way
//!       - Find contours based on the moving objhect (diff of same pixel betwee current and previouse frame)
//!       - Merge nerby contours by threshold
//! - Union contours of two ways using bitwise operation
//!
mod fine_contours;
mod fine_union;
mod fine_scan_conf;
mod fine_scan;

pub use fine_contours::*;
pub use fine_union::*;
pub use fine_scan_conf::*;
pub use fine_scan::*;