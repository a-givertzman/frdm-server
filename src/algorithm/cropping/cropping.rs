use opencv::core::MatTraitConst;
use opencv::core;
use opencv::core::Mat;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite,
    CroppingCtx,
    ContextRead,
    EvalResult,
    ResultCtx,
};
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return cropped [Image]
pub struct Cropping {
    x: i32,
    width: i32,
    y: i32,
    height: i32,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl Cropping {
    ///
    /// Cropping source image (from `ctx`) using specified parameters
    /// - `x` - new left edge
    /// - `width` - new image width
    /// - `y` - new top edge
    /// - `height` - new image height
    pub fn new(x: i32, width: i32, y: i32, height: i32, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self { 
            x,
            width,
            y,
            height,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for Cropping {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("Cropping", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let result: &ResultCtx = ctx.read();
                let frame = &result.frame;
                match Mat::roi(&frame.mat, core::Rect { x: self.x,y: self.y,width: self.width,height: self.height,}) {
                        Ok(cropped) => {
                            let frame = Image {
                                width: cropped.cols() as usize,
                                height: cropped.rows() as usize,
                                timestamp: frame.timestamp,
                                mat: cropped.clone_pointee(),
                                bytes: frame.bytes,
                            };
                            let result = CroppingCtx { result: frame.clone() };
                            let ctx = ctx.write(result)?;
                            let result = ResultCtx { frame };
                            ctx.write(result)
                        },
                        Err(err) => Err(error.pass(err.to_string())),
                    }
                }
            Err(err) => Err(error.pass(err)),
        }
    }
}
