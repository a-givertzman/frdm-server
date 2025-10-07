///
/// Holds single value
/// - call add(value) to apply new value
/// - pop current value by calling value()
/// - is_changed() - check if value was changed after las add()
pub trait Filter: std::fmt::Debug {
    type Item;
    ///
    /// - Updates state with value if value != inner
    fn add(&mut self, value: Self::Item) -> Option<Self::Item>;
}
///
/// Pass input value as is
#[derive(Debug, Clone)]
pub struct FilterEmpty<T> {
    last: Option<T>,
}
//
// 
impl<T: Copy> FilterEmpty<T> {
    pub fn new(initial: Option<T>) -> Self {
        Self { last: initial }
    }
}
//
// 
impl<T: Copy + std::fmt::Debug + std::cmp::PartialEq> Filter for FilterEmpty<T> {
    type Item = T;
    //
    //
    fn add(&mut self, value: Self::Item) -> Option<T> {
        Some(value)
    }
}