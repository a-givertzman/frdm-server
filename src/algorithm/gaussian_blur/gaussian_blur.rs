use std::time::Instant;
use opencv::{core::{Mat, Size}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, GaussianBlurCtx, ResultCtx},
    domain::{Eval, Image},
};
///
/// Apply Gaussian blur to the input image
pub struct GaussianBlur {
    width: i32,
    height: i32,
    sigma_x: f64,
    sigma_y: f64,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl GaussianBlur {
    ///
    /// Returns [GaussianBlur] new instance
    pub fn new(width: usize, height: usize, sigma_x: f64, sigma_y: f64, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            sigma_x,
            sigma_y,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for GaussianBlur {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("GaussianBlur", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx = ctx.read();
                let frame = &result.frame;
                let mut blurred = Mat::default();
                let kernel_size = Size::new(self.width, self.height);
                match imgproc::gaussian_blur(&frame.mat, &mut blurred, kernel_size, self.sigma_x, self.sigma_y, opencv::core::BORDER_DEFAULT) {
                    Ok(_) => {
                        let frame = Image::with(blurred);
                        let blurred = GaussianBlurCtx { frame: frame.clone() };
                        let ctx = ctx.write(blurred)?;
                        let result = ResultCtx { frame };
                        log::debug!("GaussianBlur.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(result)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
