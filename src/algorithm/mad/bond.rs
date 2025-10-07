///
/// Coordinates of the point that deviate from others   
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bond<T> {
    pub x: T,
    pub y: T,
}
//
//
impl From<&[usize]> for Bond<usize> {
    fn from(bond: &[usize]) -> Self {
        Bond { x: bond[0], y: bond[1] }
    }
}