///
/// Coordinates of the point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot<T> {
    pub x: T,
    pub y: T,
}
impl From<&[usize; 2]> for Dot<usize> {
    fn from(dot: &[usize; 2]) -> Self {
        Dot { x: dot[0], y: dot[1] }
    }
}
impl From<[usize; 2]> for Dot<usize> {
    fn from(dot: [usize; 2]) -> Self {
        Dot { x: dot[0], y: dot[1] }
    }
}
