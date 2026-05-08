use std::{sync::Arc, time::Instant};
use opencv::core::MatTraitConst;
use sal_core::error::Error;
use sal_sync::{services::future::Future, thread_pool::Scheduler};
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, FineConvexCtx, FineUnionCtx, ResultCtx}, conf::{UnionConf, UnionKindConf}, domain::{Eval, Image}
};
///
/// Combines input from two parallel evaluation branches to build a definitive contour
pub struct FineUnion {
    conf: UnionConf,
    scheduler: Scheduler,
    ctx1: Arc<Box<dyn Eval<Image, EvalResult> + Send + Sync>>,
    ctx2: Arc<Box<dyn Eval<Image, EvalResult> + Send + Sync>>,
}
//
//
impl FineUnion {
    ///
    /// Returns [FineUnion] new instance
    /// - `conf`: Configuration for the merge strategy (e.g., bitwise operations or weighted sum)
    /// - `scheduler`: Thread pool for concurrent execution of evaluation branches
    /// - `ctx1`: First evaluation pipeline (e.g., `TemporalFilter`)
    /// - `ctx2`: Second evaluation pipeline (e.g., `FineContours`)
    pub fn new(
        conf: UnionConf,
        scheduler: Scheduler,
        ctx1: impl Eval<Image, EvalResult> + Send + Sync + 'static,
        ctx2: impl Eval<Image, EvalResult> + Send + Sync + 'static
    ) -> Self {
        Self {
            conf,
            scheduler,
            ctx1: Arc::new(Box::new(ctx1)),
            ctx2: Arc::new(Box::new(ctx2)),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FineUnion {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FineUnion", "eval");
        let meta = frame.meta;
        let (ctx1, sink) = Future::new();
        let ctx1_eval = self.ctx1.clone();
        let frame1 = frame.clone();
        self.scheduler.spawn(move || {
            let ctx = ctx1_eval.eval(frame1);
            sink.add(ctx);
            Ok(())
        }).map_err(|err| error.pass(err))?;
        let (ctx2, sink) = Future::new();
        let ctx2_eval = self.ctx2.clone();
        self.scheduler.spawn(move || {
            let ctx = ctx2_eval.eval(frame);
            sink.add(ctx);
            Ok(())
        }).map_err(|err| error.pass(err))?;
        let ctx1 = ctx1.wait()?;
        let ctx2 = ctx2.wait()?;
        match (ctx1, ctx2) {
            (Ok(ctx1), Ok(ctx2)) => {
                let t = Instant::now();
                let src1: &ResultCtx<Image> = ctx1.read();
                let src1_mat = &src1.val.mat;
                log::trace!("FineUnion.eval | src1: {}x{}", src1_mat.cols(), src1_mat.rows());
                let src2: &ResultCtx<Image> = ctx2.read();
                let src2_mat = &src2.val.mat;
                log::trace!("FineUnion.eval | src2: {}x{}", src2_mat.cols(), src2_mat.rows());
                let mut dst = opencv::core::Mat::default();
                match self.conf.kind {
                    UnionKindConf::BitwiseAnd(_conf) => opencv::core::bitwise_and(src1_mat, src2_mat, &mut dst, &opencv::core::no_array())
                        .map_err(|err| error.pass(err.to_string()))?,
                    UnionKindConf::AddWeighted(conf) => opencv::core::add_weighted_def(src1_mat, conf.weight1, src2_mat, conf.weight2, conf.gamma, &mut dst)
                        .map_err(|err| error.pass(err.to_string()))?,
                    UnionKindConf::BitwiseOr(_conf) => opencv::core::bitwise_or(src1_mat, src2_mat, &mut dst, &opencv::core::no_array())
                        .map_err(|err| error.pass(err.to_string()))?,
                }
                let convex1: &FineConvexCtx = ctx1.read();
                let convex2: &FineConvexCtx = ctx2.read();
                let ctx = match (&convex1.convex, &convex2.convex) {
                    (None, None) => Err(error.err("Can't find convex in the context"))?,
                    (None, Some(convex)) => {
                        let mut out = opencv::core::Mat::default();
                        opencv::core::bitwise_and(&dst, &convex.mat, &mut out, &opencv::core::no_array()).map_err(|err| error.pass(err.to_string()))?;
                        dst = out;
                        ctx2
                    }
                    (Some(convex), None) => {
                        let mut out = opencv::core::Mat::default();
                        opencv::core::bitwise_and(&dst, &convex.mat, &mut out, &opencv::core::no_array()).map_err(|err| error.pass(err.to_string()))?;
                        dst = out;
                        ctx1
                    }
                    (Some(c1), Some(c2)) => {
                        let mut convex = opencv::core::Mat::default();
                        opencv::core::bitwise_and(&c1.mat, &c2.mat, &mut convex, &opencv::core::no_array()).map_err(|err| error.pass(err.to_string()))?;
                        let mut out = opencv::core::Mat::default();
                        opencv::core::bitwise_and(&dst, &convex, &mut out, &opencv::core::no_array()).map_err(|err| error.pass(err.to_string()))?;
                        dst = out;
                        ctx1
                    }
                };
                let frame = Image::from(dst, meta);
                let union = FineUnionCtx { frame: frame.clone() };
                let ctx = ctx.write(union).map_err(|err| error.pass(err))?;
                let result = ResultCtx { val: frame };
                log::trace!("FineUnion.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            (Err(err), _) | (_, Err(err)) => Err(error.pass(err)),
        }
    }
}
