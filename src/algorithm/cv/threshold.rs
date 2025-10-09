use opencv::{core::Mat, imgproc::ThresholdTypes};
use sal_core::error::Error;
use crate::Eval;
///
/// Apply `OpenCv` Threshold operator to passed image
pub struct Threshold {
    threshold: f64,
    maxval: f64,
    typ: ThresholdTypes,
    ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
}
//
//
impl Threshold {
    ///
    /// Returns [Threshold] new instance
    /// - `threshold` - Threshold value.
    /// - `maxval` - Maximum value to use with the `THRESH_BINARY` and `THRESH_BINARY_INV` thresholding types.
    /// - `typ` - Size of the Structuring element.
    #[allow(unused)]
    pub fn new(
        threshold: f64,
        maxval: f64,
        typ: ThresholdTypes,
        ctx: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            threshold,
            maxval,
            typ,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Mat, Result<Mat, Error>> for Threshold {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        let error = Error::new("Threshold", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                opencv::imgproc::threshold(
                    &mat,
                    &mut dst,
                    self.threshold,
                    self.maxval,
                    self.typ as i32,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't apply Threshold operation with threshold {:?}, maxval {:?}, typ {:?}", self.threshold, self.maxval, self.typ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
