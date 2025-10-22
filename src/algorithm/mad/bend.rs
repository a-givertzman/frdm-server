use crate::domain::Dot;

///
/// Coordinates of the point that deviate from others   
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bend<T> {
    pub upper: Dot<T>,
    pub lower: Dot<T>,
}
//
//
impl From<&[usize; 4]> for Bend<usize> {
    fn from(bend: &[usize; 4]) -> Self {
        Bend { upper: Dot::from([bend[0], bend[1]]), lower: Dot::from([bend[2], bend[3]]) }
    }
}
//
//
impl From<[usize; 4]> for Bend<usize> {
    fn from(bend: [usize; 4]) -> Self {
        Bend { upper: Dot::from([bend[0], bend[1]]), lower: Dot::from([bend[2], bend[3]]) }
    }
}
