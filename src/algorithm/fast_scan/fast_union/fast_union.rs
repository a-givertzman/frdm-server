use std::{sync::Arc, time::Instant};
use opencv::core::{MatTraitConst, Point2i, Size2i};
use sal_core::error::Error;
use sal_sync::{services::future::Future, thread_pool::Scheduler};
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, FastUnionCtx, ResultCtx},
    domain::{Eval, Image, RwLock},
};
///
/// Combine input contours
pub struct FastUnion {
    scheduler: Scheduler,
    ctx1: Arc<RwLock<Box<dyn Eval<Image, EvalResult> + Send + Sync + Send + Sync>>>,
    ctx2: Arc<RwLock<Box<dyn Eval<Image, EvalResult> + Send + Sync + Send + Sync>>>,
}
//
//
impl FastUnion {
    ///
    /// Returns [FastUnion] new instance
    pub fn new(
        scheduler: Scheduler,
        ctx1: impl Eval<Image, EvalResult> + Send + Sync + Send + Sync + 'static,
        ctx2: impl Eval<Image, EvalResult> + Send + Sync + Send + Sync + 'static,
    ) -> Self {
        Self {
            scheduler,
            ctx1: Arc::new(RwLock::new(Box::new(ctx1))),
            ctx2: Arc::new(RwLock::new(Box::new(ctx2))),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FastUnion {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FastUnion", "eval");
        let (ctx1, sink) = Future::new();
        let ctx1_eval = self.ctx1.clone();
        let frame1 = frame.clone();
        self.scheduler.spawn(move || {
            let ctx = ctx1_eval.read().eval(frame1);
            sink.add(ctx);
            Ok(())
        }).map_err(|err| error.pass(err))?;
        let (ctx2, sink) = Future::new();
        let ctx2_eval = self.ctx2.clone();
        self.scheduler.spawn(move || {
            let ctx = ctx2_eval.read().eval(frame);
            sink.add(ctx);
            Ok(())
        }).map_err(|err| error.pass(err))?;
        let ctx1 = ctx1.wait()?;
        let ctx2 = ctx2.wait()?;
        match (ctx1, ctx2) {
            (Ok(ctx1), Ok(ctx2)) => {
                let t = Instant::now();
                let src1: &ResultCtx = ctx1.read();
                let src1_mat = &src1.frame.mat;
                log::debug!("FastUnion.eval | src1: {}x{}", src1_mat.cols(), src1_mat.rows());
                let src2: &ResultCtx = ctx2.read();
                let src2_mat = &src2.frame.mat;
                log::debug!("FastUnion.eval | src1: {}x{}", src2_mat.cols(), src2_mat.rows());
                let mut dst = opencv::core::Mat::default();
                // match opencv::core::bitwise_and(src1_mat, src2_mat, &mut dst, &opencv::core::no_array()) {
                // match opencv::core::add(src1_mat, src2_mat, &mut dst, &opencv::core::no_array(), -1) {
                match opencv::core::add_weighted_def(src1_mat, 0.1, src2_mat, 1.0, 0.0, &mut dst) {
                    Ok(_) => {
                        let kernel = opencv::imgproc::get_structuring_element(opencv::imgproc::MORPH_ELLIPSE, Size2i::new(5, 5), Point2i::new(-1, -1)).unwrap();
                        opencv::imgproc::morphology_ex(
                            &dst.clone(),
                            &mut dst,
                            opencv::imgproc::MORPH_OPEN,
                            &kernel,
                            Point2i::new(-1, -1),
                            1,
                            opencv::core::BORDER_CONSTANT,
                            opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass(err.to_string()))?,
                        ).map_err(|err| error.pass(err.to_string()))?;

                        let frame = Image::with(dst);
                        let bw_and = FastUnionCtx { frame: frame.clone() };
                        let ctx = ctx1.write(bw_and)?;
                        let result = ResultCtx { frame };
                        log::debug!("FastUnion.eval | Elapsed: {:?}", t.elapsed());
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
