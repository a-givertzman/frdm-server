use opencv::{
    core, core::Mat, imgproc::MorphShapes,
};
use sal_core::error::Error;
use crate::Eval;
///
/// Creates `OpenCv` Structuring element
pub struct StructuringElement {
    kernel: Vec<i32>,
    shape: MorphShapes,
}
//
//
impl StructuringElement {
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `kernel` - Structuring element kernel size, [w, h]
    #[allow(unused)]
    pub fn new(kernel: &[i32; 2]) -> Self {
        Self { 
            kernel: kernel.into(),
            shape: MorphShapes::MORPH_ELLIPSE,
        }
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `shape` - Element shape, one of [MorphShapes]
    #[allow(unused)]
    pub fn with_shape(mut self, shape: MorphShapes) -> Self {
        self.shape = shape;
        self
    }
}
//
//
impl Eval<(), Result<Mat, Error>> for StructuringElement {
    fn eval(&self, _: ()) -> Result<Mat, Error> {
        opencv::imgproc::get_structuring_element(
            self.shape as i32,
            core::Size2i::new(self.kernel[0], self.kernel[1]),
            core::Point2i::new(-1, -1),
        )
        .map_err(|err| {
            Error::new("StructuringElement", "eval")
                .pass_with(format!("Can't create Structuring Element, kernel {:?}", self.kernel), err.to_string())
        })
    }
}