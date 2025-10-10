use opencv::{
    core::Mat, imgproc::{AdaptiveThresholdTypes, ThresholdTypes},
};
use sal_core::error::Error;
use crate::Eval;
///
/// Apply `OpenCv` AdaptiveThreshold operator to passed image
pub struct AdaptiveThreshold<In> {
    maxval: f64,
    method: AdaptiveThresholdTypes,
    typ: ThresholdTypes,
    block_size: i32,
    decrement: f64,
    ctx: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
}
//
//
impl<In> AdaptiveThreshold<In> {
    ///
    /// Returns [AdaptiveThreshold] new instance
    /// - `maxval` - Non-zero value assigned to the pixels for which the condition is satisfied.
    /// - `method` - Adaptive thresholding algorithm to use, see [AdaptiveThresholdTypes].
    ///    The `BORDER_REPLICATE` | `BORDER_ISOLATED` is used to process boundaries.
    /// - `typ` - Thresholding type that must be either THRESH_BINARY or #THRESH_BINARY_INV, see [ThresholdTypes].
    /// - `decrement` - Constant subtracted from the mean or weighted mean (see the details below).
    ///    Normally, it is positive but may be zero or negative as well.
    #[allow(unused)]
    pub fn new(
        maxval: f64,
        method: AdaptiveThresholdTypes,
        typ: ThresholdTypes,
        block_size: i32,
        decrement: f64,
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            maxval,
            method,
            typ,
            block_size,
            decrement,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<In> Eval<In, Result<Mat, Error>> for AdaptiveThreshold<In> {
    fn eval(&self, mat: In) -> Result<Mat, Error> {
        let error = Error::new("AdaptiveThreshold", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                opencv::imgproc::adaptive_threshold(
                    &mat,
                    &mut dst,
                    self.maxval,
                    self.method as i32,
                    self.typ as i32,
                    self.block_size,
                    self.decrement,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!(
                            "Can't apply Adaptive Threshold operation with maxval {:?}, method {:?}, type {:?}, block size {:?}, decrement {:?}",
                            self.maxval, self.method, self.typ, self.block_size, self.decrement,
                        ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
