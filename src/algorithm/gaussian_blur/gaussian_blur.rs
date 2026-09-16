use std::time::Instant;
use opencv::{core::{Mat, Size}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, ResultCtx},
    domain::{Eval, Image},
};
///
/// Apply Gaussian blur to the input image
pub struct GaussianBlur {
    kernel: [i32; 2],
    sigma: [f64; 2],
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
}
//
//
impl GaussianBlur {
    ///
    /// Returns [GaussianBlur] new instance
    pub fn new(kernel: [i32; 2], sigma: [f64; 2], ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            kernel,
            sigma,
            ctx: Box::new(ctx),
            debug,
        }
    }
}
//
//
impl Eval<Image, EvalResult> for GaussianBlur {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("GaussianBlur", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mut blurred = Mat::default();
                match imgproc::gaussian_blur(
                    &frame.mat,
                    &mut blurred,
                    Size::new(self.kernel[0], self.kernel[1]),
                    self.sigma[0], self.sigma[1],
                    opencv::core::BORDER_DEFAULT,
                ) {
                    Ok(_) => {
                        let frame = Image::from(blurred, meta);
                        // let ctx = if self.debug {
                        //     let result = GaussianBlurCtx { frame: frame.clone() };
                        //     ctx.write(result).map_err(|err| error.pass(err))?
                        // } else {
                        //     ctx
                        // };
                        let result = ResultCtx { val: frame };
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
