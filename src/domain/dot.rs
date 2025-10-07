///
/// Coordinates of the point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot<T> {
    pub x: T,
    pub y: T,
}
impl From<&[usize]> for Dot<usize> {
    fn from(dot: &[usize]) -> Self {
        Dot { x: dot[0], y: dot[1] }
    }
}
