use opencv::core::Mat;

///
/// To be replaced with original PImage from OpenCV
#[derive(Debug)]
pub struct PImage {
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