///
/// Store threshols values for algorithm's
#[derive(Debug, Clone, Copy)]
pub struct Threshold(pub f64);
//
//
impl Threshold {
    const VALUES: &[f64] = &[
        1.1,
        1.2,
        1.3,
    ];
    ///
    /// Returns geometry threshold minimum value 1.1
    pub fn min() -> Self {
        Self(Self::VALUES[0])
    }
    ///
    /// Returns geometry threshold minimum value 1.2
    pub fn avg() -> Self {
        Self(Self::VALUES[1])
    }
    ///
    /// Returns geometry threshold minimum value 1.3
    pub fn max() -> Self {
        Self(Self::VALUES[2])
    }
}