use opencv::imgproc;
use opencv::core;
use sal_core::error::Error;
use crate::algorithm::{
    Context, ContextWrite,
    DetectingContoursCvCtx,
    EvalResult,
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
impl Eval<Image, Result<Context, Error>> for DetectingContoursCv {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("DetectingContoursCv", "eval");
        match self.ctx.eval(()) {
            Ok(ctx) => {
                // let initial_ctx = ContextRead::<InitialCtx>::read(&ctx);
                let mut gray = core::Mat::default();
                match imgproc::cvt_color(&frame.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0) {
                    Ok(_) => {
                        let mut blurred = core::Mat::default();
                        //
                        let kernel = core::Size::new(3, 3);
                        //
                        let sigma_x = 0.0;
                        let sigma_y = 0.0;
                        //
                        match imgproc::gaussian_blur(&gray, &mut blurred, kernel, sigma_x, sigma_y, core::BORDER_DEFAULT) {
                            Ok(_) => {
                                let mut sobelx = core::Mat::default();
                                let mut sobely = core::Mat::default();
                                //
                                let x_order = 1;
                                let y_order = 0;
                                //
                                let kernel_size = 3;
                                //
                                let scale = 1.0;
                                //
                                let delta = 0.0;
                                match imgproc::sobel(&blurred, &mut sobelx, core::CV_8U, x_order, y_order, kernel_size, scale, delta, core::BORDER_DEFAULT) {
                                    Ok(_) => {
                                            let x_order = 0;
                                            let y_order = 1;
                                        match imgproc::sobel(&blurred, &mut sobely, core::CV_8U, x_order, y_order, kernel_size, scale, delta, core::BORDER_DEFAULT) {
                                            Ok(_) => {
                                                let mut absx = core::Mat::default();
                                                let mut absy = core::Mat::default();
                                                //
                                                match core::convert_scale_abs_def(&sobelx, &mut absx) {
                                                    Ok(_) => {
                                                        match core::convert_scale_abs_def(&sobely, &mut absy) {
                                                            Ok(_) => {
                                                                let mut grad = core::Mat::default();
                                                                //
                                                                let alpha = 0.5;
                                                                //
                                                                let beta = 0.5;
                                                                //
                                                                let gamma = 0.0;
                                                                match core::add_weighted_def(&absx, alpha, &absy, beta, gamma, &mut grad) {
                                                                    Ok(_) => {
                                                                        let result = DetectingContoursCvCtx {
                                                                            result: Image {
                                                                                width: frame.width,
                                                                                height: frame.height,
                                                                                timestamp: frame.timestamp,
                                                                                mat: grad,
                                                                                bytes: frame.bytes,
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