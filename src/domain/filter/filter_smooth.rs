use super::filter::Filter;
///
/// 
#[derive(Debug, Clone)]
pub struct FilterSmooth<T> {
    prev: Option<T>,
    factor: f64,
    factor_inv: f64,
}
//
// 
impl<T: Copy> FilterSmooth<T> {
    ///
    /// Creates new FilterSmooth<const N: usize, T>
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
impl Filter for FilterSmooth<i32> {
    type Item = i32;
    //
    //
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                let value = (0.5 * (self.factor * (value as f64) + self.factor_inv * (prev as f64))).round() as i32;
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
