use sal_core::error::Error;
use crate::{algorithm::{Context, EvalResult, InitialCtx, ContextWrite, ResultCtx}, domain::{Eval, Image}};
///
/// Takes [InitialCtx]
/// Returns [Context] with only [InitialCtx]
pub struct Initial {
    ctx: InitialCtx,
}
//
//
impl Initial{
    ///
    /// Returns [Initial] new instance
    pub fn new(ctx: InitialCtx) -> Self {
        Self { 
            ctx,
        }
    }
}
//
//
impl Eval<Image, EvalResult> for Initial {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("Initial", "eval");
        let ctx = Context::new(self.ctx.clone());
        let ctx = ctx.write(frame.meta).map_err(|err| error.pass(err))?;
        ctx.write(ResultCtx { val: frame })
    }
}
