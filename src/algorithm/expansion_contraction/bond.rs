///
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bond<T> {
    pub x: T,
    pub y: T,
}
//
//
impl From<&[isize]> for Bond<isize> {
    fn from(bond: &[isize]) -> Self {
        Bond { x: bond[0], y: bond[1] }
    }
}