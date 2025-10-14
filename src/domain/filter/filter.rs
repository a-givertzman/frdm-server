use std::marker::PhantomData;

///
/// Passes through single value
/// - call add(value) to apply new value, it will be returned immediately
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
    item: PhantomData<T>
}
//
// 
impl<T> FilterEmpty<T> {
    pub fn new() -> Self {
        Self {
            item: PhantomData,
        }
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