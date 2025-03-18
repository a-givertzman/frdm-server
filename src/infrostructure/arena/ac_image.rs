///
/// Contains a image with metadata
pub struct AcImage {
    pub width: usize,
    pub height: usize,
    pub timestamp: usize,
    pub mat: opencv::core::Mat,
}
