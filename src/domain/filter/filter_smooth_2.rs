use super::filter::Filter;
///
/// 
#[derive(Debug, Clone)]
pub struct FilterSmooth2<T> {
    prev: Option<T>,
    factor: f64,
}
//
// 
impl<T: Copy> FilterSmooth2<T> {
    ///
    /// Creates new FilterSmooth2<const N: usize, T>
    /// - `T` - Type of the Filter Item
    /// - `factor` - Smoothing of edge line factor. The higher the factor the smoother the line, can't be 0
    #[allow(unused)]
    pub fn new(initial: Option<T>, factor: f64) -> Self {
        Self {
            prev: initial,
            factor,
        }
    }
}
//
//
impl Filter for FilterSmooth2<u8> {
    type Item = u8;
    //
    //
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                let delta = value as f64 - prev as f64;
                let factor = (1.0 + delta.abs() / value as f64) * self.factor;
                let value = (prev as f64 + delta / factor).round() as u8;
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
//
//
impl Filter for FilterSmooth2<i32> {
    type Item = i32;
    //
    //
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                let delta = value as f64 - prev as f64;
                let factor = (1.0 + delta.abs() / value as f64) * self.factor;
                let value = (prev as f64 + delta / factor).round() as i32;
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
