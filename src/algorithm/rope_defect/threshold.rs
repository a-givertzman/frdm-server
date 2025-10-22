///
/// The threshodl value used in the `WidthEmissions` and `RopeDefect` algorithm's
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Threshold(pub f64);
//
//
#[allow(unused)]
impl Threshold {
    /// Geometry threshold minimum value 1.1
    const MIN: Self = Self(1.1);
    /// Geometry threshold average value 1.2
    const AVG: Self = Self(1.2);
    /// Geometry threshold maximum value 1.3
    const MAX: Self = Self(1.3);
}
//
//
impl Default for Threshold {
    fn default() -> Self {
        Self::AVG
    }
}
