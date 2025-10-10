use std::time::Instant;
use opencv::{core::Mat, imgproc::ThresholdTypes};
use sal_core::error::Error;
use crate::algorithm::{
    cv, ContextWrite, ContextRead,
    FastContoursCtx,
    EvalResult, ResultCtx,
};
use crate::conf::CvContoursConf;
use crate::{Eval, domain::Image};
///
/// Return filtered and binarised [Image] with contours detected
/// 
/// Binarization is based on the sharpness of the target segment
pub struct FastContours {
    conf: CvContoursConf,
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
    /// - `conf` - Configuration for `Contour dectection` algorithm:
    ///     - gausian:
    ///         - `kernel` - Gausian blur kernel size
    ///         - `sigma_x` - Standard deviation in X direction
    ///         - `sigma_y` - Standard deviation in Y direction
    ///     - sobel:
    ///         - `kernel_size` - Sobel kernel size
    ///         - `scale` - Scale factor for computed derivative values
    ///         - `delta` - Delta values added to results
    ///     - overlay:
    ///         - `src1-weight` - Weight for X gradient
    ///         - `src1-weight` - Weight for Y gradient
    ///         - `gamma` - Scalar added to weighted sum
    pub fn new(conf: CvContoursConf, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        let kernel = 13;
        Self {
            conf,
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
                                    0.4,
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
                let result: &ResultCtx = ctx.read();
                let frame = &result.frame;
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
                let result = ResultCtx { frame };
                log::debug!("FastContours.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
///
/// Closes calculation sequence, passing input `Mat`
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
