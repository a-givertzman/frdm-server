///
/// Contains a image with metadata
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub timestamp: usize,
    pub mat: opencv::core::Mat,
    pub bytes: usize,
}
