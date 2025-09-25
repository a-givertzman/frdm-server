use std::time::Instant;
use opencv::imgproc;
use opencv::core;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite, ContextRead,
    DetectingContoursCvCtx, AutoBrightnessAndContrastCtx,
    EvalResult, ResultCtx,
};
use crate::conf::DetectingContoursConf;
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return filtered and binarised [Image] with contours detected
pub struct DetectingContoursCv {
    conf: DetectingContoursConf,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl DetectingContoursCv{
    ///
    /// Returns [DetectingContoursCv] new instance
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
    pub fn new(conf: DetectingContoursConf, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self { 
            conf,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for DetectingContoursCv {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("DetectingContoursCv", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx = ctx.read();
                let frame = &result.frame;
                let mut gray = core::Mat::default();
                match imgproc::cvt_color(&frame.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0) {
                    Ok(_) => {
                        let mut blurred = core::Mat::default();
                        let kernel_size = core::Size::new(self.conf.gausian.blur_w, self.conf.gausian.blur_h);
                        match imgproc::gaussian_blur(&gray, &mut blurred, kernel_size, self.conf.gausian.sigma_x, self.conf.gausian.sigma_y, core::BORDER_DEFAULT) {
                            Ok(_) => {
                                let mut sobelx = core::Mat::default();
                                let mut sobely = core::Mat::default();
                                //
                                // Derivative order in X direction for X gradient
                                let x_order = 1;
                                //
                                // Derivative order in Y direction for X gradient
                                let y_order = 0;
                                match imgproc::sobel(&blurred, &mut sobelx, core::CV_8U, x_order, y_order, self.conf.sobel.kernel_size, self.conf.sobel.scale, self.conf.sobel.delta, core::BORDER_DEFAULT) {
                                    Ok(_) => {
                                            //
                                            // Derivative order in X direction for Y gradient
                                            let x_order = 0;
                                            //
                                            // Derivative order in Y direction for Y gradient
                                            let y_order = 1;
                                        match imgproc::sobel(&blurred, &mut sobely, core::CV_8U, x_order, y_order, self.conf.sobel.kernel_size, self.conf.sobel.scale, self.conf.sobel.delta, core::BORDER_DEFAULT) {
                                            Ok(_) => {
                                                let mut absx = core::Mat::default();
                                                let mut absy = core::Mat::default();
                                                //
                                                // Converts X gradient to 8-bit absolute value
                                                match core::convert_scale_abs_def(&sobelx, &mut absx) {
                                                    Ok(_) => {
                                                        //
                                                        // Converts Y gradient to 8-bit absolute value
                                                        match core::convert_scale_abs_def(&sobely, &mut absy) {
                                                            Ok(_) => {
                                                                let mut grad = core::Mat::default();
                                                                match core::add_weighted_def(&absx, self.conf.overlay.src1_weight, &absy, self.conf.overlay.src2_weight, self.conf.overlay.gamma, &mut grad) {
                                                                    Ok(_) => {
                                                                        let frame = Image {
                                                                            width: frame.width,
                                                                            height: frame.height,
                                                                            timestamp: frame.timestamp,
                                                                            mat: grad,
                                                                            bytes: frame.bytes,
                                                                        };
                                                                        let result = DetectingContoursCvCtx { result: frame.clone() };
                                                                        let ctx = ctx.write(result)?;
                                                                        let result = ResultCtx { frame };
                                                                        log::debug!("DetectingContoursCv.eval | Elapsed: {:?}", t.elapsed());
                                                                        ctx.write(result)
                                                                    }
                                                                    Err(err) => Err(error.pass(err.to_string())),
                                                                }
                                                            }
                                                            Err(err) => Err(error.pass(err.to_string())),
                                                        }
                                                    }
                                                    Err(err) => Err(error.pass(err.to_string())),
                                                }
                                            }
                                            Err(err) => Err(error.pass(err.to_string())),
                                        }
                                    }
                                    Err(err) => Err(error.pass(err.to_string())),
                                }
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}