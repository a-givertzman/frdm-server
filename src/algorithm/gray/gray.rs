use std::time::Instant;
use opencv::imgproc;
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, GrayCtx, ResultCtx},
    domain::{Eval, Image},
};
///
/// Converts input frame into gray scale
pub struct Gray {
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl Gray {
    ///
    /// Returns [Gray] new instance
    pub fn new(ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for Gray {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("Gray", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mut gray = opencv::core::Mat::default();
                match imgproc::cvt_color(&frame.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0) {
                    Ok(_) => {
                        let frame = Image::with(gray);
                        let ctx = ctx.write(GrayCtx { frame: frame.clone() }).map_err(|err| error.pass(err))?;
                        log::debug!("Gray.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(ResultCtx { val: frame })
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
