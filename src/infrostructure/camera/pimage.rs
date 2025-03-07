use opencv::core::Mat;
///
/// # Description to the [PImage] class
/// - contains information about frames
#[derive(Debug)]
pub struct PImage {
    /// frame pixel matrix
    pub frame: Mat,
}
//
//
impl PImage{
    pub fn new(frame:Mat) -> Self {
        Self {
            frame,
        }
    }
}