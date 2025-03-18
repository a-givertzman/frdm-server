///
/// Pixel format
/// - RGB8/10/12/16
/// - BRG8/10/12/16
/// - Mono8/10/12/16
#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    RGB8,
    BRG8,
    BGR8,
    Mono8,
}
impl PixelFormat {
    ///
    /// Returns the string name of the format``
    pub fn format(&self) -> String {
        match self {
            Self::Mono8 => String::from("Mono8"),
            Self::RGB8 => String::from("RGB8"),
            Self::BRG8 => String::from("BRG8"),
            Self::BGR8 => String::from("BGR8"),
        }
    }
    ///
    /// Returns the OpenCV color format
    pub fn cv_format(&self) -> i32 {
        match self {
            Self::Mono8 => opencv::core::CV_8UC1,
            Self::RGB8 => opencv::core::CV_8UC3,
            Self::BRG8 => opencv::core::CV_8UC3,
            Self::BGR8 => opencv::core::CV_8UC3,
        }
    }
}
