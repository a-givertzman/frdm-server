use std::time::Instant;
use opencv::core::MatTraitConst;
use sal_core::error::Error;
use crate::{
    algorithm::{BitwiseAndCtx, ContextRead, ContextWrite, EvalResult, ResultCtx},
    domain::{Eval, Image},
};
///
/// Returns Bitwise And of input frames
pub struct BitwiseAnd {
    ctx1: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    ctx2: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl BitwiseAnd {
    ///
    /// Returns [BitwiseAnd] new instance
    pub fn new(ctx1: impl Eval<Image, EvalResult> + Send + Sync + 'static, ctx2: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            ctx1: Box::new(ctx1),
            ctx2: Box::new(ctx2),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for BitwiseAnd {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("BitwiseAnd", "eval");
        match (self.ctx1.eval(frame.clone()), self.ctx2.eval(frame)) {
            (Ok(ctx1), Ok(ctx2)) => {
                let t = Instant::now();
                let src1: &ResultCtx = ctx1.read();
                let src1_mat = &src1.frame.mat;
                log::debug!("BitwiseAnd.eval | src1: {}x{}", src1_mat.cols(), src1_mat.rows());
                let src2: &ResultCtx = ctx2.read();
                let src2_mat = &src2.frame.mat;
                log::debug!("BitwiseAnd.eval | src1: {}x{}", src2_mat.cols(), src2_mat.rows());
                let mut dst = opencv::core::Mat::default();
                match opencv::core::bitwise_and(src1_mat, src2_mat, &mut dst, &opencv::core::no_array()) {
                    Ok(_) => {
                        let frame = Image::with(dst);
                        let bw_and = BitwiseAndCtx { frame: frame.clone() };
                        let ctx = ctx1.write(bw_and)?;
                        let result = ResultCtx { frame };
                        log::debug!("BitwiseAnd.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(result)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            (Ok(_), Err(err)) => Err(error.pass(err)),
            (Err(err), Ok(_)) => Err(error.pass(err)),
            (Err(err), Err(_)) => Err(error.pass(err)),
        }
    }
}
