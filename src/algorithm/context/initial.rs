use crate::{algorithm::{Context, InitialCtx, EvalResult}, domain::Eval};
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
impl Eval<(), EvalResult> for Initial {
    fn eval(&self, _: ()) -> EvalResult {
        // let error = Error::new("Initial", "eval");
        Ok(Context::new(self.ctx.clone()))
    }
}
