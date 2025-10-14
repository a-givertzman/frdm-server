use std::time::Instant;
use sal_core::error::Error;
use sal_sync::thread_pool::Scheduler;
use crate::{
    algorithm::{
        ContextRead, EdgeDetection,
        EvalResult, FineContours, FineUnion, Initial,
        InitialCtx, ResultCtx, TemporalFilter, FineScanConf,
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
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl FineScan {
    ///
    /// Returns [FineScan] new instance
    pub fn new(conf: FineScanConf, scheduler: Scheduler, debug: bool) -> Self {
        Self {
            ctx: Box::new(
                EdgeDetection::new(
                    conf.edge_detection.otsu_tune,
                    conf.edge_detection.threshold,
                    conf.edge_detection.smooth,
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
                let result: &ResultCtx = ctx.read();
                let frame = result.frame.clone();
                let result = self.ctx.eval(frame).map_err(|err| error.pass(err));
                log::debug!("FineScan.eval | Elapsed: {:?}", t.elapsed());
                result
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
