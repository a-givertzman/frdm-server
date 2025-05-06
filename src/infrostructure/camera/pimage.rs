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
//
// TODO: Better way to compare matrixes:
// bool eq = std::equal(a.begin<uchar>(), a.end<uchar>(), b.begin<uchar>());
impl PartialEq for PImage {
    fn eq(&self, other: &Self) -> bool {
        let mut dst = self.frame.clone();
        opencv::core::compare(&self.frame, &other.frame, &mut dst, opencv::core::CmpTypes::CMP_EQ as i32).is_ok()
    }
}
