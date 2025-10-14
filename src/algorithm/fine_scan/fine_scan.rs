use std::{sync::Arc, time::Instant};
use sal_core::error::Error;
use sal_sync::{sync::Owner, thread_pool::Scheduler};
use crate::{
    algorithm::{
        ContextRead, FineEdges, EvalResult, FineContours, FineScanConf, FineUnion, Initial, InitialCtx, ResultCtx, TemporalFilter,
    },
    domain::{Eval, Image}, Context,
};
///
/// Contour detection algorithms optimized for speed, tradeoff in result quality
/// 
/// - Convert into gray scale
/// - Apply autogamma
/// - First way
///    - Find contours based on the sharpness (sopel gradient or laplacian)
/// - Second way
///    - Find contours based on the moving objhect (diff of same pixel betwee current and previouse frame)
/// - Union contours of two ways using bitwise operation
pub struct FineScan {
    pass_ctx1: Arc<Owner<Context>>,
    pass_ctx2: Arc<Owner<Context>>,
    ctx_gray: Box<dyn Eval<Image, EvalResult>>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl FineScan {
    ///
    /// Returns [FineScan] new instance
    #[allow(unused)]
    pub fn new(conf: FineScanConf, scheduler: Scheduler, debug: bool) -> Self {
        let pass_ctx1 = Arc::new(Owner::empty());
        let pass_ctx2 = Arc::new(Owner::empty());
        Self {
            pass_ctx1: pass_ctx1.clone(),
            pass_ctx2: pass_ctx2.clone(),
            ctx: Box::new(
                FineEdges::new(
                    conf.fine_edges.otsu_tune,
                    conf.fine_edges.threshold,
                    conf.fine_edges.smooth,
                    FineUnion::new(
                        scheduler,
                        TemporalFilter::new(
                            conf.temporal_filter.gaussian,
                            conf.temporal_filter.open_kernel,
                            conf.temporal_filter.erode_kernel,
                            conf.temporal_filter.threshold,
                            Initial::new(
                                InitialCtx::new(),
                            ),
                            debug,
                        ),
                        FineContours::new(
                            conf.fine_contours,
                            Initial::new(
                                InitialCtx::new(),
                            ),
                            debug,
                        )
                    ),
                ),
            ),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FineScan {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FineScan", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = result.val.clone();
                let result = self.ctx.eval(frame).map_err(|err| error.pass(err));
                log::debug!("FineScan.eval | Elapsed: {:?}", t.elapsed());
                result
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
