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
pub struct FilterIsChanged<T> {
    prev: Option<T>,
    speed: f32,
    threshold: f32,
}
//
// 
impl<T: Copy> FilterIsChanged<T> {
    ///
    /// Creates new FilterIsChanged<T>
    /// - `T` - Type of the Filter Item
    pub fn new(initial: Option<T>, threshold: f64) -> Self {
        Self {
            prev: initial,
            speed: 0.0,
            threshold: threshold as f32,
        }
    }
    ///
    /// Returns current speed of changes
    #[allow(unused)]
    pub fn speed(&self) -> f32 {
        self.speed
    }
}
//
//
impl Filter for FilterIsChanged<f32> {
    type Item = f32;
    //
    /// Returns `Some(speed)` if changing `speed >= threshold`
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                self.speed = (value - prev).abs();
                self.prev = Some(value);
                if self.speed >= self.threshold {
                    Some(self.speed)
                } else {
                    None
                }
            }
            None => {
                self.prev = Some(value);
                None
            }
        }
    }
}
