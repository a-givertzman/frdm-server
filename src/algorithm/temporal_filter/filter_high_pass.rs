use crate::domain::Filter;

/// 
/// A high-pass filter is an filter passes signals with a 
/// frequency higher than a certain cutoff frequency and
/// attenuates signals with frequencies lower than the cutoff frequency.
/// 
/// In this case `fequency` is how often the same pixel of the image sequence being changed
/// 
/// Output value also depends of hte amount of the pixel value changed
#[derive(Debug, Clone)]
pub struct FilterHighPass<T> {
    prev: Option<T>,
    factor: f64,
    factor_inv: f64,
}
//
// 
impl<T: Copy> FilterHighPass<T> {
    ///
    /// Creates new FilterHighPass<const N: usize, T>
    /// - `T` - Type of the Filter Item
    pub fn new(initial: Option<T>, factor: f64) -> Self {
        Self {
            prev: initial,
            factor,
            factor_inv: 1.0 / factor,
        }
    }
}
//
//
impl Filter for FilterHighPass<u8> {
    type Item = u8;
    //
    //
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                let value = ((value as f64) + self.factor * ((prev as f64).powi(2) - (value as f64).powi(2))).powf(0.5).round() as u8;
                self.prev.replace(value);
                Some(value)
            }
            None => {
                self.prev.replace(value);
                Some(value)
            }
        }
    }
}
