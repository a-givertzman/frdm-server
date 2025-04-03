///
/// Contains a raw image buffer with metadata
pub struct AcImage {
    pub bytes: usize,
    pub width: usize,
    pub height: usize,
    pub timestamp: usize,
    pub data: *const u8,
}
