use std::{sync::Arc, time::Instant};
use opencv::core::MatTraitConst;
use sal_core::error::Error;
use sal_sync::thread_pool::Scheduler;
use crate::{
    algorithm::{
        AutoGamma, BitwiseAndCtx, ContextRead, ContextWrite, Cropping, EdgeDetection, EvalResult, FastUnion, GaussianBlur, Gray, Initial, InitialCtx, ResultCtx, TemporalFilter,
        Context,
    }, conf::Conf, domain::{Eval, Image, RwLock}, CvContours
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
pub struct FastScan {
    pass_gray1: Arc<RwLock<Option<Context>>>,
    pass_gray2: Arc<RwLock<Option<Context>>>,
    ctx_gray: Box<dyn Eval<Image, EvalResult>>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
//
//
impl FastScan {
    ///
    /// Returns [FastScan] new instance
    pub fn new(conf: Conf, scheduler: Scheduler) -> Self {
        let pass_gray1 = Arc::new(RwLock::new(None));
        let pass_gray2 = Arc::new(RwLock::new(None));
        Self {
            pass_gray1: pass_gray1.clone(),
            pass_gray2: pass_gray2.clone(),
            ctx_gray: Box::new(
                Gray::new(
                    AutoGamma::new(
                        conf.contours.gamma.factor,
                        Cropping::new(
                            conf.contours.cropping.x,
                            conf.contours.cropping.width,
                            conf.contours.cropping.y,
                            conf.contours.cropping.height,
                            Initial::new(
                                InitialCtx::new(),
                            ),
                        ),
                    ),
                ),
            ),
            ctx: Box::new(
                EdgeDetection::new(
                    conf.edge_detection.otsu_tune,
                    conf.edge_detection.threshold,
                    conf.edge_detection.smooth,
                    FastUnion::new(
                        scheduler,
                        // CvContours::new(
                        //     conf.contours.clone(),
                        TemporalFilter::new(
                            conf.contours.temporal_filter.amplify_factor,
                            conf.contours.temporal_filter.grow_speed,
                            conf.contours.temporal_filter.reduce_factor,
                            conf.contours.temporal_filter.down_speed,
                            conf.contours.temporal_filter.threshold,
                            GaussianBlur::new(
                                conf.contours.gausian.blur_w,
                                conf.contours.gausian.blur_h,
                                conf.contours.gausian.sigma_x,
                                conf.contours.gausian.sigma_y,
                                PassGray::new(pass_gray1)
                            ),
                        ),
                        CvContours::new(
                            conf.contours.clone(),
                            GaussianBlur::new(
                                conf.contours.gausian.blur_w,
                                conf.contours.gausian.blur_h,
                                conf.contours.gausian.sigma_x,
                                conf.contours.gausian.sigma_y,
                                PassGray::new(pass_gray2),
                            ),
                        )
                        // ),
                    ),
                ),
            ),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for FastScan {
    fn eval(&self, frame: Image) -> EvalResult {
        self.ctx.eval(frame).map_err(|err| Error::new("FastScan", "eval").pass(err))
    }
}
///
/// 
struct PassGray {
    ctx: Arc<RwLock<Option<Context>>>,
}
impl PassGray {
    fn new(ctx: Arc<RwLock<Option<Context>>>) -> Self {
        Self {
            ctx
        }
    }
}
impl Eval<Image, EvalResult> for PassGray {
    fn eval(&self, _: Image) -> EvalResult {
        match self.ctx.write().take() {
            Some(ctx) => Ok(ctx),
            None => Err(Error::new("PassGray", "eval").err("Can't take 'Context'")),
        }
    }
}
