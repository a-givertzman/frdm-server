use opencv::{
    core::{BorderTypes, Mat, Point2i, Scalar}, imgproc::{self, MorphTypes},
};
use sal_core::error::Error;
use crate::{algorithm::cv::StructuringElement, Eval};
///
/// Apply `OpenCv` Threshold operator to passed image
pub struct Threshold<'a> {
    operation: MorphTypes,
    kernel: Vec<i32>,
    iterations: i32,
    border: BorderTypes,
    border_val: Option<Scalar>,
    structuring_element: Option<Box<dyn Eval<(), Result<Mat, Error>>>>,
    ctx: Box<dyn Eval<&'a Mat, Result<Mat, Error>>>,
}
//
//
impl<'a> Threshold<'a> {
    ///
    /// Returns [Threshold] new instance
    /// - `operation` - Type of a morphological operation, see morph_types.
    /// - `kernel` - Size of the Structuring element.
    #[allow(unused)]
    pub fn new(
        operation: MorphTypes,
        kernel: &[i32; 2],
        ctx: impl Eval<&'a Mat, Result<Mat, Error>> + 'static,
    ) -> Self {
        Self {
            operation,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Returns [Threshold] `ERODE` operation new instance
    /// - `kernel` - Size of the Structuring element.
    #[allow(unused)]
    pub fn erode(
        kernel: &[i32; 2],
        ctx: impl Eval<&'a Mat, Result<Mat, Error>> + 'static,
    ) -> Self {
        Self {
            operation: MorphTypes::MORPH_ERODE,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Returns [Threshold] `OPEN` operation new instance
    /// - `kernel` - Size of the Structuring element.
    #[allow(unused)]
    pub fn open(
        kernel: &[i32; 2],
        ctx: impl Eval<&'a Mat, Result<Mat, Error>> + 'static,
    ) -> Self {
        Self {
            operation: MorphTypes::MORPH_OPEN,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Returns [Threshold] `DILATE` operation new instance
    /// - `kernel` - Size of the Structuring element.
    #[allow(unused)]
    pub fn dilate(
        kernel: &[i32; 2],
        ctx: impl Eval<&'a Mat, Result<Mat, Error>> + 'static,
    ) -> Self {
        Self {
            operation: MorphTypes::MORPH_DILATE,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `n` - Number of times erosion and dilation are applied.
    #[allow(unused)]
    pub fn with_iterations(mut self, n: usize) -> Self {
        self.iterations = n as i32;
        self
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `border` - Pixel extrapolation method, see #BorderTypes. BORDER_WRAP is not supported. Default `BORDER_CONSTANT`
    #[allow(unused)]
    pub fn with_border(mut self, border: BorderTypes) -> Self {
        self.border = border;
        self
    }
    ///
    /// Returns Structuring element `Mat` new instance
    /// - `ctx` - Custom [StructuringElement] can be specified for kernel calculation
    #[allow(unused)]
    pub fn with_kernel(mut self, ctx: impl Eval<(), Result<Mat, Error>> + 'static) -> Self {
        self.structuring_element = Some(Box::new(ctx));
        self
    }
}
//
//
impl<'a> Eval<&'a Mat, Result<Mat, Error>> for Threshold<'a> {
    fn eval(&self, mat: &'a Mat) -> Result<Mat, Error> {
        let error = Error::new("Threshold", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                opencv::imgproc::threshold(&blur, &mut contour, threshold * 0.4, 255.0, opencv::imgproc::ThresholdTypes::THRESH_BINARY as i32)
                    .map_err(|err| error.pass_with("Can't do Threshold", err.to_string()))?;

                let kernel = match &self.structuring_element {
                    Some(ctx) => ctx.eval(()).map_err(|err| error.pass(err))?,
                    None => StructuringElement::new(&[self.kernel[0], self.kernel[1]]).eval(()).map_err(|err| error.pass(err))?,
                };
                imgproc::morphology_ex(
                    &mat,
                    &mut dst,
                    self.operation as i32,
                    &kernel,
                    Point2i::new(-1, -1),
                    self.iterations,
                    self.border as i32,
                    self.border_val.unwrap_or(
                        opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass(err.to_string()))?,
                    ),
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't apply Threshold operation {:?}, kernel {:?}, border {:?}, border value {:?}", self.operation, self.kernel, self.border, self.border_val),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}