///
/// Coordinates of the point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot<T> {
    pub x: T,
    pub y: T,
}
impl From<&[isize]> for Dot<isize> {
    fn from(dot: &[isize]) -> Self {
        Dot { x: dot[0], y: dot[1] }
    }
}