use sal_core::error::Error;
use crate::{algorithm::{Context, InitialCtx}, domain::Eval, infrostructure::arena::Image};
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
impl Eval<(), Result<Context, Error>> for Initial {
    fn eval(&self, _: ()) -> Result<Context, Error> {
        let error = Error::new("Initial", "eval");
        Ok(Context::new(self.ctx));
    }
}
