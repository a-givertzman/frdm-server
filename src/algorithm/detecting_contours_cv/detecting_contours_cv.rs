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
    //
    // Gausian blur kernel size
    kernel: core::Size,
    //
    // Standard deviation in X direction
    sigma_x: f64,
    //
    // Standard deviation in Y direction
    sigma_y: f64,
    //
    // Sobel kernel size
    kernel_size: i32,
    //
    // Scale factor for computed derivative values
    scale: f64,
    //
    // Delta values added to results
    delta: f64,
    //
    // Weight for X gradient
    alpha: f64,
    //
    // Weight for Y gradient
    beta: f64,
    //
    // Scalar added to weighted sum
    gamma: f64,
}
//
//
impl DetectingContoursCv{
    ///
    /// Returns [DetectingContoursCv] new instance
    pub fn new(ctx: impl Eval<(), Result<Context, Error>> + 'static) -> Self {
        Self { 
            ctx: Box::new(ctx),
            kernel: core::Size::new(3,3),
            sigma_x: 0.0,
            sigma_y: 0.0,
            kernel_size: 3,
            scale: 1.0,
            delta: 0.0,
            alpha: 0.5,
            beta: 0.5,
            gamma: 0.0,
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
                        match imgproc::gaussian_blur(&gray, &mut blurred, self.kernel, self.sigma_x, self.sigma_y, core::BORDER_DEFAULT) {
                            Ok(_) => {
                                let mut sobelx = core::Mat::default();
                                let mut sobely = core::Mat::default();
                                //
                                // Derivative order in X direction for X gradient
                                let x_order = 1;
                                //
                                // Derivative order in Y direction for X gradient
                                let y_order = 0;
                                match imgproc::sobel(&blurred, &mut sobelx, core::CV_8U, x_order, y_order, self.kernel_size, self.scale, self.delta, core::BORDER_DEFAULT) {
                                    Ok(_) => {
                                            //
                                            // Derivative order in X direction for Y gradient
                                            let x_order = 0;
                                            //
                                            // Derivative order in Y direction for Y gradient
                                            let y_order = 1;
                                        match imgproc::sobel(&blurred, &mut sobely, core::CV_8U, x_order, y_order, self.kernel_size, self.scale, self.delta, core::BORDER_DEFAULT) {
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
                                                                match core::add_weighted_def(&absx, self.alpha, &absy, self.beta, self.gamma, &mut grad) {
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