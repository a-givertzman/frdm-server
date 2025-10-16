use std::{sync::Arc, time::Instant};
use sal_core::error::Error;
use sal_sync::{sync::Owner, thread_pool::Scheduler};
use crate::{
    algorithm::{
        self, Context, ContextRead, EvalResult, FineContours, FineEdges, FineScanConf, FineScanCtx, FineUnion, ResultCtx, TemporalFilter
    },
    domain::{Eval, Image},
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
    pub fn new(conf: FineScanConf, scheduler: Scheduler, ctx_gray: impl Eval<Image, EvalResult> + 'static, debug: bool) -> Self {
        let pass_ctx1 = Arc::new(Owner::empty());
        let pass_ctx2 = Arc::new(Owner::empty());
        Self {
            pass_ctx1: pass_ctx1.clone(),
            pass_ctx2: pass_ctx2.clone(),
            ctx_gray: Box::new(ctx_gray),
            ctx: Box::new(
                FineEdges::new(
                    conf.fine_edges.otsu_tune,
                    conf.fine_edges.threshold,
                    conf.fine_edges.smooth,
                    FineUnion::new(
                        scheduler,
                        TemporalFilter::<FineScanCtx>::new(
                            conf.temporal_filter.gaussian,
                            conf.temporal_filter.open_kernel,
                            conf.temporal_filter.erode_kernel,
                            conf.temporal_filter.threshold,
                            PassGrayCtx::new(pass_ctx1),
                            debug,
                        ),
                        FineContours::new(
                            conf.fine_contours,
                            PassGrayCtx::new(pass_ctx2),
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
        match self.ctx_gray.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = result.val.clone();
                log::debug!("FineScan.eval | ctx size: {:?}", size_of_val(&ctx));
                log::debug!("FineScan.eval | Image size: {:?}", size_of_val(&Image::default()));
                log::debug!("FineScan.eval | InitialCtx size: {:?}", size_of_val(ContextRead::<algorithm::InitialCtx>::read(&ctx)));
                log::debug!("FineScan.eval | NormalizedCtx size: {:?}", size_of_val(ContextRead::<algorithm::NormalizedCtx>::read(&ctx)));
                log::debug!("FineScan.eval | FastScanCtx size: {:?}", size_of_val(ContextRead::<algorithm::FastScanCtx>::read(&ctx)));
                log::debug!("FineScan.eval | FineScanCtx size: {:?}", size_of_val(ContextRead::<algorithm::FineScanCtx>::read(&ctx)));
                log::debug!("FineScan.eval | ResultCtx<Vec<GeometryDefectType>> size: {:?}", size_of_val(ContextRead::<algorithm::ResultCtx<Vec<algorithm::GeometryDefectType>>>::read(&ctx)));
                // log::debug!("FineScan.eval | InitialCtx size: {:?}", size_of_val(ContextRead::<algorithm::FastScanCtx>::read(&ctx)));
                log::debug!("FineScan.eval | frame size: {:?}", size_of_val(&frame));
                opencv::highgui::imshow("Gray", &frame.mat).unwrap();
                opencv::highgui::wait_key(0).unwrap();
                self.pass_ctx1.replace(ctx.clone());
                self.pass_ctx2.replace(ctx);
                let result = self.ctx.eval(frame).map_err(|err| error.pass(err));
                log::debug!("FineScan.eval | Elapsed: {:?}", t.elapsed());
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
