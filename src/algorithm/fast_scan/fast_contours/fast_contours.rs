use std::time::Instant;
use opencv::{core::Mat, imgproc::ThresholdTypes};
use sal_core::error::Error;
use crate::algorithm::{
    cv, ContextWrite, ContextRead,
    FastContoursCtx, FastContoursConf,
    EvalResult, ResultCtx,
};
use crate::{Eval, domain::Image};
///
/// Return filtered and binarised [Image] with contours detected
/// 
/// Binarization is based on the sharpness of the target segment
pub struct FastContours {
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    proc: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync + Send + Sync>,
    debug: bool,
}
//
//
impl FastContours {
    ///
    /// Returns [FastContours] new instance
    /// - `ctx` - Prevouse step returns [Image] in [Context]
    /// - `conf` - Configuration for `Fast Contour dectection` algorithm:
    ///     cropping:
    ///         x: 230              # New left edge
    ///         y: 300              # New top edge
    ///         width: 1410         # New image width
    ///         height: 1000        # New image height
    ///     gamma:
    ///         factor: 120.0       # Percent of influence of [AutoGamma] algorythm bigger the value more the effect of [AutoGamma] algorythm, %
    ///     otsu-tune: 0.40         # Auto threshold factor, 1 - no correction, 0..1 - more, 1.. - less sensitive
    pub fn new(conf: FastContoursConf, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        let kernel = 13;
        Self {
            ctx: Box::new(ctx),
            proc: Box::new(
                cv::Morphology::dilate(
                    &[7, 7],
                    cv::GaussianBlur::new(
                        &[15, 15],
                        cv::Morphology::open(
                            &[5, 5],
                            cv::GaussianBlur::new(
                                &[7, 7],
                                cv::AutoThreshold::new(
                                    conf.otsu_tune,
                                    255.0,
                                    ThresholdTypes::THRESH_BINARY,
                                    cv::GaussianBlur::new(
                                        &[kernel, kernel],
                                        cv::Laplacian::new(
                                            5,
                                            cv::GaussianBlur::new(
                                                &[kernel, kernel],
                                                PassCvMat::new(),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    )
                ),
            ),
            debug,
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FastContours {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FastContours", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mat = self.proc.eval(frame.mat.clone())
                    .map_err(|err| error.pass(err))?;
                let frame = Image {
                    width: frame.width,
                    height: frame.height,
                    timestamp: frame.timestamp,
                    mat: mat,
                    bytes: frame.bytes,
                };
                let ctx = if self.debug {
                    let result = FastContoursCtx { result: frame.clone() };
                    ctx.write(result)?
                } else {
                    ctx
                };
                let result = ResultCtx { val: frame };
                log::debug!("FastContours.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
///
/// Closes calculation sequence, passing input [Mat]
struct PassCvMat {}
impl PassCvMat {
    fn new() -> Self {
        Self {  }
    }
}
impl Eval<Mat, Result<Mat, Error>> for PassCvMat {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        Ok(mat)
    }
}
