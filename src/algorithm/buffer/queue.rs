use std::collections::VecDeque;
use sal_core::error::Error;
use crate::algorithm::{
    cv, ContextWrite, ContextRead, EvalResult, ResultCtx,
};
use crate::{Eval, domain::{Image, RwLock}};
///
/// Implements First In First Out (FIFO) principle with the specified length
pub struct Queue {
    len: usize,
    buf: RwLock<VecDeque<Image>>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl Queue {
    ///
    /// Returns [Queue] new instance
    /// - `len` - the length of the [Queue]
    #[allow(unused)]
    pub fn new(len: usize, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self { 
            len,
            buf: RwLock::new(VecDeque::new()),
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for Queue {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("Queue", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let result: &ResultCtx<Image> = ctx.read();
                self.buf.write().push_back(result.val.clone());
                let frame = if self.buf.read().len() > self.len {
                    match self.buf.write().pop_front() {
                        Some(frame) => frame,
                        None => Image::from(
                            cv::CreateMat::new(result.val.width(), result.val.height(), cv::MatType::Cv8uc1)
                                .eval(vec![0u8])
                                .map_err(|err| error.pass(err))?,
                            meta,
                        ),
                    }
                } else {
                    Image::from(
                        cv::CreateMat::new(result.val.width(), result.val.height(), cv::MatType::Cv8uc1)
                            .eval(vec![0u8])
                            .map_err(|err| error.pass(err))?,
                        meta,
                    )
                };
                // let ctx = if self.debug {
                //     let result = CroppingCtx { result: frame.clone() };
                //     ctx.write(result).map_err(|err| error.pass(err))?
                // } else {
                //     ctx
                // };
                let result = ResultCtx { val: frame };
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
