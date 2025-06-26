use opencv::imgproc;
use opencv::core;
use sal_core::error::Error;
use crate::algorithm::{
    Context, ContextRead, ContextWrite,
    DetectingContoursCvCtx,
    EvalResult, InitialCtx,
};
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return filtered and binarised [Image] with contours detected
pub struct DetectingContoursCv {
    ctx: Box<dyn Eval<(), Result<Context, Error>>>,
}
//
//
impl DetectingContoursCv{
    ///
    /// Returns [DetectingContoursCv] new instance
    pub fn new(ctx: impl Eval<(), Result<Context, Error>> + 'static) -> Self {
        Self { 
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(), Result<Context, Error>> for DetectingContoursCv {
    fn eval(&self, _: ()) -> EvalResult {
        let error = Error::new("DetectingContoursCv", "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                let initial_ctx = ContextRead::<InitialCtx>::read(&ctx);
                let mut gray = core::Mat::default();
                match imgproc::cvt_color(&initial_ctx.src_frame.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0) {
                    Ok(_) => {
                        let mut blurred = core::Mat::default();
                        match imgproc::gaussian_blur(&gray, &mut blurred, core::Size::new(3, 3), 0.0, 0.0, core::BORDER_DEFAULT) {
                            Ok(_) => {
                                let mut sobelx = core::Mat::default();
                                let mut sobely = core::Mat::default();
                                match imgproc::sobel(&blurred, &mut sobelx, core::CV_8U, 1, 0, 3, 1.0, 0.0, core::BORDER_DEFAULT) {
                                    Ok(_) => {
                                        match imgproc::sobel(&blurred, &mut sobely, core::CV_8U, 0, 1, 3, 1.0, 0.0, core::BORDER_DEFAULT) {
                                            Ok(_) => {
                                                let mut absx = core::Mat::default();
                                                let mut absy = core::Mat::default();
                                                match core::convert_scale_abs(&sobelx, &mut absx, 1.0, 0.0) {
                                                    Ok(_) => {
                                                        match core::convert_scale_abs(&sobely, &mut absy, 1.0, 0.0) {
                                                            Ok(_) => {
                                                                let mut grad = core::Mat::default();
                                                                match core::add_weighted(&absx, 0.5, &absy, 0.5, 0.0, &mut grad, -1) {
                                                                    Ok(_) => {
                                                                        let result = DetectingContoursCvCtx {
                                                                            result: Image {
                                                                                width: initial_ctx.src_frame.width,
                                                                                height: initial_ctx.src_frame.height,
                                                                                timestamp: initial_ctx.src_frame.timestamp,
                                                                                mat: grad,
                                                                                bytes: initial_ctx.src_frame.bytes,
                                                                            }
                                                                        };
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