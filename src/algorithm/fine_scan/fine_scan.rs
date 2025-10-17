use std::{sync::Arc, time::Instant};
use sal_core::error::Error;
use sal_sync::{services::future::Future, sync::Owner, thread_pool::Scheduler};
use crate::{
    algorithm::{
        self, Context, ContextRead, ContextWrite, EvalResult, FineContours,
        FineConvexCtx, FineEdges, FineScanConf, FineScanCtx, FineUnion,
        GeometryDefectCtx, ResultCtx, TemporalFilter,
    }, domain::{Eval, Image},
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
    defects: Option<Arc<Box<dyn Fn(&Context) + Send + Sync>>>,
    ctx_gray: Box<dyn Eval<Image, EvalResult>>,
    ctx: Arc<Box<dyn Eval<Image, EvalResult> + Send + Sync>>,
    scheduler: Scheduler,
}
//
//
impl FineScan {
    ///
    /// Returns [FineScan] new instance
    #[allow(unused)]
    pub fn new(
        conf: FineScanConf,
        scheduler: Scheduler,
        defects: Option<impl Fn(&Context) + 'static + Send + Sync>,
        ctx_gray: impl Eval<Image, EvalResult> + 'static,
        debug: bool,
    ) -> Self {
        let pass_gray1 = Arc::new(Owner::empty());
        let pass_gray2 = Arc::new(Owner::empty());
        Self {
            pass_ctx1: pass_gray1.clone(),
            pass_ctx2: pass_gray2.clone(),
            defects: match defects {
                Some(defects) => Some(Arc::new(Box::new(defects))),
                None => None,
            },
            ctx_gray: Box::new(ctx_gray),
            ctx: Arc::new(Box::new(
                FineEdges::new(
                    conf.fine_edges.otsu_tune,
                    conf.fine_edges.threshold,
                    conf.fine_edges.smooth,
                    FineUnion::new(
                        scheduler.clone(),
                        TemporalFilter::<FineScanCtx>::new(
                            conf.temporal_filter.gaussian,
                            conf.temporal_filter.open_kernel,
                            conf.temporal_filter.erode_kernel,
                            conf.temporal_filter.threshold,
                            PassGrayCtx::new(pass_gray1),
                            debug,
                        ),
                        FineContours::new(
                            conf.fine_contours,
                            PassGrayCtx::new(pass_gray2),
                            debug,
                        )
                    ),
                ),
            )),
            scheduler,
        }
    }
}
//
//
impl Eval<Image, Future<Result<Context, Error>>> for FineScan {
    fn eval(&self, frame: Image) -> Future<Result<Context, Error>> {
        let error = Error::new("FineScan", "eval");
        let (future, sink) = Future::new();
        let sink1 = sink.clone();
        let result = match self.ctx_gray.eval(frame) {
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
                log::debug!("FineScan.eval | GeometryDefectCtx size: {:?}", size_of_val(ContextRead::<GeometryDefectCtx<()>>::read(&ctx)));
                // log::debug!("FineScan.eval | InitialCtx size: {:?}", size_of_val(ContextRead::<algorithm::FastScanCtx>::read(&ctx)));
                log::debug!("FineScan.eval | frame size: {:?}", size_of_val(&frame));
                self.pass_ctx1.replace(ctx.clone());
                self.pass_ctx2.replace(ctx);
                let ctx_eval = self.ctx.clone();
                let defects = self.defects.clone();
                let handle = self.scheduler.spawn(move || {
                    let error = Error::new("FineScan", "eval");
                    let ctx = match ctx_eval.eval(frame) {
                        Ok(ctx) => {
                            let convex: &FineConvexCtx = ctx.read();
                            match &convex.convex {
                                Some(convex) => {
                                    let result: &ResultCtx<Image> = ctx.read();
                                    let mut dst = opencv::core::Mat::default();
                                    match opencv::core::bitwise_and(&result.val.mat, &convex.mat, &mut dst, &opencv::core::no_array()) {
                                        Ok(_) => {
                                            log::debug!("FineScan.eval | Elapsed: {:?}", t.elapsed());
                                            if let Some(defects) = defects {
                                                let defects_ctx: &GeometryDefectCtx<()> = ctx.read();
                                                if !defects_ctx.result.is_empty() {
                                                    (defects)(&ctx)
                                                }
                                            }
                                            ctx.write(ResultCtx { val: Image::with(dst) })
                                        }
                                        Err(err) => Err(error.pass(err.to_string())),
                                    }
                                }
                                None => Err(error.err("Can't get convex from context")),
                            }
                        }
                        Err(err) => Err(error.pass(err)),
                    };
                    match ctx {
                        Ok(ctx) => sink1.add(Ok(ctx)),
                        Err(err) => sink1.add(Err(err)),
                    };
                    Ok(())
                });
                handle.map_err(|err| error.pass(err))
            }
            Err(err) => Err(error.pass(err)),
        };
        match result {
            Ok(_) => future,
            Err(err) => {
                sink.add(Err(error.pass(err)));
                future
            }
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
