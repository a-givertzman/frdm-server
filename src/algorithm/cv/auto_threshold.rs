use opencv::{core::Mat, imgproc::ThresholdTypes};
use sal_core::error::Error;
use crate::Eval;
///
/// Apply `OpenCv` Automatic Threshold operator to passed image
/// 
/// The `treshold` value calculated on input image using Otsu's algorithm
/// Then applied multoplied by `factor` to the input image
pub struct AutoThreshold {
    factor: f64,
    maxval: f64,
    typ: ThresholdTypes,
    ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
}
//
//
impl AutoThreshold {
    ///
    /// Returns [AutoThreshold] new instance
    /// - `factor` - Multiplier for the calculated threshold value.
    /// - `maxval` - Maximum value to use with the `THRESH_BINARY` and `THRESH_BINARY_INV` thresholding types.
    #[allow(unused)]
    pub fn new(
        factor: f64,
        maxval: f64,
        typ: ThresholdTypes,
        ctx: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            factor,
            maxval,
            typ,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Mat, Result<Mat, Error>> for AutoThreshold {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        let error = Error::new("AutoThreshold", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                // let threshold = opencv::imgproc::threshold(&blur, &mut contour, 0.0, 255.0, opencv::imgproc::ThresholdTypes::THRESH_OTSU as i32)
                //     .map_err(|err| error.pass_with("Can't do Threshold", err.to_string()))?;
                // opencv::imgproc::threshold(&blur, &mut contour, threshold * 0.4, 255.0, opencv::imgproc::ThresholdTypes::THRESH_BINARY as i32)
                //     .map_err(|err| error.pass_with("Can't do Threshold", err.to_string()))?;
                let threshold = opencv::imgproc::threshold(
                    &mat,
                    &mut dst,
                    0.0,
                    self.maxval,
                    ThresholdTypes::THRESH_OTSU as i32,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't calculate Threshold with maxval {:?}, typ 'THRESH_OTSU'", self.maxval),
                        err.to_string(),
                    )
                })?;
                opencv::imgproc::threshold(
                    &mat,
                    &mut dst,
                    threshold * self.factor,
                    self.maxval,
                    self.typ as i32,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't apply Threshold operation with threshold {:?}, maxval {:?}, typ {:?}", threshold * self.factor, self.maxval, self.typ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
