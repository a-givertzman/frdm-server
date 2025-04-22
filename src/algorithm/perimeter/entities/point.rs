///
/// Represent point on image
#[derive(Debug, Clone)]
pub struct Point {
    pub rgba: Vec<u8>,
    pub x: u32,
    pub y: u32,
}
//
//
impl Point {
    ///
    /// New instance [Point]
    pub fn new() -> Self {
        Self {
            rgba: Vec::new(),
            x: 0,
            y: 0,
        }
    }
}