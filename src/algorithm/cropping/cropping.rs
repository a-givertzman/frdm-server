use opencv::core::MatTraitConst;
use opencv::core;
use opencv::core::Mat;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite,
    EvalResult,
};
use crate::algorithm::cropping::CroppingCtx;
use crate::{Eval, domain::Image};
///
/// Takes source [Image]
/// Return cropped [Image]
pub struct Cropping {
    x: i32,
    width: i32,
    y: i32,
    height: i32,
    ctx: Box<dyn Eval<(), EvalResult>>,
}
//
//
impl Cropping{
    ///
    /// 
    /// 
    pub fn new(x: i32,width: i32, y: i32, height: i32, ctx: impl Eval<(), EvalResult> + 'static) -> Self {
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
        match self.ctx.eval(()) {
            Ok(ctx) => {
                        match Mat::roi(&frame.mat, core::Rect { x: self.x,y: self.y,width: self.width,height: self.height,}) {
                                Ok(cropped) => {
                                    let result = CroppingCtx {
                                        result: Image {
                                            width: cropped.cols() as usize,
                                            height: cropped.rows() as usize,
                                            timestamp: frame.timestamp,
                                            mat: cropped.clone_pointee(),
                                            bytes: frame.bytes,
                                        }
                                    };
                                    ctx.write(result)
                                },
                                Err(err) => Err(error.pass(err.to_string())),
                            }
                        }
            Err(err) => Err(error.pass(err)),
        }
    }
}