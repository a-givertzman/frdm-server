use opencv::core::MatTraitConst;

///
/// Contains a image with metadata
#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub timestamp: usize,
    pub mat: opencv::core::Mat,
    pub bytes: usize,
}
//
//
impl Image {
    ///
    /// Returns [Image] new instance
    /// - `width` - Origin size of image
    /// - `height` - Origin size of image
    /// - `mat` - The matrix of image
    /// - `timestamp` - Timstemp of image
    /// - `bytes` - Length of image payload in bytes
    pub fn new(
        width: usize,
        height: usize,
        mat: opencv::core::Mat,
        timestamp: usize,
    ) -> Self {
        Self {
            width,
            height,
            timestamp,
            bytes: mat.elem_size1(),
            mat,
        }
    }
    ///
    /// Used for testing only !!!
    /// To simply create [Image] and compare it by matrix
    /// 
    /// Use `Image::new` instead
    pub fn with(mat: opencv::core::Mat) -> Self {
        Self {
            width: 0,
            height: 0,
            timestamp: 0,
            mat,
            bytes: 0,
        }
    }
}
//
//
impl Default for Image {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            timestamp: 0,
            mat: opencv::core::Mat::default(),
            bytes: 0,
        }
    }
}
//
// TODO: Better way to compare matrixes:
// bool eq = std::equal(a.begin<uchar>(), a.end<uchar>(), b.begin<uchar>());
impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        let mut dst = self.mat.clone();
        opencv::core::compare(&self.mat, &other.mat, &mut dst, opencv::core::CmpTypes::CMP_EQ as i32).is_ok()
    }
}
