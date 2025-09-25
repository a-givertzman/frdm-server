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
    /// 0.0...1.0
    rate: i8,
    amplify_factor: f32,
    reduce_factor: f32,
    threshold: f32,
}
//
// 
impl<T: Copy> FilterHighPass<T> {
    ///
    /// Creates new FilterHighPass<const N: usize, T>
    /// - `T` - Type of the Filter Item
    pub fn new(initial: Option<T>, rate: Option<i8>, amplify_factor: f64, reduce_factor: f64, threshold: f64) -> Self {
        Self {
            prev: initial,
            rate: rate.unwrap_or(0),
            amplify_factor: amplify_factor as f32,
            reduce_factor: reduce_factor as f32,
            threshold: threshold as f32,
        }
    }
    ///
    /// Returns current rate
    pub fn rate(&self) -> i8 {
        self.rate
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
                let delta = (value - prev) as f32;
                let delta_rel = 100.0 * delta / 255.0;
                self.prev.replace(value);
                let rate_pice = ((self.rate.abs() as f32) + 10.0 + 50.0 * delta_rel).round() as i8;
                if delta >= self.threshold {
                    let rate = self.rate + rate_pice;
                    self.rate = match rate > 100 {
                        true => 100,
                        false => rate,
                    };
                } else {
                    let rate = self.rate - rate_pice;
                    self.rate = match rate < -100 {
                        true => -100,
                        false => rate,
                    };
                };
                // log::debug!("FilterHighPass<u8>.add | rate: {:?}", self.rate);
                let value_ = match self.rate > 0 {
                    true => (value as f32) - 0.01 * (self.rate as f32) * self.amplify_factor,
                    false => (value as f32) - 0.01 * (self.rate as f32) * self.reduce_factor,
                };
                // if (value as f64) != value_ {
                //     log::debug!("FilterHighPass<u8>.add | rate: {:.3}  |  value: {} => {:.3}", self.rate, value, value_);
                // }
                Some(match value_ > 255.0 {
                    true => 255,
                    false => match value_ < 0.0 {
                        true => 0,
                        false => value_.round() as u8,
                    },
                })
            }
            None => {
                self.prev.replace(value);
                Some(value)
            }
        }
    }
}
