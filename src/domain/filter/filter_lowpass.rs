use circular_buffer::CircularBuffer;
use super::filter::Filter;

///
/// 
#[derive(Debug, Clone)]
pub struct FilterLowPass<const N: usize, T> {
    buffer: CircularBuffer<N, T>,
    factor: f64,
}
//
// 
impl<T: Copy, const N: usize> FilterLowPass<N, T> {
    ///
    /// Creates new FilterLowPass<const N: usize, T>
    /// - `T` - Type of the Filter Item
    pub fn new(initial: Option<T>, factor: f64) -> Self {
        let mut buffer = CircularBuffer::<N, T>::new();
        initial.map(|initial| {
            buffer.push_back(initial);
            initial
        });
        Self {
            buffer,
            factor,
        }
    }
}
//
//
impl<const N: usize> Filter for FilterLowPass<N, i32> {
    type Item = i32;
    //
    //
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        let sum = self.buffer.iter().sum::<i32>() + value;
        // let average = ((sum as f64) / ((self.buffer.len() + 1) as f64)).round() as i32;
        let average = sum / ((self.buffer.len() as i32) + 1);
        self.buffer.push_back(average);
        match self.buffer.front() {
            Some(v) => Some(*v),
            None => None,
        }
    }
}
