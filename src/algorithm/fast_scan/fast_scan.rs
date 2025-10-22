use std::{sync::Arc, time::Instant};
use sal_core::error::Error;
use sal_sync::{sync::Owner, thread_pool::Scheduler};
use crate::{
    algorithm::{
        Context, ContextRead, EvalResult, FastContours, FastEdges, FastScanConf, FastScanCtx, FastUnion, ResultCtx, TemporalFilter, Mad, RopeDistortions,
    },
    domain::{Eval, Image},
};
///
/// ## Contour detection algorithms optimized for speed, tradeoff in result quality
/// 
/// - Convert into gray scale
/// - Apply autogamma
/// - First way (execute in the separate thread)
///    - Find contours based on the sharpness (sopel gradient or laplacian)
/// - Second way (execute in the separate thread)
///    - Find contours based on the moving objhect (diff of same pixel betwee current and previouse frame)
/// - Union contours of two ways using bitwise/add_weighted operation
pub struct FastScan {
    pass_ctx1: Arc<Owner<Context>>,
    pass_ctx2: Arc<Owner<Context>>,
    ctx_gray: Box<dyn Eval<Image, EvalResult>>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl FastScan {
    ///
    /// Returns [FastScan] new instance
    #[allow(unused)]
    pub fn new(
        conf: FastScanConf,
        scheduler: Scheduler,
        ctx: impl Eval<Image, EvalResult> + Send + Sync + Send + Sync + 'static,
        debug: bool) -> Self {
        let pass_ctx1 = Arc::new(Owner::empty());
        let pass_ctx2 = Arc::new(Owner::empty());
        Self {
            pass_ctx1: pass_ctx1.clone(),
            pass_ctx2: pass_ctx2.clone(),
            ctx_gray: Box::new(ctx),
            ctx: Box::new(
                RopeDistortions::<FastScanCtx>::new(
                    conf.distortion_threshold,
                    *Box::new(Mad::new()),
                    FastEdges::new(
                        conf.fast_edges.otsu_tune,
                        conf.fast_edges.threshold,
                        conf.fast_edges.smooth,
                        FastUnion::new(
                            conf.union,
                            scheduler,
                            TemporalFilter::<FastScanCtx>::new(
                                conf.temporal_filter.gaussian,
                                conf.temporal_filter.open_kernel,
                                conf.temporal_filter.erode_kernel,
                                conf.temporal_filter.threshold,
                                PassGrayCtx::new(pass_ctx1),
                                debug,
                            ),
                            FastContours::new(
                                conf.fast_contours,
                                PassGrayCtx::new(pass_ctx2),
                                debug,
                            ),
                            debug,
                        ),
                    ),
                )
            ),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FastScan {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FastScan", "eval");
        match self.ctx_gray.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = result.val.clone();
                self.pass_ctx1.replace(ctx.clone());
                self.pass_ctx2.replace(ctx);
                let result = self.ctx.eval(frame).map_err(|err| error.pass(err));
                log::debug!("FastScan.eval | Elapsed: {:?}", t.elapsed());
                result
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
///
/// 
struct PassGrayCtx {
    ctx: Arc<Owner<Context>>,
}
impl PassGrayCtx {
    fn new(ctx: Arc<Owner<Context>>) -> Self {
        Self {
            ctx
        }
    }
}
impl Eval<Image, EvalResult> for PassGrayCtx {
    fn eval(&self, _: Image) -> EvalResult {
        match self.ctx.take() {
            Some(ctx) => Ok(ctx),
            None => Err(Error::new("PassGray", "eval").err("Can't take 'Context'")),
        }
    }
}
