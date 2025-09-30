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
    rate: f32,
    amplify_factor: f32,
    grow_speed: f32,
    reduce_factor: f32,
    down_speed: f32,
    threshold: f32,
    // percent_factor: f32,
}
//
// 
impl<T: Copy> FilterHighPass<T> {
    ///
    /// Creates new FilterHighPass<const N: usize, T>
    /// - `T` - Type of the Filter Item
    pub fn new(initial: Option<T>, amplify_factor: f64, grow_speed: f64, reduce_factor: f64, down_speed: f64, threshold: f64) -> Self {
        Self {
            prev: initial,
            rate: 0.0,
            amplify_factor: amplify_factor as f32,
            grow_speed: grow_speed as f32,
            reduce_factor: reduce_factor as f32,
            down_speed: down_speed as f32,
            threshold: threshold as f32,
            // percent_factor: 1.0 / 255.0,
        }
    }
    ///
    /// Returns current rate
    pub fn rate(&self) -> f32 {
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
                let delta = (value as f32 - prev as f32).abs();
                // let delta_rel = delta * self.percent_factor;
                self.prev = Some(value);
                if delta >= self.threshold {
                    // log::debug!("FilterHighPass<u8>.add | delta: {delta},  delta_rel {delta_rel}  =>  CHANGED");
                    // let rate = self.rate + 0.05 + 0.1 * delta_rel;
                    // let rate = self.rate + (1.0 - self.rate) * delta_rel * self.grow_speed;
                    let rate = self.rate + (1.0 - self.rate) * self.grow_speed;
                    self.rate = match rate > 1.0 {
                        true => 1.0,
                        false => rate,
                    };
                } else {
                    // log::debug!("FilterHighPass<u8>.add | delta: {delta},  delta_rel {delta_rel}  =>  KEEPED");
                    // let rate = self.rate - (0.05 + 0.1 * (1.0 - delta_rel));
                    let rate = self.rate + (-1.0 - self.rate) * self.down_speed;
                    self.rate = match rate < -1.0 {
                        true => -1.0,
                        false => rate,
                    };
                };
                let value_ = (value as f32) + self.rate * match self.rate > 0.0 {
                    true => self.amplify_factor,
                    false => self.reduce_factor,
                };
                Some(match value_ > 255.0 {
                    true => 255,
                    false => match value_ < 0.0 {
                        true => 0,
                        false => value_.round() as u8,
                    },
                })
            }
            None => {
                self.prev = Some(value);
                Some(value)
            }
        }
    }
}
