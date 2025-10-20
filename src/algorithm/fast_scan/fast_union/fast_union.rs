use std::{sync::Arc, time::Instant};
use opencv::core::MatTraitConst;
use sal_core::error::Error;
use sal_sync::{services::future::Future, thread_pool::Scheduler};
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, FastUnionCtx, ResultCtx}, conf::UnionConf, domain::{Eval, Image, RwLock}
};
///
/// Combine input contours
pub struct FastUnion {
    conf: UnionConf,
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
        conf: UnionConf,
        scheduler: Scheduler,
        ctx1: impl Eval<Image, EvalResult> + Send + Sync + Send + Sync + 'static,
        ctx2: impl Eval<Image, EvalResult> + Send + Sync + Send + Sync + 'static,
    ) -> Self {
        Self {
            conf,
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
                let src1: &ResultCtx<Image> = ctx1.read();
                let src1_mat = &src1.val.mat;
                log::debug!("FastUnion.eval | src1: {}x{}", src1_mat.cols(), src1_mat.rows());
                let src2: &ResultCtx<Image> = ctx2.read();
                let src2_mat = &src2.val.mat;
                log::debug!("FastUnion.eval | src1: {}x{}", src2_mat.cols(), src2_mat.rows());
                let mut dst = opencv::core::Mat::default();
                // match opencv::core::add(src1_mat, src2_mat, &mut dst, &opencv::core::no_array(), -1) {
                match (self.conf.add_weighted, self.conf.bitwise_and) {
                    (None, Some(_)) => opencv::core::bitwise_and(src1_mat, src2_mat, &mut dst, &opencv::core::no_array())
                        .map_err(|err| error.pass(err.to_string()))?,
                    (Some(conf), None) => opencv::core::add_weighted_def(src1_mat, conf.weight1, src2_mat, conf.weight2, conf.gamma, &mut dst)
                        .map_err(|err| error.pass(err.to_string()))?,
                    _ => return Err(error.err(format!("Both: 'add-weighted' and `bitwise-and` - are specified, please use one of"))),
                }
                // let convex1: &FineConvexCtx = ctx1.read();
                // let convex2: &FineConvexCtx = ctx2.read();
                // let ctx = match (&convex1.convex, &convex2.convex) {
                //     (None, None) => ctx1,
                //     (None, Some(_)) => ctx2,
                //     (Some(_), None) => ctx1,
                //     (Some(_), Some(_)) => ctx1,
                // };
                let frame = Image::with(dst);
                let union = FastUnionCtx { frame: frame.clone() };
                let ctx = ctx1.write(union).map_err(|err| error.pass(err))?;
                let result = ResultCtx { val: frame };
                log::debug!("FastUnion.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            (Ok(_), Err(err)) => Err(error.pass(err)),
            (Err(err), Ok(_)) => Err(error.pass(err)),
            (Err(err), Err(_)) => Err(error.pass(err)),
        }
    }
}
