use crate::domain::Dot;

///
/// Coordinates of the point that deviate from others   
#[derive(Debug, Clone, PartialEq)]
pub struct Bend<T> {
    pub upper: Vec<Dot<T>>,
    pub lower: Vec<Dot<T>>,
}
impl<T> Bend<T>  {
    ///
    /// Returns [Bend] new instance
    pub fn new() -> Self {
        Self { upper: vec![], lower: vec![] }
    }
    ///
    /// Adds points to the `upper` and `lower` sequences
    pub fn push(&mut self, upper: Dot<T>, lower: Dot<T>) {
        self.upper.push(upper);
        self.lower.push(lower);
    }
}
//
//
impl From<&[usize; 4]> for Bend<usize> {
    fn from(bend: &[usize; 4]) -> Self {
        Bend { upper: vec![Dot::from([bend[0], bend[1]])], lower: vec![Dot::from([bend[2], bend[3]])] }
    }
}
//
//
impl From<[usize; 4]> for Bend<usize> {
    fn from(bend: [usize; 4]) -> Self {
        Bend { upper: vec![Dot::from([bend[0], bend[1]])], lower: vec![Dot::from([bend[2], bend[3]])] }
    }
}
