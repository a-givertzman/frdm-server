extern crate frdm_tools;
mod algorithm {
mod auto_correction {
mod auto_brightness_and_contrast {
use std::time::Instant;
use opencv::core::{Mat, MatTraitConst, Vector};
use opencv::imgproc;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite, ContextRead, AutoBrightnessAndContrastCtx,
    EvalResult, ResultCtx,
};
use crate::{Eval, domain::Image};
pub struct AutoBrightnessAndContrast {
    clip_left: f32,
    clip_right: f32,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
}
impl AutoBrightnessAndContrast {
    pub fn new(clip_left: f32, clip_right: f32, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            clip_left,
            clip_right,
            ctx: Box::new(ctx),
            debug,
        }
    }
}
impl Eval<Image, EvalResult> for AutoBrightnessAndContrast {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("AutoBrightnessAndContrast", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mut gray = Mat::default();
                match imgproc::cvt_color(&frame.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0) {
                    Ok(_) => {
                        let mut hist = Mat::default();
                        let hist_size = 256 as i32;
                        let imgs: Vector<Mat> = Vector::from_iter([gray.clone()]);
                        match opencv::imgproc::calc_hist(
                            &imgs,
                            &Vector::from_slice(&[0]),
                            &Mat::default(),
                            &mut hist,
                            &Vector::from_slice(&[hist_size]),
                            &Vector::from_slice(&[0.0 ,256.0]),
                            false,
                        ) {
                            Ok(_) => {
                                let mut accumulator = vec![];
                                match hist.at::<f32>(0) {
                                    Ok(val) => accumulator.push(*val),
                                    Err(err) => return Err(error.pass(err.to_string())),
                                }
                                for index in 1..(hist_size as usize) {
                                    match hist.at::<f32>(index as i32) {
                                        Ok(val) => {
                                            if let Some(acc_val) = accumulator.get(index -1) {
                                                accumulator.push(acc_val + * val)
                                            }
                                        }
                                        Err(err) => return Err(error.pass(err.to_string())),
                                    }
                                }
                                let maximum = match accumulator.last() {
                                    Some(max) => max,
                                    None => return Err(error.pass("Empty `accumulator`"))
                                };
                                log::debug!("AutoBrightnessAndContrast.eval | maximum: {:?}", maximum);
                                let clip_hist_left = self.clip_left * maximum / 100.0;
                                let clip_hist_right = self.clip_right * maximum / 100.0;
                                let mut minimum_gray = 0;
                                for i in 0..accumulator.len() {
                                    minimum_gray = i;
                                    if !(accumulator[i] < clip_hist_left) {
                                        break;
                                    }
                                }
                                let mut maximum_gray = (hist_size - 1) as usize;
                                for i in (0..accumulator.len()).rev() {
                                    maximum_gray = i;
                                    if !(accumulator[i] >= (maximum - clip_hist_right)) {
                                        break;
                                    }
                                }
                                log::debug!("AutoBrightnessAndContrast.eval | minimum_gray: {:?}", minimum_gray);
                                log::debug!("AutoBrightnessAndContrast.eval | maximum_gray: {:?}", maximum_gray);
                                let alpha = 255.0 / ((maximum_gray - minimum_gray) as f64);
                                let beta = - (minimum_gray as f64) * alpha;
                                let mut dst = Mat::default();
                                match opencv::core::convert_scale_abs(&frame.mat, &mut dst, alpha * 1.99, beta) {
                                    Ok(_) => {
                                        let frame = Image {
                                            meta: frame.meta,
                                            mat: dst,
                                        };
                                        let ctx = if self.debug {
                                            let result = AutoBrightnessAndContrastCtx { result: frame.clone() };
                                            ctx.write(result).map_err(|err| error.pass(err))?
                                        } else {
                                            ctx
                                        };
                                        let result = ResultCtx { val: frame };
                                        log::debug!("AutoBrightnessAndContrast.eval | Elapsed: {:?}", t.elapsed());
                                        ctx.write(result)
                                    }
                                    Err(err) => Err(error.pass(err.to_string())),
                                }
                            },
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}}
mod auto_brightness_and_contrast_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct AutoBrightnessAndContrastCtx {
    pub result: Image,
}
impl Default for AutoBrightnessAndContrastCtx {
    fn default() -> Self {
        Self {
            result: Image::default()
         }
    }
}}
mod auto_gamma {
use std::time::Instant;
use opencv::core::Mat;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite, EvalResult, AutoGammaCtx, ContextRead, ResultCtx,
};
use crate::{Eval, domain::Image};
pub struct AutoGamma {
    factor: f64,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
}
impl AutoGamma {
    pub fn new(factor: f64, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            factor: factor,
            ctx: Box::new(ctx),
            debug,
        }
    }
}
impl Eval<Image, EvalResult> for AutoGamma {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("AutoGamma", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let factor = self.factor / 100.0;
                let mid = 0.5f64;
                match opencv::core::mean(&frame.mat, &Mat::default()){
                    Ok(mean_result) => {
                        let mean = mean_result.into_iter().take(3).map(|v| v as f64).sum::<f64>() / 3.0;
                        let gamma: f64 = (mid * 255.0).ln()/mean.ln();
                        let inv_gamma = 1.0 / (factor * gamma);
                        let table: Vec<_> = (0..256).map(|i| (255.0 * ((i as f64 / 255.0).powf(inv_gamma))) as u8 ).collect();
                        match Mat::from_slice(&table){
                            Ok(table_mat) => {
                                let mut dst = Mat::default();
                                match opencv::core::lut(&frame.mat, &table_mat, &mut dst){
                                    Ok(_) =>{
                                        let frame = Image {
                                            meta: frame.meta,
                                            mat: dst,
                                        };
                                        let ctx = if self.debug {
                                            let result = AutoGammaCtx { result: frame.clone() };
                                            ctx.write(result).map_err(|err| error.pass(err))?
                                        } else {
                                            ctx
                                        };
                                        let result = ResultCtx { val: frame };
                                        log::trace!("AutoGamma.eval | Elapsed: {:?}", t.elapsed());
                                        ctx.write(result)
                                    }
                                    Err(err) => Err(error.pass(err.to_string())),
                                }
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}}
mod auto_gamma_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct AutoGammaCtx {
    pub result: Image,
}
impl Default for AutoGammaCtx {
    fn default() -> Self {
        Self {
            result: Image::default()
         }
    }
}}
pub use auto_brightness_and_contrast::*;
pub use auto_brightness_and_contrast_ctx::*;
pub use auto_gamma::*;
pub use auto_gamma_ctx::*;}
mod buffer {
mod queue {
use std::collections::VecDeque;
use sal_core::error::Error;
use crate::algorithm::{
    cv, ContextWrite, ContextRead, EvalResult, ResultCtx,
};
use crate::{Eval, domain::{Image, RwLock}};
pub struct Queue {
    len: usize,
    buf: RwLock<VecDeque<Image>>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
impl Queue {
    #[allow(unused)]
    pub fn new(len: usize, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            len,
            buf: RwLock::new(VecDeque::new()),
            ctx: Box::new(ctx),
        }
    }
}
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
                let result = ResultCtx { val: frame };
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
pub use queue::*;}
mod context {
mod context_access {
mod context_access {
use crate::{
    algorithm::{
        Context, ContextRead, ContextWrite, FastScanCtx, FineScanCtx, InitialCtx, NormalizedCtx, FineConvexCtx, ResultCtx, MetaCtx,
    },
    domain::{Error, Image},
};
impl ContextWrite<MetaCtx> for Context {
    fn write(mut self, value: MetaCtx) -> Result<Self, Error> {
        self.meta = value;
        Result::Ok(self)
    }
}
impl ContextRead<MetaCtx> for Context {
    fn read(&self) -> &MetaCtx {
        &self.meta
    }
}
impl ContextWrite<InitialCtx> for Context {
    fn write(mut self, value: InitialCtx) -> Result<Self, Error> {
        self.initial = value;
        Result::Ok(self)
    }
}
impl ContextRead<InitialCtx> for Context {
    fn read(&self) -> &InitialCtx {
        &self.initial
    }
}
impl ContextWrite<ResultCtx<Image>> for Context {
    fn write(mut self, value: ResultCtx<Image>) -> Result<Self, Error> {
        self.result = value;
        Result::Ok(self)
    }
}
impl ContextRead<ResultCtx<Image>> for Context {
    fn read(&self) -> &ResultCtx<Image> {
        &self.result
    }
}
impl ContextWrite<NormalizedCtx> for Context {
    fn write(mut self, value: NormalizedCtx) -> Result<Self, Error> {
        self.normalized = value;
        Result::Ok(self)
    }
}
impl ContextRead<NormalizedCtx> for Context {
    fn read(&self) -> &NormalizedCtx {
        &self.normalized
    }
}
impl ContextWrite<FastScanCtx> for Context {
    fn write(mut self, value: FastScanCtx) -> Result<Self, Error> {
        self.fast_scan = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastScanCtx> for Context {
    fn read(&self) -> &FastScanCtx {
        &self.fast_scan
    }
}
impl ContextWrite<FineScanCtx> for Context {
    fn write(mut self, value: FineScanCtx) -> Result<Self, Error> {
        self.fine_scan = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineScanCtx> for Context {
    fn read(&self) -> &FineScanCtx {
        &self.fine_scan
    }
}
impl ContextWrite<FineConvexCtx> for Context {
    fn write(mut self, value: FineConvexCtx) -> Result<Self, Error> {
        self.convex = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineConvexCtx> for Context {
    fn read(&self) -> &FineConvexCtx {
        &self.convex
    }
}
}
mod fast_scan_access {
use crate::{
    algorithm::{
        FastContoursCtx, FastEdgesCtx, FastScanCtx, FastUnionCtx,
        RopeDimensionsCtx, TemporalFilterCtx, RopeDistortionsCtx,
        Context, ContextRead, ContextWrite,
    },
    domain::Error,
};
impl ContextWrite<FastContoursCtx> for Context {
    fn write(mut self, value: FastContoursCtx) -> Result<Self, Error> {
        self.fast_scan.fast_contours = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastContoursCtx> for Context {
    fn read(&self) -> &FastContoursCtx {
        &self.fast_scan.fast_contours
    }
}
impl ContextWrite<FastEdgesCtx> for Context {
    fn write(mut self, value: FastEdgesCtx) -> Result<Self, Error> {
        self.fast_scan.edges = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastEdgesCtx> for Context {
    fn read(&self) -> &FastEdgesCtx {
        &self.fast_scan.edges
    }
}
impl ContextWrite<RopeDistortionsCtx<FastScanCtx>> for Context {
    fn write(mut self, value: RopeDistortionsCtx<FastScanCtx>) -> Result<Self, Error> {
        self.fast_scan.distortions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDistortionsCtx<FastScanCtx>> for Context {
    fn read(&self) -> &RopeDistortionsCtx<FastScanCtx> {
        &self.fast_scan.distortions
    }
}
impl ContextWrite<RopeDimensionsCtx<FastScanCtx>> for Context {
    fn write(mut self, value: RopeDimensionsCtx<FastScanCtx>) -> Result<Self, Error> {
        self.fast_scan.rope_dimensions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDimensionsCtx<FastScanCtx>> for Context {
    fn read(&self) -> &RopeDimensionsCtx<FastScanCtx> {
        &self.fast_scan.rope_dimensions
    }
}
impl ContextWrite<TemporalFilterCtx<FastScanCtx>> for Context {
    fn write(mut self, value: TemporalFilterCtx<FastScanCtx>) -> Result<Self, Error> {
        self.fast_scan.temporal_filter = value;
        Result::Ok(self)
    }
}
impl ContextRead<TemporalFilterCtx<FastScanCtx>> for Context {
    fn read(&self) -> &TemporalFilterCtx<FastScanCtx> {
        &self.fast_scan.temporal_filter
    }
}
impl ContextWrite<FastUnionCtx> for Context {
    fn write(mut self, value: FastUnionCtx) -> Result<Self, Error> {
        self.fast_scan.union = value;
        Result::Ok(self)
    }
}
impl ContextRead<FastUnionCtx> for Context {
    fn read(&self) -> &FastUnionCtx {
        &self.fast_scan.union
    }
}
}
mod fine_scan_access {
use crate::{
    algorithm::{
        FineContoursCtx, FineScanCtx, FineUnionCtx, RopeDefectCtx, RopeDimensionsCtx, TemporalFilterCtx, RopeDistortionsCtx, FineEdgesCtx,
        Context, ContextRead, ContextWrite,
    },
    domain::Error,
};
impl ContextWrite<RopeDistortionsCtx<FineScanCtx>> for Context {
    fn write(mut self, value: RopeDistortionsCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.width_emissions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDistortionsCtx<FineScanCtx>> for Context {
    fn read(&self) -> &RopeDistortionsCtx<FineScanCtx> {
        &self.fine_scan.width_emissions
    }
}
impl ContextWrite<RopeDefectCtx<FineScanCtx>> for Context {
    fn write(mut self, value: RopeDefectCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.defects = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDefectCtx<FineScanCtx>> for Context {
    fn read(&self) -> &RopeDefectCtx<FineScanCtx> {
        &self.fine_scan.defects
    }
}
impl ContextWrite<RopeDimensionsCtx<FineScanCtx>> for Context {
    fn write(mut self, value: RopeDimensionsCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.rope_dimensions = value;
        Result::Ok(self)
    }
}
impl ContextRead<RopeDimensionsCtx<FineScanCtx>> for Context {
    fn read(&self) -> &RopeDimensionsCtx<FineScanCtx> {
        &self.fine_scan.rope_dimensions
    }
}
impl ContextWrite<TemporalFilterCtx<FineScanCtx>> for Context {
    fn write(mut self, value: TemporalFilterCtx<FineScanCtx>) -> Result<Self, Error> {
        self.fine_scan.temporal_filter = value;
        Result::Ok(self)
    }
}
impl ContextRead<TemporalFilterCtx<FineScanCtx>> for Context {
    fn read(&self) -> &TemporalFilterCtx<FineScanCtx> {
        &self.fine_scan.temporal_filter
    }
}
impl ContextWrite<FineEdgesCtx> for Context {
    fn write(mut self, value: FineEdgesCtx) -> Result<Self, Error> {
        self.fine_scan.edges = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineEdgesCtx> for Context {
    fn read(&self) -> &FineEdgesCtx {
        &self.fine_scan.edges
    }
}
impl ContextWrite<FineUnionCtx> for Context {
    fn write(mut self, value: FineUnionCtx) -> Result<Self, Error> {
        self.fine_scan.union = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineUnionCtx> for Context {
    fn read(&self) -> &FineUnionCtx {
        &self.fine_scan.union
    }
}
impl ContextWrite<FineContoursCtx> for Context {
    fn write(mut self, value: FineContoursCtx) -> Result<Self, Error> {
        self.fine_scan.fine_contours = value;
        Result::Ok(self)
    }
}
impl ContextRead<FineContoursCtx> for Context {
    fn read(&self) -> &FineContoursCtx {
        &self.fine_scan.fine_contours
    }
}
}
mod normalized_access {
use crate::{
    algorithm::{
        auto_correction::{AutoBrightnessAndContrastCtx, AutoGammaCtx}, Context, ContextRead, ContextWrite, CroppingCtx, GrayCtx,
    }, domain::Error,
};
impl ContextWrite<CroppingCtx> for Context {
    fn write(mut self, value: CroppingCtx) -> Result<Self, Error> {
        self.normalized.cropping = value;
        Result::Ok(self)
    }
}
impl ContextRead<CroppingCtx> for Context {
    fn read(&self) -> &CroppingCtx {
        &self.normalized.cropping
    }
}
impl ContextWrite<AutoGammaCtx> for Context {
    fn write(mut self, value: AutoGammaCtx) -> Result<Self, Error> {
        self.normalized.auto_gamma = value;
        Result::Ok(self)
    }
}
impl ContextRead<AutoGammaCtx> for Context {
    fn read(&self) -> &AutoGammaCtx {
        &self.normalized.auto_gamma
    }
}
impl ContextWrite<AutoBrightnessAndContrastCtx> for Context {
    fn write(mut self, value: AutoBrightnessAndContrastCtx) -> Result<Self, Error> {
        self.normalized.auto_brightness_and_contrast = value;
        Result::Ok(self)
    }
}
impl ContextRead<AutoBrightnessAndContrastCtx> for Context {
    fn read(&self) -> &AutoBrightnessAndContrastCtx {
        &self.normalized.auto_brightness_and_contrast
    }
}
impl ContextWrite<GrayCtx> for Context {
    fn write(mut self, value: GrayCtx) -> Result<Self, Error> {
        self.normalized.gray = value;
        Result::Ok(self)
    }
}
impl ContextRead<GrayCtx> for Context {
    fn read(&self) -> &GrayCtx {
        &self.normalized.gray
    }
}
}
pub use context_access::*;
pub use fast_scan_access::*;
pub use fine_scan_access::*;
pub use normalized_access::*;
use sal_core::error::Error;
use crate::algorithm::Context;
pub trait ContextWrite<T> {
    fn write(self, value: T) -> Result<Context, Error>;
}
pub trait ContextRead<T> {
    fn read(&self) -> &T;
}
}
mod context {
use crate::{
    algorithm::{
        FastScanCtx, FineConvexCtx, FineScanCtx, InitialCtx, NormalizedCtx, ResultCtx, TestingCtx,
    },
    domain::Image,
};
pub type MetaCtx = usize;
#[derive(Debug, Clone)]
pub struct Context {
    pub(super) meta: MetaCtx,
    pub(super) initial: InitialCtx,
    pub(super) result: ResultCtx<Image>,
    pub(super) normalized: NormalizedCtx,
    pub(super) fast_scan: FastScanCtx,
    pub(super) fine_scan: FineScanCtx,
    pub(super) convex: FineConvexCtx,
    #[allow(dead_code)]
    pub testing: Option<TestingCtx>,
}
impl Context {
    pub fn new(initial: InitialCtx) -> Self {
        Self {
            meta: 0,
            initial,
            result: ResultCtx::default(),
            normalized: NormalizedCtx::default(),
            fast_scan: FastScanCtx::default(),
            fine_scan: FineScanCtx::default(),
            convex: FineConvexCtx::default(),
            testing: None,
        }
    }
}
    }
mod initial {
use sal_core::error::Error;
use crate::{algorithm::{Context, EvalResult, InitialCtx, ContextWrite, ResultCtx}, domain::{Eval, Image}};
pub struct Initial {
    ctx: InitialCtx,
}
impl Initial{
    pub fn new(ctx: InitialCtx) -> Self {
        Self {
            ctx,
        }
    }
}
impl Eval<Image, EvalResult> for Initial {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("Initial", "eval");
        let ctx = Context::new(self.ctx.clone());
        let ctx = ctx.write(frame.meta).map_err(|err| error.pass(err))?;
        ctx.write(ResultCtx { val: frame })
    }
}
}
mod normalised_ctx {
use crate::
    algorithm::{
        auto_correction::{AutoBrightnessAndContrastCtx, AutoGammaCtx},
        CroppingCtx, GrayCtx,
    }
;
#[derive(Debug, Clone)]
pub struct NormalizedCtx {
    pub(super) cropping: CroppingCtx,
    pub(super) auto_gamma: AutoGammaCtx,
    pub(super) auto_brightness_and_contrast: AutoBrightnessAndContrastCtx,
    pub(super) gray: GrayCtx,
}
impl Default for NormalizedCtx {
    fn default() -> Self {
        Self {
            cropping: CroppingCtx::default(),
            auto_gamma: AutoGammaCtx::default(),
            auto_brightness_and_contrast: AutoBrightnessAndContrastCtx::default(),
            gray: GrayCtx::default(),
        }
    }
}
}
mod result_ctx {
#[derive(Debug, Clone)]
pub struct ResultCtx<T> {
    pub val: T,
}
impl<T: Default> Default for ResultCtx<T> {
    fn default() -> Self {
        Self {
            val: T::default()
         }
    }
}
}
mod testing_ctx {
use testing::entities::test_value::Value;
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestingCtx {
    pub mok_user_reply: MokUserReplyTestCtx,
}
#[derive(Debug, Clone, PartialEq)]
pub struct MokUserReplyTestCtx {
    pub value: Value,
}}
pub use context_access::*;
pub use context::*;
pub use initial::*;
pub use normalised_ctx::*;
pub use result_ctx::*;
#[allow(unused)]
pub use testing_ctx::*;
use crate::domain::Error;
pub type EvalResult = Result<Context, Error>;}
mod cropping {
mod cropping_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CroppingConf {
    pub x: i32,
    pub width: i32,
    pub y: i32,
    pub height: i32,
}
impl CroppingConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "CroppingConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let x: i64 = conf.get("x").expect(&format!("{dbg}.new | 'x' - not found or wrong configuration"));
        log::trace!("{dbg}.new | x: {:?}", x);
        let width: i64 = conf.get("width").expect(&format!("{dbg}.new | 'width' - not found or wrong configuration"));
        log::trace!("{dbg}.new | width: {:?}", width);
        let y: i64 = conf.get("y").expect(&format!("{dbg}.new | 'y' - not found or wrong configuration"));
        log::trace!("{dbg}.new | y: {:?}", y);
        let height: i64 = conf.get("height").expect(&format!("{dbg}.new | 'height' - not found or wrong configuration"));
        log::trace!("{dbg}.new | height: {:?}", height);
        Self {
            x: x as i32,
            width: width as i32,
            y: y as i32,
            height: height as i32,
        }
    }
}
impl Default for CroppingConf {
    fn default() -> Self {
        Self {
            x: 0,
            width: 1920,
            y: 0,
            height: 1200,
        }
    }
}
}
mod cropping {
use opencv::core;
use opencv::core::Mat;
use sal_core::error::Error;
use crate::algorithm::{
    ContextWrite,
    CroppingCtx,
    ContextRead,
    EvalResult,
    ResultCtx,
};
use crate::{Eval, domain::Image};
pub struct Cropping {
    x: i32,
    width: i32,
    y: i32,
    height: i32,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
}
impl Cropping {
    pub fn new(x: i32, width: i32, y: i32, height: i32, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            x,
            width,
            y,
            height,
            ctx: Box::new(ctx),
            debug,
        }
    }
}
impl Eval<Image, EvalResult> for Cropping {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("Cropping", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                match Mat::roi(&frame.mat, core::Rect { x: self.x,y: self.y,width: self.width,height: self.height,}) {
                    Ok(cropped) => {
                        let frame = Image {
                            meta: frame.meta,
                            mat: cropped.clone_pointee(),
                        };
                        let ctx = if self.debug {
                            let result = CroppingCtx { result: frame.clone() };
                            ctx.write(result).map_err(|err| error.pass(err))?
                        } else {
                            ctx
                        };
                        let result = ResultCtx { val: frame };
                        ctx.write(result)
                    },
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
mod cropping_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct CroppingCtx {
    pub result: Image,
}
impl Default for CroppingCtx {
    fn default() -> Self {
        Self {
            result: Image::default()
         }
    }
}
}
pub use cropping_conf::*;
pub use cropping::*;
pub use cropping_ctx::*;}
pub mod cv {
mod adaptive_threshold {
use opencv::{
    core::Mat, imgproc::{AdaptiveThresholdTypes, ThresholdTypes},
};
use sal_core::error::Error;
use crate::Eval;
pub struct AdaptiveThreshold<In> {
    maxval: f64,
    method: AdaptiveThresholdTypes,
    typ: ThresholdTypes,
    block_size: i32,
    decrement: f64,
    ctx: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
}
impl<In> AdaptiveThreshold<In> {
    #[allow(unused)]
    pub fn new(
        maxval: f64,
        method: AdaptiveThresholdTypes,
        typ: ThresholdTypes,
        block_size: i32,
        decrement: f64,
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            maxval,
            method,
            typ,
            block_size,
            decrement,
            ctx: Box::new(ctx),
        }
    }
}
impl<In> Eval<In, Result<Mat, Error>> for AdaptiveThreshold<In> {
    fn eval(&self, mat: In) -> Result<Mat, Error> {
        let error = Error::new("AdaptiveThreshold", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                opencv::imgproc::adaptive_threshold(
                    &mat,
                    &mut dst,
                    self.maxval,
                    self.method as i32,
                    self.typ as i32,
                    self.block_size,
                    self.decrement,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!(
                            "Can't apply Adaptive Threshold operation with maxval {:?}, method {:?}, type {:?}, block size {:?}, decrement {:?}",
                            self.maxval, self.method, self.typ, self.block_size, self.decrement,
                        ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
mod auto_threshold {
use opencv::{core::Mat, imgproc::ThresholdTypes};
use sal_core::error::Error;
use crate::Eval;
pub struct AutoThreshold<In> {
    factor: f64,
    maxval: f64,
    typ: ThresholdTypes,
    ctx: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
}
impl<In> AutoThreshold<In> {
    #[allow(unused)]
    pub fn new(
        factor: f64,
        maxval: f64,
        typ: ThresholdTypes,
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            factor,
            maxval,
            typ,
            ctx: Box::new(ctx),
        }
    }
}
impl<In> Eval<In, Result<Mat, Error>> for AutoThreshold<In> {
    fn eval(&self, mat: In) -> Result<Mat, Error> {
        let error = Error::new("AutoThreshold", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                let threshold = opencv::imgproc::threshold(
                    &mat,
                    &mut dst,
                    0.0,
                    self.maxval,
                    ThresholdTypes::THRESH_OTSU as i32,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't calculate Threshold with maxval {:?}, typ 'THRESH_OTSU'", self.maxval),
                        err.to_string(),
                    )
                })?;
                opencv::imgproc::threshold(
                    &mat,
                    &mut dst,
                    threshold * self.factor,
                    self.maxval,
                    self.typ as i32,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't apply Threshold operation with threshold {:?}, maxval {:?}, typ {:?}", threshold * self.factor, self.maxval, self.typ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
mod bitwise_and {
use opencv::core::{Mat, MatTraitConst};
use sal_core::error::Error;
use crate::domain::Eval;
pub struct BitwiseAnd<In> {
    ctx1: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
    ctx2: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
}
impl<In> BitwiseAnd<In> {
    #[allow(unused)]
    pub fn new(
        ctx1: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
        ctx2: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static
    ) -> Self {
        Self {
            ctx1: Box::new(ctx1),
            ctx2: Box::new(ctx2),
        }
    }
}
impl<In: Clone> Eval<In, Result<Mat, Error>> for BitwiseAnd<In> {
    fn eval(&self, frame: In) -> Result<Mat, Error> {
        let error = Error::new("BitwiseAnd", "eval");
        match (self.ctx1.eval(frame.clone()), self.ctx2.eval(frame)) {
            (Ok(mat1), Ok(mat2)) => {
                let mut dst = opencv::core::Mat::default();
                opencv::core::bitwise_and(&mat1, &mat2, &mut dst, &opencv::core::no_array())
                .map_err(|err| {
                    error.pass_with(
                        format!(
                            "Can't apply Bitwise And operation to src1: {}x{} and src2: {}x{}",
                            mat1.cols(), mat1.rows(), mat2.cols(), mat2.rows(),
                        ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            (Ok(_), Err(err)) => Err(error.pass(err)),
            (Err(err), Ok(_)) => Err(error.pass(err)),
            (Err(err), Err(_)) => Err(error.pass(err)),
        }
    }
}
}
mod create_mat {
use opencv::core::{Mat, MatTraitManual};
use sal_core::error::Error;
use crate::{algorithm::cv::MatType, Eval};
pub struct CreateMat {
    width: i32,
    height: i32,
    typ: MatType,
}
impl CreateMat {
    #[allow(unused)]
    pub fn new(width: i32, height: i32, typ: MatType) -> Self {
        Self { width, height, typ }
    }
    #[allow(unused)]
    pub fn gray8(width: i32, height: i32) -> Self {
        Self { width, height, typ: MatType::Cv8uc1 }
    }
    #[allow(unused)]
    pub fn color8(width: i32, height: i32) -> Self {
        Self { width, height, typ: MatType::Cv8uc3 }
    }
    #[allow(unused)]
    pub fn color8alpha(width: i32, height: i32, typ: MatType) -> Self {
        Self { width, height, typ: MatType::Cv8uc4 }
    }
}
impl Eval<(), Result<Mat, Error>> for CreateMat {
    fn eval(&self, _: ()) -> Result<Mat, Error> {
        unsafe { opencv::core::Mat::new_rows_cols(
            self.height,
            self.width,
            self.typ as i32,
        )}
        .map_err(|err| Error::new("CreateMat", "eval()").pass_with(
            format!("Can't create Mat with width {}, height {}, type {:?}", self.width, self.height, self.typ),
            err.to_string(),
        ))
    }
}
impl Eval<&[u8], Result<Mat, Error>> for CreateMat {
    fn eval(&self, val: &[u8]) -> Result<Mat, Error> {
        let error = Error::new("CreateMat", "eval(val: &[u8])");
        let mut mat = self.eval(()).map_err(|err| error.clone().pass(err.to_string()))?;
        if val.is_empty() {
            return Ok(mat);
        }
        let data_bytes = mat.data_bytes_mut().map_err(|err| error.clone().pass_with(
            "Failed to get mutable byte slice from Mat".to_string(),
            err.to_string()
        ))?;
        if data_bytes.len() != val.len() {
            return Err(error.pass_with(
                format!("Data size mismatch. Expected {} bytes, got {}", data_bytes.len(), val.len()),
                "".to_string()
            ));
        }
        data_bytes.copy_from_slice(val);
        Ok(mat)
    }
}
impl Eval<Vec<u8>, Result<Mat, Error>> for CreateMat {
    fn eval(&self, val: Vec<u8>) -> Result<Mat, Error> {
        self.eval(val.as_slice())
        .map_err(|err| Error::new("CreateMat", "eval(val: Vec<u8>)").pass(err.to_string()))
    }
}
}
mod gaussian_blur {
use opencv::{
    core::{BorderTypes, Mat, Size2i}, imgproc,
};
use sal_core::error::Error;
use crate::Eval;
pub struct GaussianBlur<In> {
    kernel: Vec<i32>,
    sigma: Vec<f64>,
    border: BorderTypes,
    ctx: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
}
impl<In> GaussianBlur<In> {
    #[allow(unused)]
    pub fn new(kernel: &[i32; 2], ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static) -> Self {
        Self {
            kernel: kernel.into(),
            sigma: vec![0.0, 0.0],
            border: BorderTypes::BORDER_REFLECT_101,
            ctx: Box::new(ctx),
        }
    }
    #[allow(unused)]
    pub fn with_sigma(mut self, xy: &[f64; 2]) -> Self {
        self.sigma = xy.into();
        self
    }
    #[allow(unused)]
    pub fn with_border(mut self, border: BorderTypes) -> Self {
        self.border = border;
        self
    }
}
impl<In> Eval<In, Result<Mat, Error>> for GaussianBlur<In> {
    fn eval(&self, mat: In) -> Result<Mat, Error> {
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                imgproc::gaussian_blur(
                    &mat,
                    &mut dst,
                    Size2i::new(self.kernel[0], self.kernel[1]),
                    self.sigma[0],
                    self.sigma[1],
                    self.border as i32,
                )
                .map_err(|err| {
                    Error::new("GaussianBlur", "eval").pass_with(
                        format!("Can't apply Gausian Blur, kernel {:?}, sigma {:?}, border {:?}", self.kernel, self.sigma, self.border),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(Error::new("GaussianBlur", "eval").pass(err)),
        }
    }
}}
mod laplacian {
use opencv::core::{BorderTypes, Mat};
use sal_core::error::Error;
use crate::Eval;
pub struct Laplacian {
    out_depth: i32,
    kernel: i32,
    scale: f64,
    delta: f64,
    border: BorderTypes,
    ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
}
impl Laplacian {
    #[allow(unused)]
    pub fn new(kernel: i32, ctx: impl Eval<Mat, Result<Mat, Error>> + Send + Sync + 'static) -> Self {
        Self {
            out_depth: opencv::core::CV_8UC1,
            kernel,
            scale: 1.0,
            delta: 0.0,
            border: BorderTypes::BORDER_REFLECT_101,
            ctx: Box::new(ctx),
        }
    }
    #[allow(unused)]
    pub fn with_depth(mut self, depth: i32) -> Self {
        self.out_depth = depth;
        self
    }
    #[allow(unused)]
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
    #[allow(unused)]
    pub fn with_delta(mut self, delta: f64) -> Self {
        self.delta = delta;
        self
    }
    #[allow(unused)]
    pub fn with_border(mut self, border: BorderTypes) -> Self {
        self.border = border;
        self
    }
    fn depth_name(depth: i32) -> String {
        match depth {
            opencv::core::CV_16FC1 => String::from("CV_16FC1"),
            opencv::core::CV_16FC2 => String::from("CV_16FC2"),
            opencv::core::CV_16FC3 => String::from("CV_16FC3"),
            opencv::core::CV_16FC4 => String::from("CV_16FC4"),
            opencv::core::CV_16SC1 => String::from("CV_16SC1"),
            opencv::core::CV_16SC2 => String::from("CV_16SC2"),
            opencv::core::CV_16SC3 => String::from("CV_16SC3"),
            opencv::core::CV_16SC4 => String::from("CV_16SC4"),
            opencv::core::CV_16UC1 => String::from("CV_16UC1"),
            opencv::core::CV_16UC2 => String::from("CV_16UC2"),
            opencv::core::CV_16UC3 => String::from("CV_16UC3"),
            opencv::core::CV_16UC4 => String::from("CV_16UC4"),
            opencv::core::CV_32FC1 => String::from("CV_32FC1"),
            opencv::core::CV_32FC2 => String::from("CV_32FC2"),
            opencv::core::CV_32FC3 => String::from("CV_32FC3"),
            opencv::core::CV_32FC4 => String::from("CV_32FC4"),
            opencv::core::CV_32SC1 => String::from("CV_32SC1"),
            opencv::core::CV_32SC2 => String::from("CV_32SC2"),
            opencv::core::CV_32SC3 => String::from("CV_32SC3"),
            opencv::core::CV_32SC4 => String::from("CV_32SC4"),
            opencv::core::CV_64FC1 => String::from("CV_64FC1"),
            opencv::core::CV_64FC2 => String::from("CV_64FC2"),
            opencv::core::CV_64FC3 => String::from("CV_64FC3"),
            opencv::core::CV_64FC4 => String::from("CV_64FC4"),
            opencv::core::CV_8SC1 => String::from("CV_8SC1"),
            opencv::core::CV_8SC2 => String::from("CV_8SC2"),
            opencv::core::CV_8SC3 => String::from("CV_8SC3"),
            opencv::core::CV_8SC4 => String::from("CV_8SC4"),
            opencv::core::CV_8UC1 => String::from("CV_8UC1"),
            opencv::core::CV_8UC2 => String::from("CV_8UC2"),
            opencv::core::CV_8UC3 => String::from("CV_8UC3"),
            opencv::core::CV_8UC4 => String::from("CV_8UC4"),
            _ => format!("{depth} - Unknown color depth"),
        }
    }
}
impl Eval<Mat, Result<Mat, Error>> for Laplacian {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                opencv::imgproc::laplacian(
                    &mat,
                    &mut dst,
                    self.out_depth,
                    self.kernel,
                    self.scale,
                    self.delta,
                    self.border as i32,
                )
                .map_err(|err| {
                    Error::new("Laplacian", "eval")
                        .pass_with(
                            format!(
                                "Can't apply Laplacian with depth {}, kernel {:?}, scale {:?}, delta {:?}, border {:?}",
                                Self::depth_name(self.out_depth), self.kernel, self.scale, self.delta, self.border),
                            err.to_string(),
                        )
                })?;
                Ok(dst)
            }
            Err(err) => Err(Error::new("Laplacian", "eval").pass(err)),
        }
    }
}
}
mod mat_type {
#[derive(Debug ,Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum MatType {
    Cv8uc1 = opencv::core::CV_8UC1 as isize,
    Cv8uc2 = opencv::core::CV_8UC2 as isize,
    Cv8uc3 = opencv::core::CV_8UC3 as isize,
    Cv8uc4 = opencv::core::CV_8UC4 as isize,
    Cv8sc1 = opencv::core::CV_8SC1 as isize,
    Cv8sc2 = opencv::core::CV_8SC2 as isize,
    Cv8sc3 = opencv::core::CV_8SC3 as isize,
    Cv8sc4 = opencv::core::CV_8SC4 as isize,
    Cv16uc1 = opencv::core::CV_16UC1 as isize,
    Cv16uc2 = opencv::core::CV_16UC2 as isize,
    Cv16uc3 = opencv::core::CV_16UC3 as isize,
    Cv16uc4 = opencv::core::CV_16UC4 as isize,
    Cv16sc1 = opencv::core::CV_16SC1 as isize,
    Cv16sc2 = opencv::core::CV_16SC2 as isize,
    Cv16sc3 = opencv::core::CV_16SC3 as isize,
    Cv16sc4 = opencv::core::CV_16SC4 as isize,
    Cv32sc1 = opencv::core::CV_32SC1 as isize,
    Cv32sc2 = opencv::core::CV_32SC2 as isize,
    Cv32sc3 = opencv::core::CV_32SC3 as isize,
    Cv32sc4 = opencv::core::CV_32SC4 as isize,
    Cv32fc1 = opencv::core::CV_32FC1 as isize,
    Cv32fc2 = opencv::core::CV_32FC2 as isize,
    Cv32fc3 = opencv::core::CV_32FC3 as isize,
    Cv32fc4 = opencv::core::CV_32FC4 as isize,
    Cv64fc1 = opencv::core::CV_64FC1 as isize,
    Cv64fc2 = opencv::core::CV_64FC2 as isize,
    Cv64fc3 = opencv::core::CV_64FC3 as isize,
    Cv64fc4 = opencv::core::CV_64FC4 as isize,
}
impl MatType {
    #[allow(unused)]
    pub fn depth(&self) -> u8 {
        match self {
            Self::Cv8uc1 => 8,
            Self::Cv8uc2 => 8,
            Self::Cv8uc3 => 8,
            Self::Cv8uc4 => 8,
            Self::Cv8sc1 => 8,
            Self::Cv8sc2 => 8,
            Self::Cv8sc3 => 8,
            Self::Cv8sc4 => 8,
            Self::Cv16uc1 => 16,
            Self::Cv16uc2 => 16,
            Self::Cv16uc3 => 16,
            Self::Cv16uc4 => 16,
            Self::Cv16sc1 => 16,
            Self::Cv16sc2 => 16,
            Self::Cv16sc3 => 16,
            Self::Cv16sc4 => 16,
            Self::Cv32sc1 => 32,
            Self::Cv32sc2 => 32,
            Self::Cv32sc3 => 32,
            Self::Cv32sc4 => 32,
            Self::Cv32fc1 => 32,
            Self::Cv32fc2 => 32,
            Self::Cv32fc3 => 32,
            Self::Cv32fc4 => 32,
            Self::Cv64fc1 => 64,
            Self::Cv64fc2 => 64,
            Self::Cv64fc3 => 64,
            Self::Cv64fc4 => 64,
        }
    }
    #[allow(unused)]
    pub fn channels(&self) -> u8 {
        match self {
            Self::Cv8uc1 => 1,
            Self::Cv8uc2 => 2,
            Self::Cv8uc3 => 3,
            Self::Cv8uc4 => 4,
            Self::Cv8sc1 => 1,
            Self::Cv8sc2 => 2,
            Self::Cv8sc3 => 3,
            Self::Cv8sc4 => 4,
            Self::Cv16uc1 => 1,
            Self::Cv16uc2 => 2,
            Self::Cv16uc3 => 3,
            Self::Cv16uc4 => 4,
            Self::Cv16sc1 => 1,
            Self::Cv16sc2 => 2,
            Self::Cv16sc3 => 3,
            Self::Cv16sc4 => 4,
            Self::Cv32sc1 => 1,
            Self::Cv32sc2 => 2,
            Self::Cv32sc3 => 3,
            Self::Cv32sc4 => 4,
            Self::Cv32fc1 => 1,
            Self::Cv32fc2 => 2,
            Self::Cv32fc3 => 3,
            Self::Cv32fc4 => 4,
            Self::Cv64fc1 => 1,
            Self::Cv64fc2 => 2,
            Self::Cv64fc3 => 3,
            Self::Cv64fc4 => 4,
        }
    }
}}
mod morphology {
use opencv::{
    core::{BorderTypes, Mat, Point2i, Scalar}, imgproc::{self, MorphTypes},
};
use sal_core::error::Error;
use crate::{algorithm::cv::StructuringElement, Eval};
pub struct Morphology<In> {
    operation: MorphTypes,
    kernel: Vec<i32>,
    iterations: i32,
    border: BorderTypes,
    border_val: Option<Scalar>,
    structuring_element: Option<Box<dyn Eval<(), Result<Mat, Error>> + Send + Sync>>,
    ctx: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
}
impl<In> Morphology<In> {
    #[allow(unused)]
    pub fn new(
        operation: MorphTypes,
        kernel: &[i32; 2],
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            operation,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    #[allow(unused)]
    pub fn erode(
        kernel: &[i32; 2],
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            operation: MorphTypes::MORPH_ERODE,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    #[allow(unused)]
    pub fn open(
        kernel: &[i32; 2],
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            operation: MorphTypes::MORPH_OPEN,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    #[allow(unused)]
    pub fn dilate(
        kernel: &[i32; 2],
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            operation: MorphTypes::MORPH_DILATE,
            kernel: kernel.into(),
            iterations: 1,
            border: BorderTypes::BORDER_CONSTANT,
            border_val: None,
            structuring_element: None,
            ctx: Box::new(ctx),
        }
    }
    #[allow(unused)]
    pub fn with_iterations(mut self, n: usize) -> Self {
        self.iterations = n as i32;
        self
    }
    #[allow(unused)]
    pub fn with_border(mut self, border: BorderTypes) -> Self {
        self.border = border;
        self
    }
    #[allow(unused)]
    pub fn with_kernel(mut self, ctx: impl Eval<(), Result<Mat, Error>> + Send + Sync + 'static) -> Self {
        self.structuring_element = Some(Box::new(ctx));
        self
    }
}
impl<In> Eval<In, Result<Mat, Error>> for Morphology<In> {
    fn eval(&self, mat: In) -> Result<Mat, Error> {
        let error = Error::new("Morphology", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                let kernel = match &self.structuring_element {
                    Some(ctx) => ctx.eval(()).map_err(|err| error.pass(err))?,
                    None => StructuringElement::new(&[self.kernel[0], self.kernel[1]]).eval(()).map_err(|err| error.pass(err))?,
                };
                imgproc::morphology_ex(
                    &mat,
                    &mut dst,
                    self.operation as i32,
                    &kernel,
                    Point2i::new(-1, -1),
                    self.iterations,
                    self.border as i32,
                    self.border_val.unwrap_or(
                        opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass(err.to_string()))?,
                    ),
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't apply Morphology operation {:?}, kernel {:?}, border {:?}, border value {:?}", self.operation, self.kernel, self.border, self.border_val),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}}
mod structure_element {
use opencv::{
    core, core::Mat, imgproc::MorphShapes,
};
use sal_core::error::Error;
use crate::Eval;
pub struct StructuringElement {
    kernel: Vec<i32>,
    shape: MorphShapes,
}
impl StructuringElement {
    #[allow(unused)]
    pub fn new(kernel: &[i32; 2]) -> Self {
        Self {
            kernel: kernel.into(),
            shape: MorphShapes::MORPH_ELLIPSE,
        }
    }
    #[allow(unused)]
    pub fn with_shape(mut self, shape: MorphShapes) -> Self {
        self.shape = shape;
        self
    }
}
impl Eval<(), Result<Mat, Error>> for StructuringElement {
    fn eval(&self, _: ()) -> Result<Mat, Error> {
        opencv::imgproc::get_structuring_element(
            self.shape as i32,
            core::Size2i::new(self.kernel[0], self.kernel[1]),
            core::Point2i::new(-1, -1),
        )
        .map_err(|err| {
            Error::new("StructuringElement", "eval")
                .pass_with(format!("Can't create Structuring Element, kernel {:?}", self.kernel), err.to_string())
        })
    }
}}
mod threshold {
use opencv::{core::Mat, imgproc::ThresholdTypes};
use sal_core::error::Error;
use crate::Eval;
pub struct Threshold<In> {
    threshold: f64,
    maxval: f64,
    typ: ThresholdTypes,
    ctx: Box<dyn Eval<In, Result<Mat, Error>> + Send + Sync>,
}
impl<In> Threshold<In> {
    #[allow(unused)]
    pub fn new(
        threshold: f64,
        maxval: f64,
        typ: ThresholdTypes,
        ctx: impl Eval<In, Result<Mat, Error>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            threshold,
            maxval,
            typ,
            ctx: Box::new(ctx),
        }
    }
}
impl<In> Eval<In, Result<Mat, Error>> for Threshold<In> {
    fn eval(&self, mat: In) -> Result<Mat, Error> {
        let error = Error::new("Threshold", "eval");
        match self.ctx.eval(mat) {
            Ok(mat) => {
                let mut dst = Mat::default();
                opencv::imgproc::threshold(
                    &mat,
                    &mut dst,
                    self.threshold,
                    self.maxval,
                    self.typ as i32,
                )
                .map_err(|err| {
                    error.pass_with(
                        format!("Can't apply Threshold operation with threshold {:?}, maxval {:?}, typ {:?}", self.threshold, self.maxval, self.typ),
                        err.to_string(),
                    )
                })?;
                Ok(dst)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
pub use adaptive_threshold::*;
pub use auto_threshold::*;
pub use bitwise_and::*;
pub use create_mat::*;
pub use gaussian_blur::*;
pub use laplacian::*;
pub use mat_type::*;
pub use morphology::*;
pub use structure_element::*;
pub use threshold::*;}
mod detecting_contours {
mod detecting_contours_ctx {
use photon_rs::PhotonImage;
pub struct DetectingContoursCtx {
    pub result: PhotonImage,
}}
mod detecting_contours {
use photon_rs::PhotonImage;
use sal_core::dbg::Dbg;
use crate::domain::Eval;
use super::detecting_contours_ctx::DetectingContoursCtx;
use photon_rs::monochrome::grayscale;
use photon_rs::monochrome::threshold;
use photon_rs::conv::{
    noise_reduction,
    gaussian_blur,
    sobel_vertical,
};
pub struct DetectingContours {
    dbg: Dbg,
    input_frame: PhotonImage,
    result: Option<DetectingContoursCtx>
}
impl DetectingContours {
    pub fn new(input_frame: PhotonImage) -> Self {
        Self {
            dbg: Dbg::own("DetectingContours"),
            input_frame,
            result: None,
        }
    }
    fn get_contours(&self) -> PhotonImage {
        let mut result = self.input_frame.clone();
        grayscale(&mut result);
        gaussian_blur(&mut result, 3_i32);
        sobel_vertical(&mut result);
        noise_reduction(&mut result);
        threshold(&mut result, 7_u32);
        result
    }
}
impl Eval<(), DetectingContoursCtx> for DetectingContours {
    fn eval(&self, _: ()) -> DetectingContoursCtx {
        DetectingContoursCtx {
            result: self.get_contours()
        }
    }
}}
pub use detecting_contours_ctx::*;
pub use detecting_contours::*;}
mod fast_scan {
mod fast_contours {
mod fast_contours_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FastContoursConf {
    pub otsu_tune: f64,
}
impl FastContoursConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "DetectingContoursConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let otsu_tune = conf.get("otsu-tune").expect(&format!("{dbg}.new | 'otsu-tune' - not found or wrong configuration"));
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        Self {
            otsu_tune,
        }
    }
}
impl Default for FastContoursConf {
    fn default() -> Self {
        Self {
            otsu_tune: 0.4,
        }
    }
}
}
mod fast_contours_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct FastContoursCtx {
    pub result: Image,
}
impl Default for FastContoursCtx {
    fn default() -> Self {
        Self {
            result: Image::default()
         }
    }
}
}
mod fast_contours {
use std::time::Instant;
use opencv::{core::Mat, imgproc::ThresholdTypes};
use sal_core::error::Error;
use crate::algorithm::{
    cv, ContextWrite, ContextRead,
    FastContoursCtx, FastContoursConf,
    EvalResult, ResultCtx,
};
use crate::{Eval, domain::Image};
pub struct FastContours {
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    proc: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
    debug: bool,
}
impl FastContours {
    pub fn new(conf: FastContoursConf, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        let kernel = 13;
        Self {
            ctx: Box::new(ctx),
            proc: Box::new(
                cv::Morphology::dilate(
                    &[7, 7],
                    cv::GaussianBlur::new(
                        &[15, 15],
                        cv::Morphology::open(
                            &[5, 5],
                            cv::GaussianBlur::new(
                                &[7, 7],
                                cv::AutoThreshold::new(
                                    conf.otsu_tune,
                                    255.0,
                                    ThresholdTypes::THRESH_BINARY,
                                    cv::GaussianBlur::new(
                                        &[kernel, kernel],
                                        cv::Laplacian::new(
                                            5,
                                            cv::GaussianBlur::new(
                                                &[kernel, kernel],
                                                PassCvMat::new(),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    )
                ),
            ),
            debug,
        }
    }
}
impl Eval<Image, EvalResult> for FastContours {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FastContours", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mat = self.proc.eval(frame.mat.clone())
                    .map_err(|err| error.pass(err))?;
                let frame = Image {mat, meta: frame.meta};
                let ctx = if self.debug {
                    ctx.write(FastContoursCtx { result: frame.clone() }).map_err(|err| error.pass(err))?
                } else {
                    ctx
                };
                let result = ResultCtx { val: frame };
                log::trace!("FastContours.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
struct PassCvMat {}
impl PassCvMat {
    fn new() -> Self {
        Self {  }
    }
}
impl Eval<Mat, Result<Mat, Error>> for PassCvMat {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        Ok(mat)
    }
}
}
pub use fast_contours_conf::*;
pub use fast_contours_ctx::*;
pub use fast_contours::*;}
mod fast_edges {
mod fast_edges_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FastEdgesConf {
    pub otsu_tune: Option<f64>,
    pub threshold: Option<u8>,
    pub smooth: Option<f64>,
}
impl FastEdgesConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "FastEdgesConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let otsu_tune = conf.get("otsu-tune");
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        let threshold = conf.get("threshold").map(|val: u64| val as u8);
        log::trace!("{dbg}.new | threshold: {:#?}", threshold);
        let otsu_tune = match (otsu_tune, threshold) {
            (None, None) => Some(1.0),
            (None, Some(_)) => None,
            (Some(otsu_tune), None) => Some(otsu_tune),
            (Some(_), Some(_)) => panic!("{dbg}.new |  'otsu-tune' and 'threshold' - both specified, use on of them, otsu auto threshol with 'otsu-tune' or static 'threshold'"),
        };
        let smooth = conf.get("smooth");
        let smooth = match smooth {
            Some(smooth) => if smooth <= 0.0 {
                None
            } else {
                Some(smooth)
            }
            None => None,
        };
        log::trace!("{dbg}.new | smooth: {:#?}", smooth);
        Self {
            otsu_tune,
            threshold,
            smooth,
        }
    }
}
impl Default for FastEdgesConf {
    fn default() -> Self {
        Self {
            otsu_tune: Some(1.0),
            threshold: None,
            smooth: None,
        }
    }
}
}
mod fast_edges_ctx {
use crate::{algorithm::Edges};
#[derive(Debug, Clone, PartialEq)]
pub struct FastEdgesCtx {
    pub edges: Edges<usize>,
}
impl Default for FastEdgesCtx {
    fn default() -> Self {
        Self {
            edges: Edges::default()
         }
    }
}
}
mod fast_edges {
use std::time::Instant;
use opencv::{core::{Mat, MatTraitConst, MatTraitConstManual}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{
        ContextRead, ContextWrite, EvalResult, Edges, ResultCtx, FastEdgesCtx,
    },
    domain::{Dot, Eval, Filter, FilterEmpty, FilterSmooth2, Image}
};
pub struct FastEdges {
    otsu_tune: Option<f64>,
    threshold: Option<u8>,
    smooth: Option<f64>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
impl FastEdges {
    pub fn new(otsu_tune: Option<f64>, threshold: Option<u8>, smooth: Option<f64>, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            otsu_tune,
            threshold,
            smooth,
            ctx: Box::new(ctx),
        }
    }
}
impl Eval<Image, EvalResult> for FastEdges {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FastEdges", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let threshold = match (self.otsu_tune, self.threshold) {
                    (None, None) => imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap().round() as u8,
                    (None, Some(threshold)) => threshold,
                    (Some(otsu_tune), None) => (imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * otsu_tune).round() as u8,
                    (Some(otsu_tune), Some(_)) => (imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * otsu_tune).round() as u8,
                };
                log::trace!("FastEdges.eval | threshold: {threshold}");
                let rows = frame.mat.rows();
                let cols = frame.mat.cols();
                let mut upper_edge = Vec::with_capacity(cols as usize);
                let mut lower_edge = Vec::with_capacity(cols as usize);
                let (mut filter_smooth_upper, mut filter_smooth_lower): (Box<dyn Filter<Item = i32>>, Box<dyn Filter<Item = i32>>) = match self.smooth {
                    Some(smooth) => (Box::new(FilterSmooth2::<i32>::new(None, smooth)), Box::new(FilterSmooth2::<i32>::new(None, smooth))),
                    None => (Box::new(FilterEmpty::new()), Box::new(FilterEmpty::new())),
                };
                let mut upper;
                let mut lower;
                let mat = frame.mat.data_bytes().unwrap();
                for x in 0..cols {
                    upper = false;
                    lower = false;
                    for y in 0..rows {
                        if !upper {
                            match mat.get((y * cols + x) as usize) {
                                Some(pixel_value) => {
                                    if pixel_value > &threshold {
                                        if let Some(y) = filter_smooth_upper.add(y) {
                                            upper_edge.push(Dot {x: x as usize, y: y as usize});
                                            upper = true;
                                        }
                                    }
                                }
                                None => {
                                    return Err(error.err("Input image format error, index out of image range"));
                                }
                            }
                        }
                        let y = rows - y -1;
                        if !lower {
                            match mat.get((y * cols + x) as usize) {
                                Some(pixel_value) => {
                                    if pixel_value > &threshold {
                                        if let Some(y) = filter_smooth_lower.add(y) {
                                            lower_edge.push(Dot {x: x as usize, y: y as usize});
                                            lower = true;
                                        }
                                    }
                                }
                                None => {
                                    return Err(error.err("Input image format error, index out of image range"));
                                }
                            }
                        }
                        if upper && lower {
                            break;
                        }
                    }
                }
                let result = FastEdgesCtx {
                    edges: Edges::new(upper_edge, lower_edge),
                };
                log::trace!("FastEdges.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
pub use fast_edges_conf::*;
pub use fast_edges_ctx::*;
pub use fast_edges::*;
}
mod fast_union {
mod fast_union_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct FastUnionCtx {
    pub frame: Image,
}
impl Default for FastUnionCtx {
    fn default() -> Self {
        Self {
            frame: Image::default()
         }
    }
}
}
mod fast_union {
use std::{sync::Arc, time::Instant};
use opencv::core::MatTraitConst;
use sal_core::error::Error;
use sal_sync::{services::future::Future, thread_pool::Scheduler};
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, FastUnionCtx, ResultCtx}, conf::UnionConf, domain::{Eval, Image, RwLock}
};
pub struct FastUnion {
    conf: UnionConf,
    scheduler: Scheduler,
    ctx1: Arc<RwLock<Box<dyn Eval<Image, EvalResult> + Send + Sync>>>,
    ctx2: Arc<RwLock<Box<dyn Eval<Image, EvalResult> + Send + Sync>>>,
    debug: bool,
}
impl FastUnion {
    pub fn new(
        conf: UnionConf,
        scheduler: Scheduler,
        ctx1: impl Eval<Image, EvalResult> + Send + Sync + 'static,
        ctx2: impl Eval<Image, EvalResult> + Send + Sync + 'static,
        debug: bool,
    ) -> Self {
        Self {
            conf,
            scheduler,
            ctx1: Arc::new(RwLock::new(Box::new(ctx1))),
            ctx2: Arc::new(RwLock::new(Box::new(ctx2))),
            debug,
        }
    }
}
impl Eval<Image, EvalResult> for FastUnion {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FastUnion", "eval");
        let (ctx1, sink) = Future::new();
        let ctx1_eval = self.ctx1.clone();
        let frame1 = frame.clone();
        let meta = frame.meta;
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
                log::trace!("FastUnion.eval | src1: {}x{}", src1_mat.cols(), src1_mat.rows());
                let src2: &ResultCtx<Image> = ctx2.read();
                let src2_mat = &src2.val.mat;
                log::trace!("FastUnion.eval | src1: {}x{}", src2_mat.cols(), src2_mat.rows());
                let mut dst = opencv::core::Mat::default();
                match (self.conf.add_weighted, self.conf.bitwise_and) {
                    (None, Some(_)) => opencv::core::bitwise_and(src1_mat, src2_mat, &mut dst, &opencv::core::no_array())
                        .map_err(|err| error.pass(err.to_string()))?,
                    (Some(conf), None) => opencv::core::add_weighted_def(src1_mat, conf.weight1, src2_mat, conf.weight2, conf.gamma, &mut dst)
                        .map_err(|err| error.pass(err.to_string()))?,
                    _ => Err(error.err(format!("Both: 'add-weighted' and `bitwise-and` - are specified, please use one of")))?,
                }
                let frame = Image::from(dst, meta);
                let ctx = if self.debug {
                    let union = FastUnionCtx { frame: frame.clone() };
                    ctx1.write(union).map_err(|err| error.pass(err))?
                } else {
                    ctx1
                };
                log::trace!("FastUnion.eval | Elapsed: {:?}", t.elapsed());
                ctx.write( ResultCtx { val: frame } )
            }
            (Ok(_), Err(err)) => Err(error.pass(err)),
            (Err(err), Ok(_)) => Err(error.pass(err)),
            (Err(err), Err(_)) => Err(error.pass(err)),
        }
    }
}
}
pub use fast_union_ctx::*;
pub use fast_union::*;
}
mod fast_scan_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::{FastContoursConf, FastEdgesConf, RopeDimensionsConf, TemporalFilterConf, Threshold}, conf::UnionConf};
#[derive(Debug, PartialEq, Clone)]
pub struct FastScanConf {
    pub fast_contours: FastContoursConf,
    pub temporal_filter: TemporalFilterConf,
    pub fast_edges: FastEdgesConf,
    pub union: UnionConf,
    pub rope_dimensions: RopeDimensionsConf,
    pub distortion_threshold: Threshold,
}
impl FastScanConf {
    #[allow(unused)]
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "FastScanConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let fast_contours = conf.get("fast-contours").expect(&format!("{dbg}.new | 'fast-contours' - not found or wrong configuration"));
        let fast_contours = FastContoursConf::new(&name, fast_contours);
        log::trace!("{dbg}.new | fast-contours: {:#?}", fast_contours);
        let temporal_filter = conf.get("temporal-filter").expect(&format!("{dbg}.new | 'temporal-filter' - not found or wrong configuration"));
        let temporal_filter = TemporalFilterConf::new(&name, temporal_filter);
        log::trace!("{dbg}.new | temporal-filter: {:#?}", temporal_filter);
        let fast_edges = conf.get("fast-edges").expect(&format!("{dbg}.new | 'fast-edges' - not found or wrong configuration"));
        let fast_edges = FastEdgesConf::new(&name, fast_edges);
        log::trace!("{dbg}.new | fast-edges: {:#?}", fast_edges);
        let union = conf.get("union").expect(&format!("{dbg}.new | 'union' - not found or wrong configuration"));
        let union = UnionConf::new(&name, union);
        log::trace!("{dbg}.new | union: {:#?}", union);
        let rope_dimensions = conf.get("rope-dimensions").expect(&format!("{dbg}.new | 'rope-dimensions' - not found or wrong configuration"));
        let rope_dimensions = RopeDimensionsConf::new(&name, rope_dimensions);
        log::trace!("{dbg}.new | rope-dimensions: {:#?}", rope_dimensions);
        let distortion_threshold = conf.get("distortion-threshold").expect(&format!("{dbg}.new | 'distortion-threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | distortion-threshold: {:?}", distortion_threshold);
        Self {
            fast_contours,
            temporal_filter,
            fast_edges,
            union,
            rope_dimensions,
            distortion_threshold: Threshold(distortion_threshold),
        }
    }
}
impl Default for FastScanConf {
    fn default() -> Self {
        Self {
            fast_contours: Default::default(),
            temporal_filter: Default::default(),
            fast_edges: Default::default(),
            union: UnionConf::default(),
            rope_dimensions: Default::default(),
            distortion_threshold: Default::default(),
        }
    }
}}
mod fast_scan_ctx {
use crate::algorithm::{
    FastEdgesCtx, FastContoursCtx, FastUnionCtx, RopeDimensionsCtx, TemporalFilterCtx, RopeDistortionsCtx,
};
#[derive(Debug, Clone)]
pub struct FastScanCtx {
    pub fast_contours: FastContoursCtx,
    pub temporal_filter: TemporalFilterCtx<FastScanCtx>,
    pub union: FastUnionCtx,
    pub edges: FastEdgesCtx,
    pub rope_dimensions: RopeDimensionsCtx<FastScanCtx>,
    pub distortions: RopeDistortionsCtx<FastScanCtx>,
}
impl Default for FastScanCtx {
    fn default() -> Self {
        Self {
            fast_contours: FastContoursCtx::default(),
            temporal_filter: TemporalFilterCtx::default(),
            union: FastUnionCtx::default(),
            edges: FastEdgesCtx::default(),
            rope_dimensions: RopeDimensionsCtx::default(),
            distortions: RopeDistortionsCtx::default(),
        }
    }
}
}
mod fast_scan {
use std::{sync::Arc, time::Instant};
use sal_core::error::Error;
use sal_sync::{sync::Owner, thread_pool::Scheduler};
use crate::{
    algorithm::{
        Context, ContextRead, EvalResult, FastContours, FastEdges, FastScanConf, FastScanCtx, FastUnion, ResultCtx, TemporalFilter, Mad, RopeDistortions,
    },
    domain::{Eval, Image},
};
pub struct FastScan {
    pass_ctx1: Arc<Owner<Context>>,
    pass_ctx2: Arc<Owner<Context>>,
    ctx_gray: Box<dyn Eval<Image, EvalResult>>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
impl FastScan {
    #[allow(unused)]
    pub fn new(
        conf: FastScanConf,
        scheduler: Scheduler,
        ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static,
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
}
pub use fast_contours::*;
pub use fast_edges::*;
pub use fast_union::*;
pub use fast_scan_conf::*;
pub use fast_scan_ctx::*;
pub use fast_scan::*;
}
mod fine_scan {
mod fine_contours {
mod fine_contours_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FineContoursConf {
    pub otsu_tune: f64,
    pub merge_distance: f64
}
impl FineContoursConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "DetectingContoursConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let otsu_tune = conf.get("otsu-tune").expect(&format!("{dbg}.new | 'otsu-tune' - not found or wrong configuration"));
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        let merge_distance = conf.get("merge-distance").expect(&format!("{dbg}.new | 'merge-distance' - not found or wrong configuration"));
        log::trace!("{dbg}.new | merge-distance: {:#?}", merge_distance);
        Self {
            otsu_tune,
            merge_distance,
        }
    }
}
impl Default for FineContoursConf {
    fn default() -> Self {
        Self {
            otsu_tune: 0.4,
            merge_distance: 24.0,
        }
    }
}
}
mod fine_contours_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct FineContoursCtx {
    pub result: Image,
}
impl Default for FineContoursCtx {
    fn default() -> Self {
        Self {
            result: Image::default(),
         }
    }
}
}
mod fine_contours {
use std::collections::VecDeque;
use std::time::Instant;
use opencv::{
    core, imgproc,
    core::{Mat, MatTraitConst, Point, Point2i},
    imgproc::{LineTypes ,ThresholdTypes},
};
use sal_core::error::Error;
use crate::{algorithm::{
    cv, ContextRead, ContextWrite, EvalResult, FineContoursConf, FineContoursCtx, FineConvexCtx, ResultCtx,
}};
use crate::{Eval, domain::Image};
pub struct FineContours {
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    thresh_ctx: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
    conf: FineContoursConf,
    debug: bool,
}
impl FineContours {
    #[allow(unused)]
    pub fn new(conf: FineContoursConf, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            ctx: Box::new(ctx),
            thresh_ctx: Box::new(
                cv::AutoThreshold::new(
                    conf.otsu_tune,
                    255.0,
                    ThresholdTypes::THRESH_BINARY,
                    cv::Morphology::open(
                        &[5, 5],
                        cv::GaussianBlur::new(
                            &[13, 13],
                            cv::Laplacian::new(
                                5,
                                cv::GaussianBlur::new(
                                    &[11, 11],
                                    PassCvMat::new(),
                                ),
                            ),
                        ),
                    ),
                )
            ),
            conf,
            debug,
        }
    }
    fn inc(i: usize, len: usize) -> usize {
        if i >= (len - 1) {
            0
        } else {
            i + 1
        }
    }
    fn dec(i: usize, len: usize) -> usize {
        if i == 0 {
            len - 1
        } else {
            i - 1
        }
    }
    fn distance(pt1: &Point, pt2: &Point) -> f32 {
        ((pt2.x - pt1.x).pow(2) as f32 + (pt2.y - pt1.y).pow(2)  as f32).sqrt()
    }
    fn merge(contour1: &core::Vector<Point>, contour2: &core::Vector<Point>, threshold: f32) -> Result<core::Vector<Point>, Error> {
        let error = Error::new("FineContours", "merge");
        if contour1.len() < 4 || contour2.len() < 4 {
            log::warn!("FineContours.merge | c1[{}], c2[{}]", contour1.len(), contour2.len());
            return Err(error.err("c1 and c2 can't be length < 3"))
        }
        let mut found = false;
        let mut min = (0, 0, threshold);
        for (i1, pt1) in contour1.iter().enumerate() {
            for (i2, pt2) in contour2.iter().enumerate() {
                let distance = Self::distance(&pt1, &pt2);
                if distance <= min.2 {
                    min = (i1, i2, distance);
                    found = true;
                }
            }
        }
        if !found {
            return Err(error.err("Can't merge"));
        }
        let mut c1_remove = std::collections::HashSet::new();
        let mut c2_remove = std::collections::HashSet::new();
        let mut i1 = min.0;
        let mut i2 = min.1;
        let mut count = [contour1.len(), contour2.len()].iter().min().map(|m| *m).unwrap_or(contour1.len()) / 4;
        while count > 0 {
            let pt1 = contour1.get(i1).unwrap();
            let pt2 = contour2.get(i2).unwrap();
            if Self::distance(&pt1, &pt2) <= threshold {
                c1_remove.insert(i1);
                c2_remove.insert(i2);
                i1 = Self::dec(i1, contour1.len());
                i2 = Self::inc(i2, contour2.len());
                count -= 1;
            } else {
                break;
            }
        }
        let c1_end = Self::dec(i1, contour1.len());
        let c2_start = Self::inc(i2, contour2.len());
        let mut i1 = min.0;
        let mut i2 = min.1;
        let mut count = [contour1.len(), contour2.len()].iter().min().map(|m| *m).unwrap_or(contour1.len()) / 4;
        while count > 0 {
            let pt1 = contour1.get(i1).unwrap();
            let pt2 = contour2.get(i2).unwrap();
            if Self::distance(&pt1, &pt2) <= threshold {
                c1_remove.insert(i1);
                c2_remove.insert(i2);
                i1 = Self::inc(i1, contour1.len());
                i2 = Self::dec(i2, contour2.len());
                count -= 1;
            } else {
                break;
            }
        }
        let c1_start = Self::inc(i1, contour1.len());
        let c2_end = Self::dec(i2, contour2.len());
        let mut c: core::Vector<Point> = core::Vector::default();
        let mut i = c1_start;
        while i != c1_end {
            c.push(contour1.get(i).unwrap());
            i = Self::inc(i, contour1.len());
        }
        let mut i = c2_start;
        while i != c2_end {
            c.push(contour2.get(i).unwrap());
            i = Self::inc(i, contour2.len());
        }
        Ok(c)
    }
    fn contour(image: &Mat, threshold: f64) -> Result<core::Vector<Point>, Error> {
        let error = Error::new("FineContours", "max_contour");
        let mut contours: core::Vector<core::Vector<Point>> = core::Vector::default();
        log::trace!("FineContours.eval | contours...");
        imgproc::find_contours(
            image,
            &mut contours,
            imgproc::RetrievalModes::RETR_EXTERNAL as i32,
            imgproc::ContourApproximationModes::CHAIN_APPROX_SIMPLE as i32,
            core::Point2i::new(0, 0),
        ).map_err(|err| error.pass(err.to_string()))?;
        let mut contours = VecDeque::from_iter(contours.iter().filter(|c| c.len() > 3));
        let mut count = 1;
        while count > 0 {
            let mut found = None;
            'contour1: for (i1, contour1) in contours.iter().enumerate() {
                for (i2, contour2) in contours.iter().enumerate().filter(|(i, _)| i1 != *i) {
                    if let Ok(hull) = Self::merge(&contour1, &contour2, threshold as f32) {
                        if hull.len() > 0 {
                                    found = Some((i1, i2, hull));
                                    break 'contour1;
                        }
                    }
                }
            }
            match found {
                Some((i1, i2, hull)) => {
                    contours = contours.into_iter().enumerate().filter_map(|(i, h)| {
                        match [i1, i2].contains(&i) {
                            true => None,
                            false => Some(h),
                        }
                    }).collect();
                    contours.push_front(hull);
                }
                None => {
                    count -= 1;
                }
            }
        }
        log::trace!("FineContours.eval | contours: {}", contours.len());
        let contour = contours.into_iter().max_by(|c1, c2| {
            let area1 = imgproc::contour_area(c1, false).ok();
            let area2 = imgproc::contour_area(c2, false).ok();
            match (area1, area2) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Equal,
                (Some(_), None) => std::cmp::Ordering::Equal,
                (Some(area1), Some(area2)) => match area1.partial_cmp(&area2) {
                    Some(cmp) => cmp,
                    None => std::cmp::Ordering::Equal,
                }
            }
        });
        match contour {
            Some(contour) => Ok(contour),
            None => Err(error.err("Max contour isn't found")),
        }
    }
}
impl Eval<Image, EvalResult> for FineContours {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FineContours", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let thresh = self.thresh_ctx.eval(frame.mat.clone()).map_err(|err| error.pass(err))?;
                let contour = Self::contour(&thresh, self.conf.merge_distance).map_err(|err| error.pass(err))?;
                let mut convex = Mat::new_nd_vec_with_default(
                    &core::Vector::from_slice(&[thresh.rows(), thresh.cols()]),
                    core::CV_8UC1,
                    core::Vec4d::from_array([0.0, 0.0, 0.0, 0.0]),
                ).map_err(|err| error.pass(err.to_string()))?;
                if !contour.is_empty() {
                    let mut convex_contour: core::Vector<Point> = core::Vector::default();
                    imgproc::convex_hull(&contour, &mut convex_contour, true, true)
                        .map_err(|err| error.pass(err.to_string()))?;
                    imgproc::fill_poly(&mut convex, &convex_contour, core::Vec4d::from_array([255.0, 255.0, 255.0, 255.0]), LineTypes::LINE_8 as i32, 0, Point2i::new(0, 0))
                        .map_err(|err| error.pass(err.to_string()))?;
                }
                let frame = Image::from(thresh, meta);
                let ctx = match self.debug {
                    true => ctx.write(FineContoursCtx { result: frame.clone() }).map_err(|err| error.pass(err))?,
                    false => ctx,
                };
                let ctx = ctx.write(FineConvexCtx { convex: Some(Image::from(convex, meta)) }).map_err(|err| error.pass(err))?;
                let result = ResultCtx { val: frame };
                log::debug!("FineContours.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
struct PassCvMat {}
impl PassCvMat {
    fn new() -> Self {
        Self {  }
    }
}
impl Eval<Mat, Result<Mat, Error>> for PassCvMat {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        Ok(mat)
    }
}
}
mod fine_convex_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct FineConvexCtx {
    pub convex: Option<Image>,
}
impl Default for FineConvexCtx {
    fn default() -> Self {
        Self {
            convex: Default::default(),
         }
    }
}
}
pub use fine_contours_conf::*;
pub use fine_contours_ctx::*;
pub use fine_contours::*;
pub use fine_convex_ctx::*;}
mod fine_edges {
mod fine_edges_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FineEdgesConf {
    pub otsu_tune: Option<f64>,
    pub threshold: Option<u8>,
    pub smooth: Option<f64>,
}
impl FineEdgesConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "FineEdgesConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let otsu_tune = conf.get("otsu-tune");
        log::trace!("{dbg}.new | otsu-tune: {:#?}", otsu_tune);
        let threshold = conf.get("threshold").map(|val: u64| val as u8);
        log::trace!("{dbg}.new | threshold: {:#?}", threshold);
        let otsu_tune = match (otsu_tune, threshold) {
            (None, None) => Some(1.0),
            (None, Some(_)) => None,
            (Some(otsu_tune), None) => Some(otsu_tune),
            (Some(_), Some(_)) => panic!("{dbg}.new |  'otsu-tune' and 'threshold' - both specified, use on of them, otsu auto threshol with 'otsu-tune' or static 'threshold'"),
        };
        let smooth = conf.get("smooth");
        let smooth = match smooth {
            Some(smooth) => if smooth <= 0.0 {
                None
            } else {
                Some(smooth)
            }
            None => None,
        };
        log::trace!("{dbg}.new | smooth: {:#?}", smooth);
        Self {
            otsu_tune,
            threshold,
            smooth,
        }
    }
}
impl Default for FineEdgesConf {
    fn default() -> Self {
        Self {
            otsu_tune: Some(1.0),
            threshold: None,
            smooth: None,
        }
    }
}
}
mod fine_edges_ctx {
use crate::{algorithm::Edges};
#[derive(Debug, Clone, PartialEq)]
pub struct FineEdgesCtx {
    pub edges: Edges<usize>,
}
impl Default for FineEdgesCtx {
    fn default() -> Self {
        Self {
            edges: Edges::default()
         }
    }
}
}
mod fine_edges {
use std::time::Instant;
use opencv::{core::{Mat, MatTraitConst, MatTraitConstManual}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, FineEdgesCtx, Edges, ResultCtx},
    domain::{Dot, Eval, Filter, FilterEmpty, FilterSmooth2, Image},
};
pub struct FineEdges {
    otsu_tune: Option<f64>,
    threshold: Option<u8>,
    smooth: Option<f64>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
impl FineEdges {
    pub fn new(otsu_tune: Option<f64>, threshold: Option<u8>, smooth: Option<f64>, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            otsu_tune,
            threshold,
            smooth,
            ctx: Box::new(ctx),
        }
    }
}
impl Eval<Image, EvalResult> for FineEdges {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FineEdges", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let threshold = match (self.otsu_tune, self.threshold) {
                    (None, None) => imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap().round() as u8,
                    (None, Some(threshold)) => threshold,
                    (Some(otsu_tune), None) => (imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * otsu_tune).round() as u8,
                    (Some(otsu_tune), Some(_)) => (imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * otsu_tune).round() as u8,
                };
                log::trace!("FineEdges.eval | threshold: {threshold}");
                let rows = frame.mat.rows();
                let cols = frame.mat.cols();
                let mut upper_edge = Vec::with_capacity(cols as usize);
                let mut lower_edge = Vec::with_capacity(cols as usize);
                let (mut filter_smooth_upper, mut filter_smooth_lower): (Box<dyn Filter<Item = i32>>, Box<dyn Filter<Item = i32>>) = match self.smooth {
                    Some(smooth) => (Box::new(FilterSmooth2::<i32>::new(None, smooth)), Box::new(FilterSmooth2::<i32>::new(None, smooth))),
                    None => (Box::new(FilterEmpty::new()), Box::new(FilterEmpty::new())),
                };
                let mut upper;
                let mut lower;
                let mat = frame.mat.data_bytes().unwrap();
                for x in 0..cols {
                    upper = false;
                    lower = false;
                    for y in 0..rows {
                        if !upper {
                            match mat.get((y * cols + x) as usize) {
                                Some(pixel_value) => {
                                    if pixel_value > &threshold {
                                        if let Some(y) = filter_smooth_upper.add(y) {
                                            upper_edge.push(Dot {x: x as usize, y: y as usize});
                                            upper = true;
                                        }
                                    }
                                }
                                None => {
                                    return Err(error.err("Input image format error, index out of image range"));
                                }
                            }
                        }
                        let y = rows - y -1;
                        if !lower {
                            match mat.get((y * cols + x) as usize) {
                                Some(pixel_value) => {
                                    if pixel_value > &threshold {
                                        if let Some(y) = filter_smooth_lower.add(y) {
                                            lower_edge.push(Dot {x: x as usize, y: y as usize});
                                            lower = true;
                                        }
                                    }
                                }
                                None => {
                                    return Err(error.err("Input image format error, index out of image range"));
                                }
                            }
                        }
                        if upper && lower {
                            break;
                        }
                    }
                }
                let result = FineEdgesCtx {
                    edges: Edges::new(upper_edge, lower_edge),
                };
                log::trace!("FineEdges.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
pub use fine_edges_conf::*;
pub use fine_edges_ctx::*;
pub use fine_edges::*;
}
mod fine_union {
mod fine_union_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct FineUnionCtx {
    pub frame: Image,
}
impl Default for FineUnionCtx {
    fn default() -> Self {
        Self {
            frame: Image::default()
         }
    }
}
}
mod fine_union {
use std::{sync::Arc, time::Instant};
use opencv::core::MatTraitConst;
use parking_lot::RwLock;
use sal_core::error::Error;
use sal_sync::{services::future::Future, thread_pool::Scheduler};
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, FineConvexCtx, FineUnionCtx, ResultCtx}, conf::UnionConf, domain::{Eval, Image}
};
pub struct FineUnion {
    conf: UnionConf,
    scheduler: Scheduler,
    ctx1: Arc<RwLock<Box<dyn Eval<Image, EvalResult> + Send + Sync>>>,
    ctx2: Arc<RwLock<Box<dyn Eval<Image, EvalResult> + Send + Sync>>>,
}
impl FineUnion {
    pub fn new(
        conf: UnionConf,
        scheduler: Scheduler,
        ctx1: impl Eval<Image, EvalResult> + Send + Sync + 'static,
        ctx2: impl Eval<Image, EvalResult> + Send + Sync + 'static
    ) -> Self {
        Self {
            conf,
            scheduler,
            ctx1: Arc::new(RwLock::new(Box::new(ctx1))),
            ctx2: Arc::new(RwLock::new(Box::new(ctx2))),
        }
    }
}
impl Eval<Image, EvalResult> for FineUnion {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("FineUnion", "eval");
        let meta = frame.meta;
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
                log::trace!("FineUnion.eval | src1: {}x{}", src1_mat.cols(), src1_mat.rows());
                let src2: &ResultCtx<Image> = ctx2.read();
                let src2_mat = &src2.val.mat;
                log::trace!("FineUnion.eval | src2: {}x{}", src2_mat.cols(), src2_mat.rows());
                let mut dst = opencv::core::Mat::default();
                match (self.conf.add_weighted, self.conf.bitwise_and) {
                    (None, Some(_)) => opencv::core::bitwise_and(src1_mat, src2_mat, &mut dst, &opencv::core::no_array())
                        .map_err(|err| error.pass(err.to_string()))?,
                    (Some(conf), None) => opencv::core::add_weighted_def(src1_mat, conf.weight1, src2_mat, conf.weight2, conf.gamma, &mut dst)
                        .map_err(|err| error.pass(err.to_string()))?,
                    _ => Err(error.err(format!("Both: 'add-weighted' and `bitwise-and` - are specified, please use one of")))?,
                }
                let convex1: &FineConvexCtx = ctx1.read();
                let convex2: &FineConvexCtx = ctx2.read();
                let (convex, ctx) = match (&convex1.convex, &convex2.convex) {
                    (None, None) => Err(error.err("Can't find convex in the context"))?,
                    (None, Some(convex)) => (Some(convex.clone()), ctx2),
                    (Some(convex), None) => (Some(convex.clone()), ctx1),
                    (Some(convex), Some(_)) => (Some(convex.clone()), ctx1),
                };
                let dst = match convex {
                    Some(convex) => {
                        let mut out = opencv::core::Mat::default();
                        opencv::core::bitwise_and(&dst, &convex.mat, &mut out, &opencv::core::no_array())
                            .map_err(|err| error.pass(err.to_string()))?;
                        out
                    }
                    None => dst,
                };
                let frame = Image::from(dst, meta);
                let union = FineUnionCtx { frame: frame.clone() };
                let ctx = ctx.write(union).map_err(|err| error.pass(err))?;
                let result = ResultCtx { val: frame };
                log::trace!("FineUnion.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            (Ok(_), Err(err)) => Err(error.pass(err)),
            (Err(err), Ok(_)) => Err(error.pass(err)),
            (Err(err), Err(_)) => Err(error.pass(err)),
        }
    }
}
}
pub use fine_union_ctx::*;
pub use fine_union::*;
}
mod fine_scan_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::{
    FineContoursConf, FineEdgesConf, RopeDimensionsConf, TemporalFilterConf, Threshold
}, conf::UnionConf};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FineScanConf {
    pub fine_contours: FineContoursConf,
    pub temporal_filter: TemporalFilterConf,
    pub fine_edges: FineEdgesConf,
    pub union: UnionConf,
    pub rope_dimensions: RopeDimensionsConf,
    pub distortion_threshold: Threshold,
    pub defect_threshold: Threshold,
}
impl FineScanConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "FineScanConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let fine_contours = conf.get("fine-contours").expect(&format!("{dbg}.new | 'fine-contours' - not found or wrong configuration"));
        let fine_contours = FineContoursConf::new(&name, fine_contours);
        log::trace!("{dbg}.new | fine-contours: {:?}", fine_contours);
        let temporal_filter = conf.get("temporal-filter").expect(&format!("{dbg}.new | 'temporal-filter' - not found or wrong configuration"));
        let temporal_filter = TemporalFilterConf::new(&name, temporal_filter);
        log::trace!("{dbg}.new | temporal-filter: {:#?}", temporal_filter);
        let fine_edges = conf.get("fine-edges").expect(&format!("{dbg}.new | 'fine-edges' - not found or wrong configuration"));
        let fine_edges = FineEdgesConf::new(&name, fine_edges);
        log::trace!("{dbg}.new | fine-edges: {:#?}", fine_edges);
        let union = conf.get("union").expect(&format!("{dbg}.new | 'union' - not found or wrong configuration"));
        let union = UnionConf::new(&name, union);
        log::trace!("{dbg}.new | union: {:#?}", union);
        let rope_dimensions = conf.get("rope-dimensions").expect(&format!("{dbg}.new | 'rope-dimensions' - not found or wrong configuration"));
        let rope_dimensions = RopeDimensionsConf::new(&name, rope_dimensions);
        log::trace!("{dbg}.new | rope-dimensions: {:#?}", rope_dimensions);
        let distortion_threshold = conf.get("distortion-threshold").expect(&format!("{dbg}.new | 'distortion-threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | distortion-threshold: {:?}", distortion_threshold);
        let defect_threshold = conf.get("defect-threshold").expect(&format!("{dbg}.new | 'defect-threshold' - not found or wrong configuration"));
        log::trace!("{dbg}.new | defect-threshold: {:?}", defect_threshold);
        Self {
            fine_contours,
            temporal_filter,
            fine_edges,
            union,
            rope_dimensions,
            distortion_threshold: Threshold(distortion_threshold),
            defect_threshold: Threshold(defect_threshold),
        }
    }
}
impl Default for FineScanConf {
    fn default() -> Self {
        Self {
            fine_contours: FineContoursConf::default(),
            temporal_filter: TemporalFilterConf::default(),
            fine_edges: FineEdgesConf::default(),
            union: UnionConf::default(),
            rope_dimensions: RopeDimensionsConf::default(),
            distortion_threshold: Threshold::default(),
            defect_threshold: Threshold::default(),
        }
    }
}
}
mod fine_scan_ctx {
use crate::algorithm::{
    FineEdgesCtx, FineContoursCtx, FineUnionCtx, RopeDimensionsCtx, TemporalFilterCtx, RopeDefectCtx, RopeDistortionsCtx,
};
#[derive(Debug, Clone)]
pub struct FineScanCtx {
    pub fine_contours: FineContoursCtx,
    pub temporal_filter: TemporalFilterCtx<FineScanCtx>,
    pub union: FineUnionCtx,
    pub edges: FineEdgesCtx,
    pub rope_dimensions: RopeDimensionsCtx<FineScanCtx>,
    pub width_emissions: RopeDistortionsCtx<FineScanCtx>,
    pub defects: RopeDefectCtx<FineScanCtx>,
}
impl Default for FineScanCtx {
    fn default() -> Self {
        Self {
            fine_contours: FineContoursCtx::default(),
            temporal_filter: TemporalFilterCtx::default(),
            union: FineUnionCtx::default(),
            edges: FineEdgesCtx::default(),
            rope_dimensions: RopeDimensionsCtx::default(),
            width_emissions: RopeDistortionsCtx::default(),
            defects: RopeDefectCtx::default(),
        }
    }
}
}
mod fine_scan {
use std::{sync::Arc, time::Instant};
use sal_core::error::Error;
use sal_sync::{services::future::Future, sync::Owner, thread_pool::Scheduler};
use crate::{
    algorithm::{
        Context, ContextRead, ContextWrite, EvalResult, FineContours, FineEdges, FineScanConf, FineScanCtx, FastScanCtx,
        FineUnion, RopeDefectCtx, GrayCtx, TemporalFilter, ResultCtx, RopeDefect, RopeDistortions, RopeDistortionsCtx, Mad,
    }, domain::{Eval, Image},
};
pub struct FineScan {
    pass_ctx1: Arc<Owner<Context>>,
    pass_ctx2: Arc<Owner<Context>>,
    defects: Option<Arc<Box<dyn Fn(&Context) + Send + Sync>>>,
    ctx_fast: Box<dyn Eval<Image, EvalResult>>,
    ctx: Arc<Box<dyn Eval<Image, EvalResult> + Send + Sync>>,
    scheduler: Scheduler,
}
impl FineScan {
    #[allow(unused)]
    pub fn new(
        conf: FineScanConf,
        scheduler: Scheduler,
        defects: Option<impl Fn(&Context) + 'static + Send + Sync>,
        ctx: impl Eval<Image, EvalResult> + 'static,
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
            ctx_fast: Box::new(ctx),
            ctx: Arc::new(Box::new(
                RopeDefect::<FineScanCtx>::new(
                    conf.defect_threshold,
                    *Box::new(Mad::new()),
                    RopeDistortions::<FineScanCtx>::new(
                        conf.distortion_threshold,
                        *Box::new(Mad::new()),
                        FineEdges::new(
                            conf.fine_edges.otsu_tune,
                            conf.fine_edges.threshold,
                            conf.fine_edges.smooth,
                            FineUnion::new(
                                conf.union,
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
                    )
                ),
            )),
            scheduler,
        }
    }
}
impl Eval<Image, Future<Result<Context, Error>>> for FineScan {
    fn eval(&self, frame: Image) -> Future<Result<Context, Error>> {
        let error = Error::new("FineScan", "eval");
        let (future, sink) = Future::new();
        let sink1 = sink.clone();
        let result = match self.ctx_fast.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let distortions: &RopeDistortionsCtx<FastScanCtx> = ctx.read();
                if !distortions.result.is_empty() {
                    let result: &GrayCtx = ctx.read();
                    let result = ResultCtx { val: result.frame.clone() };
                    match ctx.write(result) {
                        Ok(ctx) => {
                            log::trace!("FineScan.eval | ctx size: {:?}", size_of_val(&ctx));
                            self.pass_ctx1.replace(ctx.clone());
                            self.pass_ctx2.replace(ctx);
                            let ctx_eval = self.ctx.clone();
                            let defects = self.defects.clone();
                            let handle = self.scheduler.spawn(move || {
                                let error = Error::new("FineScan", "eval");
                                match ctx_eval.eval(Image::default()) {
                                    Ok(ctx) => {
                                        log::debug!("FineScan.eval | Elapsed: {:?}", t.elapsed());
                                        if let Some(defects) = defects {
                                            let defects_ctx: &RopeDefectCtx<FineScanCtx> = ctx.read();
                                            if !defects_ctx.result.is_empty() {
                                                (defects)(&ctx)
                                            }
                                        }
                                        sink1.add(Ok(ctx));
                                    }
                                    Err(err) => sink1.add(Err(error.pass(err))),
                                }
                                Ok(())
                            });
                            handle.map(|_| ()).map_err(|err| error.pass(err))
                        }
                        Err(err) => Err(error.pass(err)),
                    }
                } else {
                    sink1.add(Ok(ctx));
                    Ok(())
                }
            }
            Err(err) => Err(error.pass(err)),
        };
        if let Err(err) = result {
            sink.add(Err(error.pass(err)));
        }
        future
    }
}
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
}
pub use fine_contours::*;
pub use fine_edges::*;
pub use fine_union::*;
pub use fine_scan_conf::*;
pub use fine_scan_ctx::*;
pub use fine_scan::*;}
mod gaussian_blur {
mod gaussian_blur_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct GaussianBlurCtx {
    pub frame: Image,
}
impl Default for GaussianBlurCtx {
    fn default() -> Self {
        Self {
            frame: Image::default()
         }
    }
}
}
mod gaussian_blur {
use std::time::Instant;
use opencv::{core::{Mat, Size}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, ResultCtx},
    domain::{Eval, Image},
};
pub struct GaussianBlur {
    kernel: [i32; 2],
    sigma: [f64; 2],
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
}
impl GaussianBlur {
    pub fn new(kernel: [i32; 2], sigma: [f64; 2], ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            kernel,
            sigma,
            ctx: Box::new(ctx),
            debug,
        }
    }
}
impl Eval<Image, EvalResult> for GaussianBlur {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("GaussianBlur", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mut blurred = Mat::default();
                match imgproc::gaussian_blur(
                    &frame.mat,
                    &mut blurred,
                    Size::new(self.kernel[0], self.kernel[1]),
                    self.sigma[0], self.sigma[1],
                    opencv::core::BORDER_DEFAULT,
                ) {
                    Ok(_) => {
                        let frame = Image::from(blurred, meta);
                        let result = ResultCtx { val: frame };
                        log::debug!("GaussianBlur.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(result)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
pub use gaussian_blur_ctx::*;
pub use gaussian_blur::*;
}
mod graham {
mod find_start {
use crate::domain::{Dot, Eval};
pub struct FindStart {
    points: Option<Vec<Dot<isize>>>,
}
impl FindStart {
    pub fn new(points: impl Into<Vec<Dot<isize>>>) -> Self {
        Self {
            points: Some(points.into()),
        }
    }
}
impl Eval<(), FindStartCtx> for FindStart {
    fn eval(&self, _: ()) -> FindStartCtx {
        let points = self.points.clone().take().unwrap();
        let start = points
            .iter()
            .enumerate()
            .min_by(|(_, dot1), (_, dot2)| {
                dot1.y.cmp(&dot2.y)
            });
        match start {
            Some((start, _)) => FindStartCtx { points, start: start as isize },
            None => FindStartCtx { points, start: 0 },
        }
    }
}
#[derive(Debug, Clone)]
pub struct FindStartCtx {
    pub points: Vec<Dot<isize>>,
    pub start: isize,
}
}
mod sort {
use std::cmp::Ordering;
use crate::domain::{Dot, Eval};
use super::{find_start::FindStartCtx};
pub struct Sort {
    eval: Box<dyn Eval<(), FindStartCtx>>,
}
impl Sort {
    pub fn new(eval: impl Eval<(), FindStartCtx> + 'static) -> Self {
        Self {
            eval: Box::new(eval),
        }
    }
}
impl Eval<(), SortByAngCtx> for Sort {
    fn eval(&self, _: ()) -> SortByAngCtx {
        let mut ctx = self.eval.eval(());
        let dot0 = ctx.points[ctx.start as usize];
        ctx.points.sort_by(|dot1, dot2| {
            let ang = (dot1.x - dot0.x) * (dot2.y - dot0.y) - (dot1.y - dot0.y) * (dot2.x - dot0.x);
            if ang > 0 {
                Ordering::Greater
            } else if ang < 0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        SortByAngCtx { points: ctx.points, start: ctx.start }
    }
}
#[derive(Debug)]
pub struct SortByAngCtx {
    pub points: Vec<Dot<isize>>,
    pub start: isize,
}}
pub use find_start::*;
pub use sort::*;
}
mod gray {
mod gray_ctx {
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct GrayCtx {
    pub frame: Image,
}
impl Default for GrayCtx {
    fn default() -> Self {
        Self {
            frame: Image::default()
         }
    }
}
}
mod gray {
use std::time::Instant;
use opencv::imgproc;
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, GrayCtx, ResultCtx},
    domain::{Eval, Image},
};
pub struct Gray {
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
}
impl Gray {
    pub fn new(ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
impl Eval<Image, EvalResult> for Gray {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("Gray", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mut gray = opencv::core::Mat::default();
                match imgproc::cvt_color(&frame.mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0) {
                    Ok(_) => {
                        let frame = Image::from(gray, meta);
                        let ctx = ctx.write(GrayCtx { frame: frame.clone() }).map_err(|err| error.pass(err))?;
                        log::trace!("Gray.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(ResultCtx { val: frame })
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
pub use gray_ctx::*;
pub use gray::*;
}
mod initial_ctx {
mod initial_ctx {
use sal_core::dbg::Dbg;
#[derive(Debug, Clone)]
pub struct InitialCtx {
    dbg: Dbg,
}
impl InitialCtx {
    pub fn new() -> Self {
        let dbg = Dbg::own("InitialCtx");
        Self {
            dbg,
        }
    }
}
}
pub use initial_ctx::*;
}
mod mad {
mod bend {
use crate::domain::Dot;
#[derive(Debug, Clone, PartialEq)]
pub struct Bend<T> {
    pub upper: Vec<Dot<T>>,
    pub lower: Vec<Dot<T>>,
}
impl<T> Bend<T>  {
    pub fn new() -> Self {
        Self { upper: vec![], lower: vec![] }
    }
    pub fn push(&mut self, upper: Dot<T>, lower: Dot<T>) {
        self.upper.push(upper);
        self.lower.push(lower);
    }
}
impl From<&[usize; 4]> for Bend<usize> {
    fn from(bend: &[usize; 4]) -> Self {
        Bend { upper: vec![Dot::from([bend[0], bend[1]])], lower: vec![Dot::from([bend[2], bend[3]])] }
    }
}
impl From<[usize; 4]> for Bend<usize> {
    fn from(bend: [usize; 4]) -> Self {
        Bend { upper: vec![Dot::from([bend[0], bend[1]])], lower: vec![Dot::from([bend[2], bend[3]])] }
    }
}
}
mod mad_ctx {
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MadCtx {
    pub median: f64,
    pub mad: f64,
}}
mod mad {
use sal_core::error::Error;
use crate::domain::Eval;
use super::MadCtx;
pub struct Mad;
impl Mad {
    pub fn new() -> Self {
        Self {}
    }
    pub fn median(sample: &[usize]) -> Result<usize, Error> {
        let len = sample.len();
        match len {
            0 => Err(Error::new("Mad", "median").err("Input sequence is empty")),
            1 => Ok(sample[0]),
            _ => {
                let half_len = len / 2;
                if len % 2 == 1 {
                    Ok(sample[half_len])
                } else {
                    Ok(((sample[half_len - 1] + sample[half_len]) as f64 * 0.5).round() as usize)
                }
            }
        }
    }
    fn mad(sample: &[usize], median: usize) -> Result<usize, Error> {
        let mut sample: Vec<usize> = sample.iter().map(|v| (*v as isize - median as isize).abs() as usize).collect();
        sample.sort_by(|a, b| a.cmp(b));
        Self::median(&sample)
    }
}
impl Eval<Vec<usize>, Result<MadCtx, Error>> for Mad {
    fn eval(&self, mut sample: Vec<usize>) -> Result<MadCtx, Error> {
        sample.sort_by(|a, b| a.cmp(b));
        match Self::median(&sample) {
            Ok(median) => Ok(MadCtx {
                median: median as f64,
                mad: {
                    let mad = Self::mad(&sample, median).map_err(|err| Error::new("Mad", "eval").pass(err))? as f64;
                    mad
                }
            }),
            Err(err) => Err(Error::new("Mad", "eval").pass(err)),
        }
    }
}
}
pub use bend::*;
pub use mad_ctx::*;
pub use mad::*;
}
mod rope_defect {
mod rope_defect_ctx {
use std::marker::PhantomData;
use crate::algorithm::RopeDefectKind;
#[derive(Debug, Clone, Default)]
pub struct RopeDefectCtx<Branch> {
    pub result: Vec<RopeDefectKind>,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDefectCtx<Branch> {
    pub fn new(result: Vec<RopeDefectKind>) -> Self {
        Self { result, branch: PhantomData }
    }
}}
mod rope_defect_kind {
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RopeDefectKind {
    Expansion(usize, usize),
    Compressing(usize, usize),
    Hill(usize, usize),
    Pit(usize, usize),
}
impl RopeDefectKind {
    pub fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (RopeDefectKind::Expansion(_, _), RopeDefectKind::Expansion(_, _)) => true,
            (RopeDefectKind::Compressing(_, _), RopeDefectKind::Compressing(_, _)) => true,
            (RopeDefectKind::Hill(_, _), RopeDefectKind::Hill(_, _)) => true,
            (RopeDefectKind::Pit(_, _), RopeDefectKind::Pit(_, _)) => true,
            _ => false,
        }
    }
    pub fn start(&self) -> usize {
        match self {
            RopeDefectKind::Expansion(start, _) => *start,
            RopeDefectKind::Compressing(start, _) => *start,
            RopeDefectKind::Hill(start, _) => *start,
            RopeDefectKind::Pit(start, _) => *start,
        }
    }
    pub fn end(&self) -> usize {
        match self {
            RopeDefectKind::Expansion(_, end) => *end,
            RopeDefectKind::Compressing(_, end) => *end,
            RopeDefectKind::Hill(_, end) => *end,
            RopeDefectKind::Pit(_, end) => *end,
        }
    }
    pub fn start_with(&self, start: usize) -> RopeDefectKind {
        match self {
            RopeDefectKind::Expansion(_, end) => RopeDefectKind::Expansion(start, *end),
            RopeDefectKind::Compressing(_, end) => RopeDefectKind::Compressing(start, *end),
            RopeDefectKind::Hill(_, end) => RopeDefectKind::Hill(start, *end),
            RopeDefectKind::Pit(_, end) => RopeDefectKind::Pit(start, *end),
        }
    }
    pub fn end_with(&self, end: usize) -> RopeDefectKind {
        match self {
            RopeDefectKind::Expansion(start, _) => RopeDefectKind::Expansion(*start, end),
            RopeDefectKind::Compressing(start, _) => RopeDefectKind::Compressing(*start, end),
            RopeDefectKind::Hill(start, _) => RopeDefectKind::Hill(*start, end),
            RopeDefectKind::Pit(start, _) => RopeDefectKind::Pit(*start, end),
        }
    }
}}
mod rope_defect {
use std::{any::TypeId, marker::PhantomData};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        RopeDefectCtx, RopeDefectKind, Threshold,
        mad::MadCtx, RopeDistortions, RopeDistortionsCtx, ContextRead, ContextWrite,
        FastEdgesCtx, FineEdgesCtx, EvalResult, Side,
        FastScanCtx, FineScanCtx,
    },
    domain::{Dot, Error, Eval, Image},
};
pub struct RopeDefect<Branch> {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync>,
    ctx: RopeDistortions<Branch>,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDefect<Branch> {
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync + 'static,
        ctx: RopeDistortions<Branch>,
    ) -> Self {
        Self {
            dbg: Dbg::own("RopeDefect"),
            threshold,
            mad: Box::new(mad),
            ctx,
            branch: PhantomData,
        }
    }
    fn expansion(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper < self.threshold.0 * upper_mad.mad) &&
            (deviation_lower > self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }
    fn compressing(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper > self.threshold.0 * upper_mad.mad) &&
            (deviation_lower < self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }
    fn pit(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper.abs() < self.threshold.0 * upper_mad.mad) &&
        (deviation_lower > self.threshold.0 * lower_mad.mad) {
            return Some(());
        } else if (deviation_upper > self.threshold.0 * upper_mad.mad) &&
            (deviation_lower.abs() < self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }
    fn hill(&self, upper_point: &Dot<usize>, lower_point: &Dot<usize>, upper_mad: &MadCtx, lower_mad: &MadCtx) -> Option<()> {
        let deviation_upper = upper_point.y as f64 - upper_mad.median;
        let deviation_lower = lower_point.y as f64 - lower_mad.median;
        if (deviation_upper.abs() < self.threshold.0 * upper_mad.mad) &&
            (deviation_lower < -self.threshold.0 * lower_mad.mad) {
            return Some(());
        } else if (deviation_upper < -self.threshold.0 * upper_mad.mad) &&
            (deviation_lower.abs() < self.threshold.0 * lower_mad.mad) {
            return Some(());
        }
        None
    }
}
impl<Branch: 'static> Eval<Image, EvalResult> for RopeDefect<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let rope_distortions = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => &ContextRead::<RopeDistortionsCtx<FastScanCtx>>::read(&ctx).result,
                    typ if typ == TypeId::of::<FineScanCtx>() => &ContextRead::<RopeDistortionsCtx<FineScanCtx>>::read(&ctx).result,
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                if rope_distortions.is_empty() {
                    return match TypeId::of::<Branch>() {
                        typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(RopeDefectCtx::<FineScanCtx>::new(vec![])),
                        _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                    }
                }
                let (upper, lower) = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        let edges_ctx: &FastEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        let edges_ctx: &FineEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                let upper_mad = self.mad.eval(
                    upper.iter().map(|dot| dot.y).collect(),
                ).map_err(|err| error.pass(err))?;
                let lower_mad = self.mad.eval(
                    lower.iter().map(|dot| dot.y).collect()
                ).map_err(|err| error.pass(err))?;
                let mut result = Defects::new();
                let mut x = 0;
                for bend in rope_distortions {
                    for (upper_point, lower_point) in bend.upper.iter().zip(&bend.lower) {
                        match self.expansion(&upper_point, &lower_point, &upper_mad, &lower_mad) {
                            Some(_) => result.push(RopeDefectKind::Expansion(upper_point.x, upper_point.x)),
                            None => match self.compressing(&upper_point, &lower_point, &upper_mad, &lower_mad) {
                                Some(_) => result.push(RopeDefectKind::Compressing(upper_point.x, upper_point.x)),
                                None => match self.hill(&upper_point, &lower_point, &upper_mad, &lower_mad) {
                                    Some(_) => result.push(RopeDefectKind::Hill(upper_point.x, upper_point.x)),
                                    None => match self.pit(&upper_point, &lower_point, &upper_mad, &lower_mad) {
                                        Some(_) => result.push(RopeDefectKind::Pit(upper_point.x, upper_point.x)),
                                        None => result.no_defect(upper_point.x),
                                    }
                                }
                            }
                        }
                        x = upper_point.x;
                    }
                    result.no_defect(x);
                }
                match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(RopeDefectCtx::<FineScanCtx>::new(result.all())),
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}
struct Defects {
    items: Vec<RopeDefectKind>,
    prev: Option<RopeDefectKind>,
}
impl Defects {
    pub fn new() -> Self {
        Self { items: vec![], prev: None }
    }
    pub fn no_defect(&mut self, x: usize) {
        if let Some(prev) = &self.prev {
            let detected = prev.end_with(x);
            if detected.end() - detected.start() > 16 {
                self.items.push(detected);
            }
            self.prev = None;
        }
    }
    pub fn push(&mut self, defect: RopeDefectKind) {
        match &self.prev {
            Some(prev) => {
                if !defect.is_same(prev) {
                    if prev.is_same(&RopeDefectKind::Hill(0, 0)) && defect.is_same(&RopeDefectKind::Expansion(0, 0)) {
                        self.prev = Some(RopeDefectKind::Expansion(prev.start(), defect.end()));
                        return;
                    }
                    if prev.is_same(&RopeDefectKind::Expansion(0, 0)) && (defect.is_same(&RopeDefectKind::Pit(0, 0)) || defect.is_same(&RopeDefectKind::Hill(0, 0))) {
                        return;
                    }
                    if prev.is_same(&RopeDefectKind::Pit(0, 0)) && defect.is_same(&RopeDefectKind::Compressing(0, 0)) {
                        self.prev = Some(RopeDefectKind::Compressing(prev.start(), defect.end()));
                        return;
                    }
                    if prev.is_same(&RopeDefectKind::Compressing(0, 0)) && (defect.is_same(&RopeDefectKind::Pit(0, 0)) || defect.is_same(&RopeDefectKind::Hill(0, 0))) {
                        return;
                    }
                    let detected = prev.end_with(defect.start());
                    if detected.end() - detected.start() > 16 {
                        self.items.push(detected);
                    }
                    self.prev = Some(defect);
                }
            }
            None => {
                self.prev = Some(defect);
            }
        }
    }
    pub fn all(self) -> Vec<RopeDefectKind> {
        self.items
    }
}}
mod threshold {
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Threshold(pub f64);
#[allow(unused)]
impl Threshold {
    const MIN: Self = Self(1.1);
    const AVG: Self = Self(1.2);
    const MAX: Self = Self(1.3);
}
impl Default for Threshold {
    fn default() -> Self {
        Self::AVG
    }
}
}
pub use rope_defect_ctx::*;
pub use rope_defect_kind::*;
pub use rope_defect::*;
pub use threshold::*;
}
mod rope_dimensions {
mod rope_dimensions {
use std::{any::TypeId, marker::PhantomData, time::Instant};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, FastEdgesCtx, FineEdgesCtx, RopeDimensionsCtx, EvalResult, Side, FastScanCtx, FineScanCtx},
    domain::{Eval, Image},
};
pub struct RopeDimensions<Branch> {
    rope_width: f64,
    width_tolerance: f64,
    square_tolerance: f64,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDimensions<Branch> {
    #[allow(unused)]
    pub fn new(rope_width: usize, width_tolerance: f64, square_tolerance: f64, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static) -> Self {
        Self {
            rope_width: rope_width as f64,
            width_tolerance: width_tolerance / 100.0,
            square_tolerance: square_tolerance / 100.0,
            ctx: Box::new(ctx),
            branch: PhantomData,
        }
    }
}
impl<Branch: 'static> Eval<Image, EvalResult> for RopeDimensions<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("RopeDimensions", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let (upper, lower) = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        let edges_ctx: &FastEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        let edges_ctx: &FineEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                let mut upper_average = 0.0f64;
                let mut lower_average = 0.0f64;
                let mut rope_square = 0.0;
                for (upper, lower) in upper.iter().zip(&lower) {
                    upper_average += upper.y as f64;
                    lower_average += lower.y as f64;
                    rope_square += (upper.y as f64 - lower.y as f64).abs();
                }
                upper_average = upper_average / upper.len() as f64;
                lower_average = lower_average / lower.len() as f64;
                let rope_width = (upper_average - lower_average).abs();
                let rope_width_error = (1.0 - rope_width / self.rope_width).abs();
                if rope_width_error >= self.width_tolerance {
                    return Err(error.err(format!("Rope width error: {:.3}%, {rope_width} of {}", rope_width_error, self.rope_width)));
                }
                let rope_square_error = (1.0 - rope_square / (self.rope_width * upper.len() as f64)).abs();
                if rope_square_error >= self.square_tolerance {
                    return Err(error.err(format!("Rope square error: {:.3}%, {rope_square} of {}", rope_square_error, self.rope_width * upper.len() as f64)));
                }
                log::debug!("RopeDimensions.eval | Elapsed: {:?}", t.elapsed());
                match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => ctx.write(RopeDimensionsCtx::<FastScanCtx>::new(rope_width, rope_square)),
                    typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(RopeDimensionsCtx::<FineScanCtx>::new(rope_width, rope_square)),
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
mod rope_dimensions_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RopeDimensionsConf {
    pub rope_width: usize,
    pub width_tolerance: f64,
    pub square_tolerance: f64,
}
impl RopeDimensionsConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "RopeDimensionsConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let rope_width: u64 = conf.get("rope-width").expect(&format!("{dbg}.new | 'rope-width' - not found or wrong configuration"));
        log::trace!("{dbg}.new | rope-width: {:?}", rope_width);
        let width_tolerance = conf.get("width-tolerance").expect(&format!("{dbg}.new | 'width-tolerance' - not found or wrong configuration"));
        log::trace!("{dbg}.new | width-tolerance: {:?}", width_tolerance);
        let square_tolerance = conf.get("square-tolerance").expect(&format!("{dbg}.new | 'square-tolerance' - not found or wrong configuration"));
        log::trace!("{dbg}.new | square-tolerance: {:?}", square_tolerance);
        Self {
            rope_width: rope_width as usize,
            width_tolerance,
            square_tolerance,
        }
    }
}
impl Default for RopeDimensionsConf {
    fn default() -> Self {
        Self {
            rope_width: 35,
            width_tolerance: 10.0,
            square_tolerance: 10.0,
        }
    }
}
}
mod rope_dimensions_ctx {
use std::marker::PhantomData;
#[derive(Debug, Clone, Default)]
pub struct RopeDimensionsCtx<Branch> {
    pub width: f64,
    pub square: f64,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDimensionsCtx<Branch> {
    pub fn new(width: f64, square: f64) -> Self {
        Self { width, square, branch: PhantomData }
    }
}}
pub use rope_dimensions::*;
pub use rope_dimensions_conf::*;
pub use rope_dimensions_ctx::*;}
mod rope_distortions {
mod rope_distortions_ctx {
use std::marker::PhantomData;
use crate::algorithm::{Bend, MadCtx};
#[derive(Debug, Clone, Default)]
pub struct RopeDistortionsCtx<Branch> {
    pub result: Vec<Bend<usize>>,
    pub mad: MadCtx,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDistortionsCtx<Branch> {
    pub fn new(result: Vec<Bend<usize>>, mad: MadCtx) -> Self {
        Self {
            result,
            mad,
            branch: PhantomData }
    }
}}
mod rope_distortions {
use std::{any::TypeId, marker::PhantomData, time::Instant};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        Threshold, mad::{Bend, MadCtx},
        ContextRead, ContextWrite, FastEdgesCtx, FineEdgesCtx, EvalResult, Side,
        FastScanCtx, FineScanCtx, RopeDistortionsCtx,
    },
    domain::{Dot, Error, Eval, Image}
};
pub struct RopeDistortions<Branch> {
    dbg: Dbg,
    threshold: Threshold,
    mad: Box<dyn Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    branch: PhantomData<Branch>,
}
impl<Branch> RopeDistortions<Branch> {
    pub fn new(
        threshold: Threshold,
        mad: impl Eval<Vec<usize>, Result<MadCtx, Error>> + Send + Sync + 'static,
        ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static,
    ) -> Self {
        Self {
            dbg: Dbg::own("RopeDistortions"),
            threshold,
            mad: Box::new(mad),
            ctx: Box::new(ctx),
            branch: PhantomData,
        }
    }
    fn points_width(upper: &[Dot<usize>], lower: &[Dot<usize>]) -> Vec<usize> {
        let mut dots_width = Vec::new();
        for i in 0..upper.len() {
            let width = (upper[i].y as isize - lower[i].y as isize).abs() as usize;
            dots_width.push(width);
        };
        dots_width
    }
    fn distortion(
        upper: Vec<Dot<usize>>,
        lower: Vec<Dot<usize>>,
        median: f64,
        mad: f64,
        threshold: Threshold,
    ) -> Vec<Bend<usize>> {
        let mut distortion = Vec::new();
        let mut bend: Option<Bend<usize>> = None;
        for (upper, lower) in upper.iter().zip(&lower) {
            let deviation = ((upper.y as f64 - lower.y as f64).abs() - median).abs();
            if deviation > threshold.0 * mad {
                match &mut bend {
                    Some(bend) => bend.push(*upper, *lower),
                    None => {
                        let mut init = Bend::new();
                        init.push(*upper, *lower);
                        bend = Some(init);
                    },
                }
            } else {
                if let Some(bend) = bend.take() {
                    distortion.push(bend);
                }
            }
        };
        distortion
    }
}
impl<Branch: 'static> Eval<Image, EvalResult> for RopeDistortions<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new(&self.dbg, "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let (upper, lower) = match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        let edges_ctx: &FastEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        let edges_ctx: &FineEdgesCtx = ctx.read();
                        (edges_ctx.edges.get(Side::Upper), edges_ctx.edges.get(Side::Lower))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>())))?,
                };
                let (result, mad) = if upper.is_empty() || lower.is_empty() {
                    (vec![], MadCtx::default())
                } else {
                    let mad = self.mad.eval(Self::points_width(&upper, &lower)).map_err(|err| error.pass(err))?;
                    (Self::distortion(upper, lower, mad.median, mad.mad, self.threshold), mad)
                };
                match TypeId::of::<Branch>() {
                    typ if typ == TypeId::of::<FastScanCtx>() => {
                        log::trace!("RopeDistortions<FastScanCtx>.eval | Elapsed: {:?}", t.elapsed());
                        log::trace!("RopeDistortions<FastScanCtx>.eval | mad: {:?}", mad);
                        ctx.write(RopeDistortionsCtx::<FastScanCtx>::new(result, mad))
                    }
                    typ if typ == TypeId::of::<FineScanCtx>() => {
                        log::trace!("RopeDistortions<FineScanCtx>.eval | Elapsed: {:?}", t.elapsed());
                        log::trace!("RopeDistortions<FineScanCtx>.eval | mad: {:?}", mad);
                        ctx.write(RopeDistortionsCtx::<FineScanCtx>::new(result, mad))
                    }
                    _ => Err(error.err(format!("Can't read result from: '{:?}' branch of 'Context'", TypeId::of::<Branch>()))),
                }
            },
            Err(err) => Err(error.pass(err)),
        }
    }
}
}
pub use rope_distortions_ctx::*;
pub use rope_distortions::*;
}
mod temporal_filter {
mod temporal_filter_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::conf::GaussianConf;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemporalFilterConf {
    pub open_kernel: [i32; 2],
    pub erode_kernel: [i32; 2],
    pub gaussian: GaussianConf,
    pub threshold: f64,
}
impl TemporalFilterConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "TemporalFilterConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let gaussian = conf.get("gaussian").expect(&format!("{dbg}.new | 'gaussian' - not found or wrong configuration"));
        let gaussian = GaussianConf::new(&name, gaussian);
        log::trace!("{dbg}.new | gaussian: {:#?}", gaussian);
        let open_kernel: Vec<i32> = conf.as_vec("open-kernel").map(|val| {
            val.into_iter().map(|v| v.as_u64().expect(&format!("{dbg}.new | 'open-kernel' - wrong configuration")) as i32)
        }).expect(&format!("{dbg}.new | 'open-kernel' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | open-kernel: {:?}", open_kernel);
        let erode_kernel: Vec<i32> = conf.as_vec("erode-kernel").map(|val| {
            val.into_iter().map(|v| v.as_u64().expect(&format!("{dbg}.new | 'erode-kernel' - wrong configuration")) as i32)
        }).expect(&format!("{dbg}.new | 'erode-kernel' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | erode-kernel: {:?}", erode_kernel);
        let threshold = conf.get("threshold").unwrap_or(1.0);
        log::trace!("{dbg}.new | threshold: {:?}", threshold);
        Self {
            gaussian,
            open_kernel: open_kernel.try_into().expect(&format!("{dbg}.new | 'open-kernel' - wrong configuration")),
            erode_kernel: erode_kernel.try_into().expect(&format!("{dbg}.new | 'erode-kernel' - wrong configuration")),
            threshold,
        }
    }
}
impl Default for TemporalFilterConf {
    fn default() -> Self {
        Self {
            gaussian: GaussianConf::default(),
            open_kernel: [5; 2],
            erode_kernel: [5; 2],
            threshold: 1.0,
        }
    }
}
}
mod temporal_filter_ctx {
use std::marker::PhantomData;
use crate::domain::Image;
#[derive(Debug, Clone)]
pub struct TemporalFilterCtx<Branch> {
    pub frame: Image,
    branch: PhantomData<Branch>,
}
impl<Branch> TemporalFilterCtx<Branch> {
    pub fn new(frame: Image) -> Self {
        Self {
            frame,
            branch: PhantomData,
         }
    }
}
impl<Branch> Default for TemporalFilterCtx<Branch> {
    fn default() -> Self {
        Self {
            frame: Image::default(),
            branch: PhantomData,
         }
    }
}
}
mod temporal_filter {
use std::{any::TypeId, marker::PhantomData, time::Instant};
use opencv::{core::{self, Mat, MatTraitConst}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{cv, ContextRead, ContextWrite, EvalResult, ResultCtx, TemporalFilterCtx, FastScanCtx, FineScanCtx},
    conf::GaussianConf, domain::{Eval, Image, RwLock}
};
pub struct TemporalFilter<Branch> {
    threshold: f64,
    prev: RwLock<Option<Mat>>,
    proc: Box<dyn Eval<Mat, Result<Mat, Error>> + Send + Sync>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
    branch: PhantomData<Branch>,
}
impl<Branch> TemporalFilter<Branch> {
    pub fn new(gaussian: GaussianConf, open_kernel: [i32; 2], erode_kernel: [i32; 2], threshold: f64, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            threshold,
            prev: RwLock::new(None),
            proc: Box::new(
                cv::Morphology::erode(
                    &erode_kernel,
                    cv::Morphology::open(
                        &open_kernel,
                        cv::GaussianBlur::new(
                            &gaussian.kernel,
                            PassCvMat::new(),
                        )
                    ),
                ),
            ),
            ctx: Box::new(ctx),
            debug,
            branch: PhantomData,
        }
    }
}
impl<Branch: 'static> Eval<Image, EvalResult> for TemporalFilter<Branch> {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("TemporalFilter", "eval");
        let meta = frame.meta;
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx<Image> = ctx.read();
                let frame = &result.val;
                let mut prev_guard = self.prev.write();
                let mut dst = Mat::default();
                if let Some(prev) = prev_guard.as_mut() {
                    let mut diff = Mat::default();
                    core::absdiff(prev, &frame.mat, &mut diff)
                        .map_err(|err| error.clone().pass(err.to_string()))?;
                    imgproc::threshold(&diff, &mut dst, self.threshold, 255.0, imgproc::THRESH_BINARY)
                        .map_err(|err| error.clone().pass(err.to_string()))?;
                    frame.mat.copy_to(prev)
                        .map_err(|err| error.clone().pass(err.to_string()))?;
                } else {
                    dst = Mat::new_rows_cols_with_default(
                        frame.mat.rows(),
                        frame.mat.cols(),
                        core::CV_8UC1,
                        core::Scalar::all(0.0)
                    ).map_err(|err| error.clone().pass(err.to_string()))?;
                    let mut prev = Mat::default();
                    frame.mat.copy_to(&mut prev)
                        .map_err(|err| error.clone().pass(err.to_string()))?;
                    *prev_guard = Some(prev);
                }
                let dst = self.proc.eval(dst).map_err(|err| error.pass(err))?;
                let frame = Image::from(dst, meta);
                let ctx = if self.debug {
                    match TypeId::of::<Branch>() {
                        typ if typ == TypeId::of::<FastScanCtx>() => ctx.write(TemporalFilterCtx::<FastScanCtx>::new(frame.clone()))
                            .map_err(|err| error.pass(err))?,
                        typ if typ == TypeId::of::<FineScanCtx>() => ctx.write(TemporalFilterCtx::<FineScanCtx>::new(frame.clone()))
                            .map_err(|err| error.pass(err))?,
                        _ => {
                            log::warn!("TemporalFilter.eval | Can't write to result to: '{:?}' branch of 'Context'", TypeId::of::<Branch>());
                            ctx
                        }
                    }
                } else {
                    ctx
                };
                let result = ResultCtx { val: frame };
                log::trace!("TemporalFilter.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
struct PassCvMat {}
impl PassCvMat {
    fn new() -> Self {
        Self {  }
    }
}
impl Eval<Mat, Result<Mat, Error>> for PassCvMat {
    fn eval(&self, mat: Mat) -> Result<Mat, Error> {
        Ok(mat)
    }
}
}
pub use temporal_filter_conf::*;
pub use temporal_filter_ctx::*;
pub use temporal_filter::*;}
mod edges {
use indexmap::IndexMap;
use crate::domain::Dot;
#[derive(Debug, Clone)]
pub struct Edges<T> {
    sides: IndexMap<Side, Vec<Dot<T>>>,
}
impl<T: Copy> Edges<T> {
    pub fn new(upper: Vec<Dot<T>>, lower: Vec<Dot<T>>) -> Self {
        Self {
            sides: IndexMap::from([
                (Side::Upper, upper),
                (Side::Lower, lower),
            ])
        }
    }
    pub fn get(&self, side: Side) -> Vec<Dot<T>> {
        match self.sides.get(&side) {
            Some(side) => side.to_vec(),
            None => vec![],
        }
    }
}
impl<T> Default for Edges<T> {
    fn default() -> Self {
        Self { sides: IndexMap::<Side, Vec<Dot<T>>>::new() }
    }
}
impl<T: PartialEq> PartialEq for Edges<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sides == other.sides
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Side {
    Upper,
    Lower,
}}
pub use auto_correction::*;
pub use buffer::*;
pub use context::*;
pub use cropping::*;
pub use detecting_contours::*;
pub use fast_scan::*;
pub use fine_scan::*;
pub use gaussian_blur::*;
pub use graham::*;
pub use gray::*;
pub use initial_ctx::*;
pub use mad::*;
pub use rope_defect::*;
pub use rope_dimensions::*;
pub use rope_distortions::*;
pub use temporal_filter::*;
pub use edges::*;
}
mod conf {
mod detecting_contouts {
mod brightness_contrast {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, PartialEq)]
pub struct BrightnessContrastConf {
    pub hist_clip_left: f32,
    pub hist_clip_right: f32,
}
impl BrightnessContrastConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "BrightnessContrastConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let hist_clip_left = conf.get("hist-clip-left").unwrap_or(0.0);
        log::trace!("{dbg}.new | hist-clip-left: {:?}", hist_clip_left);
        let hist_clip_right = conf.get("hist-clip-right").unwrap_or(0.0);
        log::trace!("{dbg}.new | hist-clip-right: {:?}", hist_clip_right);
        Self {
            hist_clip_left: hist_clip_left as f32,
            hist_clip_right: hist_clip_right as f32,
        }
    }
}
impl Default for BrightnessContrastConf {
    fn default() -> Self {
        Self {
            hist_clip_left: 0.0,
            hist_clip_right: 0.0,
        }
    }
}
}
mod gamma_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GammaConf {
    pub factor: f64,
}
impl GammaConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "OverlayConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let factor = conf.get("factor").expect(&format!("{dbg}.new | 'factor' - not found or wrong configuration"));
        log::trace!("{dbg}.new | factor: {:?}", factor);
        Self {
            factor,
        }
    }
}
impl Default for GammaConf {
    fn default() -> Self {
        Self {
            factor: 100.0,
        }
    }
}
}
mod gaussian_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::ConfTree, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaussianConf {
    pub kernel: [i32; 2],
    pub sigma: [f64; 2],
}
impl GaussianConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "GaussianConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let kernel: Vec<i32> = conf.as_vec("kernel").map(|val| {
            val.into_iter().map(|v| v.as_u64().expect(&format!("{dbg}.new | 'kernel' - wrong configuration")) as i32)
        }).expect(&format!("{dbg}.new | 'kernel' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | kernel: {:?}", kernel);
        let sigma: Vec<f64> = conf.as_vec("sigma").map(|val| {
            val.into_iter().map(|v| v.as_f64().expect(&format!("{dbg}.new | 'sigma' - wrong configuration")))
        }).expect(&format!("{dbg}.new | 'sigma' - not found or wrong configuration")).collect();
        log::trace!("{dbg}.new | sigma: {:?}", sigma);
        Self {
            kernel: kernel.try_into().expect(&format!("{dbg}.new | 'kernel' - wrong configuration")),
            sigma: sigma.try_into().expect(&format!("{dbg}.new | 'sigma' - wrong configuration")),
        }
    }
}
impl Default for GaussianConf {
    fn default() -> Self {
        Self { kernel: [3, 3], sigma: [0.0, 0.0] }
    }
}
}
mod overlay_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OverlayConf {
    pub src1_weight: f64,
    pub src2_weight: f64,
    pub gamma: f64,
}
impl OverlayConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "OverlayConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let src1_weight = conf.get("alpha").unwrap_or(0.5);
        log::trace!("{dbg}.new | alpha: {:?}", src1_weight);
        let src2_weight = conf.get("beta").unwrap_or(0.5);
        log::trace!("{dbg}.new | beta: {:?}", src2_weight);
        let gamma = conf.get("gamma").unwrap_or(0.0);
        log::trace!("{dbg}.new | gamma: {:?}", gamma);
        Self {
            src1_weight,
            src2_weight,
            gamma,
        }
    }
}
impl Default for OverlayConf {
    fn default() -> Self {
        Self { src1_weight: 0.5, src2_weight: 0.5, gamma: 0.0 }
    }
}
}
mod sobel_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SobelConf {
    pub kernel_size: i32,
    pub scale: f64,
    pub delta: f64,
}
impl SobelConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "SobelConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let kernel_size: i64 = conf.get("kernel-size").unwrap_or(3);
        log::trace!("{dbg}.new | kernel-size: {:?}", kernel_size);
        let scale = conf.get("scale").unwrap_or(1.0);
        log::trace!("{dbg}.new | scale: {:?}", scale);
        let delta = conf.get("delta").unwrap_or(0.0);
        log::trace!("{dbg}.new | delta: {:?}", delta);
        Self {
            kernel_size: kernel_size as i32,
            scale,
            delta,
        }
    }
}
impl Default for SobelConf {
    fn default() -> Self {
        Self { kernel_size: 3, scale: 1.0, delta: 0.0 }
    }
}
}
pub use brightness_contrast::*;
pub use gamma_conf::*;
pub use gaussian_conf::*;
pub use overlay_conf::*;
pub use sobel_conf::*;
}
mod add_wighted_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AddWeightedConf {
    pub weight1: f64,
    pub weight2: f64,
    pub gamma: f64,
}
impl AddWeightedConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "AddWeightedConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let weight1 = conf.get("weight1").expect(&format!("{dbg}.new | 'weight1' - not found or wrong configuration"));
        log::trace!("{dbg}.new | weight1: {:#?}", weight1);
        let weight2 = conf.get("weight2").expect(&format!("{dbg}.new | 'weight2' - not found or wrong configuration"));
        log::trace!("{dbg}.new | weight2: {:#?}", weight1);
        let gamma = conf.get("gamma").unwrap_or(0.0);
        log::trace!("{dbg}.new | gamma: {:#?}", gamma);
        Self {
            weight1,
            weight2,
            gamma,
        }
    }
}
impl Default for AddWeightedConf {
    fn default() -> Self {
        Self {
            weight1: 1.0,
            weight2: 1.0,
            gamma: 0.0,
        }
    }
}
}
mod bitwise_and_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::ConfTree, entity::Name};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitwiseAndConf {
}
impl BitwiseAndConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "BitwiseAndConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        Self {
        }
    }
}
impl Default for BitwiseAndConf {
    fn default() -> Self {
        Self {
        }
    }
}
}
mod conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::{FastScanConf, FineScanConf}, conf::NormalizeConf};
#[derive(Debug, PartialEq, Clone)]
pub struct Conf {
    pub normalize: NormalizeConf,
    pub fast_scan: FastScanConf,
    pub fine_scan: FineScanConf,
}
impl Conf {
    #[allow(unused)]
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "Conf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let fast_scan = conf.get("fast-scan").expect(&format!("{dbg}.new | 'fast-scan' - not found or wrong configuration"));
        let fast_scan = FastScanConf::new(&name, fast_scan);
        log::trace!("{dbg}.new | fast-scan: {:#?}", fast_scan);
        let fine_scan = conf.get("fine-scan").expect(&format!("{dbg}.new | 'fine-scan' - not found or wrong configuration"));
        let fine_scan = FineScanConf::new(&name, fine_scan);
        log::trace!("{dbg}.new | fine-scan: {:#?}", fine_scan);
        let normalize = conf.get("normalize").expect(&format!("{dbg}.new | 'normalize' - not found or wrong configuration"));
        let normalize = NormalizeConf::new(&name, normalize);
        log::trace!("{dbg}.new | normalize: {:#?}", normalize);
        Self {
            normalize,
            fast_scan,
            fine_scan,
        }
    }
}
impl Default for Conf {
    fn default() -> Self {
        Self {
            normalize: Default::default(),
            fast_scan: Default::default(),
            fine_scan: Default::default(),
        }
    }
}}
mod normalize_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::{algorithm::CroppingConf, conf::GammaConf};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalizeConf {
    pub cropping: CroppingConf,
    pub gamma: GammaConf,
}
impl NormalizeConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "NormalizeConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let cropping = conf.get("cropping").expect(&format!("{dbg}.new | 'cropping' - not found or wrong configuration"));
        let cropping = CroppingConf::new(&name, cropping);
        log::trace!("{dbg}.new | cropping: {:#?}", cropping);
        let gamma = conf.get("gamma").expect(&format!("{dbg}.new | 'gamma' - not found or wrong configuration"));
        let gamma = GammaConf::new(&name, gamma);
        log::trace!("{dbg}.new | gamma: {:#?}", gamma);
        Self {
            cropping,
            gamma,
        }
    }
}
impl Default for NormalizeConf {
    fn default() -> Self {
        Self {
            cropping: CroppingConf::default(),
            gamma: GammaConf::default(),
        }
    }
}
}
mod union_conf {
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfTree, ConfTreeGet}, entity::Name};
use crate::conf::{AddWeightedConf, BitwiseAndConf};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnionConf {
    pub add_weighted: Option<AddWeightedConf>,
    pub bitwise_and: Option<BitwiseAndConf>,
}
impl UnionConf {
    pub fn new(parent: impl Into<String>, conf: ConfTree) -> Self {
        let parent = parent.into();
        let me = "UnionConf";
        let dbg = Dbg::new(&parent, me);
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let add_weighted = conf.get("add-weighted").map(|conf| AddWeightedConf::new(&name, conf));
        log::trace!("{dbg}.new | add-weighted: {:#?}", add_weighted);
        let bitwise_and = conf.get("bitwise-and").map(|conf| BitwiseAndConf::new(&name, conf));
        log::trace!("{dbg}.new | bitwise-and: {:#?}", bitwise_and);
        match (add_weighted, bitwise_and) {
            (None, None) => panic!("{dbg}.new | One of 'add-weighted' / `bitwise-and` - have to be specified"),
            (Some(_), Some(_)) => panic!("{dbg}.new | Both: 'add-weighted' and `bitwise-and` - are specified, please use one of"),
            _ => {},
        }
        Self {
            add_weighted,
            bitwise_and,
        }
    }
}
impl Default for UnionConf {
    fn default() -> Self {
        Self {
            add_weighted: None,
            bitwise_and: Some(BitwiseAndConf::default()),
        }
    }
}
}
pub use detecting_contouts::*;
pub use add_wighted_conf::*;
pub use bitwise_and_conf::*;
pub use conf::*;
pub use normalize_conf::*;
pub use union_conf::*;
}
mod domain {
mod eval {
mod eval {
pub trait Eval<In, Out> {
    fn eval(&self, val: In) -> Out;
}
#[allow(unused)]
pub trait EvalMut<In, Out> {
    fn eval(&mut self, val: In) -> Out;
}
}
pub use eval::*;
}
mod filter {
mod filter {
use std::marker::PhantomData;
pub trait Filter: std::fmt::Debug {
    type Item;
    fn add(&mut self, value: Self::Item) -> Option<Self::Item>;
}
#[derive(Debug, Clone)]
pub struct FilterEmpty<T> {
    item: PhantomData<T>
}
impl<T> FilterEmpty<T> {
    pub fn new() -> Self {
        Self {
            item: PhantomData,
        }
    }
}
impl<T: Copy + std::fmt::Debug + std::cmp::PartialEq> Filter for FilterEmpty<T> {
    type Item = T;
    fn add(&mut self, value: Self::Item) -> Option<T> {
        Some(value)
    }
}}
mod filter_lowpass {
use circular_buffer::CircularBuffer;
use super::filter::Filter;
#[derive(Debug, Clone)]
pub struct FilterLowPass<const N: usize, T> {
    buffer: CircularBuffer<N, T>,
}
impl<T: Copy, const N: usize> FilterLowPass<N, T> {
    #[allow(unused)]
    pub fn new(initial: Option<T>) -> Self {
        let mut buffer = CircularBuffer::<N, T>::new();
        initial.map(|initial| {
            buffer.push_back(initial);
            initial
        });
        Self {
            buffer,
        }
    }
}
impl<const N: usize> Filter for FilterLowPass<N, i32> {
    type Item = i32;
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        let sum = self.buffer.iter().sum::<i32>() + value;
        let average = sum / ((self.buffer.len() as i32) + 1);
        self.buffer.push_back(average);
        match self.buffer.front() {
            Some(v) => Some(*v),
            None => None,
        }
    }
}
}
mod filter_smooth {
use super::filter::Filter;
#[derive(Debug, Clone)]
pub struct FilterSmooth<T> {
    prev: Option<T>,
    factor_inv: f64,
}
impl<T: Copy> FilterSmooth<T> {
    #[allow(unused)]
    pub fn new(initial: Option<T>, factor: f64) -> Self {
        Self {
            prev: initial,
            factor_inv: 1.0 / factor,
        }
    }
}
impl Filter for FilterSmooth<i32> {
    type Item = i32;
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                let value = (prev as f64 + ((value as f64) - (prev as f64)) * self.factor_inv).round() as i32;
                self.prev.replace(value);
                Some(value)
            }
            None => {
                self.prev.replace(value);
                Some(value)
            }
        }
    }
}
}
mod filter_smooth_2 {
use super::filter::Filter;
#[derive(Debug, Clone)]
pub struct FilterSmooth2<T> {
    prev: Option<T>,
    factor: f64,
}
impl<T: Copy> FilterSmooth2<T> {
    #[allow(unused)]
    pub fn new(initial: Option<T>, factor: f64) -> Self {
        Self {
            prev: initial,
            factor,
        }
    }
}
impl Filter for FilterSmooth2<u8> {
    type Item = u8;
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                let delta = value as f64 - prev as f64;
                let factor = (1.0 + delta.abs() / value as f64) * self.factor;
                let value = (prev as f64 + delta / factor).round() as u8;
                self.prev.replace(value);
                Some(value)
            }
            None => {
                self.prev.replace(value);
                Some(value)
            }
        }
    }
}
impl Filter for FilterSmooth2<i32> {
    type Item = i32;
    fn add(&mut self, value: Self::Item) -> Option<Self::Item> {
        match self.prev {
            Some(prev) => {
                let delta = value as f64 - prev as f64;
                let factor = (1.0 + delta.abs() / value as f64) * self.factor;
                let value = (prev as f64 + delta / factor).round() as i32;
                self.prev.replace(value);
                Some(value)
            }
            None => {
                self.prev.replace(value);
                Some(value)
            }
        }
    }
}
}
pub use filter::*;
#[allow(unused)]
pub use filter_lowpass::*;
#[allow(unused)]
pub use filter_smooth::*;
pub use filter_smooth_2::*;
}
mod types {
mod channel {
pub type Sender<T> = sal_sync::sync::channel::Sender<T>;
pub type Receiver<T> = sal_sync::sync::channel::Receiver<T>;
pub type RecvTimeoutError = sal_sync::sync::channel::RecvTimeoutError;
pub type SendError = sal_sync::sync::channel::SendError;
pub fn channel_bounded<T>(size: usize) -> (Sender<T>, Receiver<T>) {
    sal_sync::sync::channel::bounded(size)
}
pub fn channel_unbounded<T>() -> (Sender<T>, Receiver<T>) {
    sal_sync::sync::channel::unbounded()
}}
mod error {
pub type Error = sal_core::error::Error;}
mod sync {
pub type RwLock<T> = parking_lot::RwLock<T>;
pub type Mutex<T> = parking_lot::Mutex<T>;
}
pub use channel::*;
pub use error::*;
pub use sync::*;
}
mod color {
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black,
    Navy,
    DarkBlue,
    MediumBlue,
    Blue,
    DarkGreen,
    Green,
    Teal,
    DarkCyan,
    DeepSkyBlue,
    DarkTurquoise,
    MediumSpringGreen,
    Lime,
    SpringGreen,
    Aqua,
    Cyan,
    MidnightBlue,
    DodgerBlue,
    LightSeaGreen,
    ForestGreen,
    SeaGreen,
    DarkSlateGray,
    DarkSlateGrey,
    LimeGreen,
    MediumSeaGreen,
    Turquoise,
    RoyalBlue,
    SteelBlue,
    DarkSlateBlue,
    MediumTurquoise,
    Indigo,
    DarkOliveGreen,
    CadetBlue,
    CornflowerBlue,
    RebeccaPurple,
    MediumAquaMarine,
    DimGray,
    DimGrey,
    SlateBlue,
    OliveDrab,
    SlateGray,
    SlateGrey,
    LightSlateGray,
    LightSlateGrey,
    MediumSlateBlue,
    LawnGreen,
    Chartreuse,
    Aquamarine,
    Maroon,
    Purple,
    Olive,
    Gray,
    Grey,
    SkyBlue,
    LightSkyBlue,
    BlueViolet,
    DarkRed,
    DarkMagenta,
    SaddleBrown,
    DarkSeaGreen,
    LightGreen,
    MediumPurple,
    DarkViolet,
    PaleGreen,
    DarkOrchid,
    YellowGreen,
    Sienna,
    Brown,
    DarkGray,
    DarkGrey,
    LightBlue,
    GreenYellow,
    PaleTurquoise,
    LightSteelBlue,
    PowderBlue,
    FireBrick,
    DarkGoldenRod,
    MediumOrchid,
    RosyBrown,
    DarkKhaki,
    Silver,
    MediumVioletRed,
    IndianRed,
    Peru,
    Chocolate,
    Tan,
    LightGray,
    LightGrey,
    Thistle,
    Orchid,
    GoldenRod,
    PaleVioletRed,
    Crimson,
    Gainsboro,
    Plum,
    BurlyWood,
    LightCyan,
    Lavender,
    DarkSalmon,
    Violet,
    PaleGoldenRod,
    LightCoral,
    Khaki,
    AliceBlue,
    HoneyDew,
    Azure,
    SandyBrown,
    Wheat,
    Beige,
    WhiteSmoke,
    MintCream,
    GhostWhite,
    Salmon,
    AntiqueWhite,
    Linen,
    LightGoldenRodYellow,
    OldLace,
    Red,
    Fuchsia,
    Magenta,
    DeepPink,
    OrangeRed,
    Tomato,
    HotPink,
    Coral,
    DarkOrange,
    LightSalmon,
    Orange,
    LightPink,
    Pink,
    Gold,
    PeachPuff,
    NavajoWhite,
    Moccasin,
    Bisque,
    MistyRose,
    BlanchedAlmond,
    PapayaWhip,
    LavenderBlush,
    SeaShell,
    Cornsilk,
    LemonChiffon,
    FloralWhite,
    Snow,
    Yellow,
    LightYellow,
}
impl Color {
    #[allow(unused)]
    fn hex(&self) -> usize {
        match self {
            Self::Black => 0x000000,
            Self::Navy => 0x000080,
            Self::DarkBlue => 0x00008B,
            Self::MediumBlue => 0x0000CD,
            Self::Blue => 0x0000FF,
            Self::DarkGreen => 0x006400,
            Self::Green => 0x008000,
            Self::Teal => 0x008080,
            Self::DarkCyan => 0x008B8B,
            Self::DeepSkyBlue => 0x00BFFF,
            Self::DarkTurquoise => 0x00CED1,
            Self::MediumSpringGreen => 0x00FA9A,
            Self::Lime => 0x00FF00,
            Self::SpringGreen => 0x00FF7F,
            Self::Aqua => 0x00FFFF,
            Self::Cyan => 0x00FFFF,
            Self::MidnightBlue => 0x191970,
            Self::DodgerBlue => 0x1E90FF,
            Self::LightSeaGreen => 0x20B2AA,
            Self::ForestGreen => 0x228B22,
            Self::SeaGreen => 0x2E8B57,
            Self::DarkSlateGray => 0x2F4F4F,
            Self::DarkSlateGrey => 0x2F4F4F,
            Self::LimeGreen => 0x32CD32,
            Self::MediumSeaGreen => 0x3CB371,
            Self::Turquoise => 0x40E0D0,
            Self::RoyalBlue => 0x4169E1,
            Self::SteelBlue => 0x4682B4,
            Self::DarkSlateBlue => 0x483D8B,
            Self::MediumTurquoise => 0x48D1CC,
            Self::Indigo => 0x4B0082,
            Self::DarkOliveGreen => 0x556B2F,
            Self::CadetBlue => 0x5F9EA0,
            Self::CornflowerBlue => 0x6495ED,
            Self::RebeccaPurple => 0x663399,
            Self::MediumAquaMarine => 0x66CDAA,
            Self::DimGray => 0x696969,
            Self::DimGrey => 0x696969,
            Self::SlateBlue => 0x6A5ACD,
            Self::OliveDrab => 0x6B8E23,
            Self::SlateGray => 0x708090,
            Self::SlateGrey => 0x708090,
            Self::LightSlateGray => 0x778899,
            Self::LightSlateGrey => 0x778899,
            Self::MediumSlateBlue => 0x7B68EE,
            Self::LawnGreen => 0x7CFC00,
            Self::Chartreuse => 0x7FFF00,
            Self::Aquamarine => 0x7FFFD4,
            Self::Maroon => 0x800000,
            Self::Purple => 0x800080,
            Self::Olive => 0x808000,
            Self::Gray => 0x808080,
            Self::Grey => 0x808080,
            Self::SkyBlue => 0x87CEEB,
            Self::LightSkyBlue => 0x87CEFA,
            Self::BlueViolet => 0x8A2BE2,
            Self::DarkRed => 0x8B0000,
            Self::DarkMagenta => 0x8B008B,
            Self::SaddleBrown => 0x8B4513,
            Self::DarkSeaGreen => 0x8FBC8F,
            Self::LightGreen => 0x90EE90,
            Self::MediumPurple => 0x9370DB,
            Self::DarkViolet => 0x9400D3,
            Self::PaleGreen => 0x98FB98,
            Self::DarkOrchid => 0x9932CC,
            Self::YellowGreen => 0x9ACD32,
            Self::Sienna => 0xA0522D,
            Self::Brown => 0xA52A2A,
            Self::DarkGray => 0xA9A9A9,
            Self::DarkGrey => 0xA9A9A9,
            Self::LightBlue => 0xADD8E6,
            Self::GreenYellow => 0xADFF2F,
            Self::PaleTurquoise => 0xAFEEEE,
            Self::LightSteelBlue => 0xB0C4DE,
            Self::PowderBlue => 0xB0E0E6,
            Self::FireBrick => 0xB22222,
            Self::DarkGoldenRod => 0xB8860B,
            Self::MediumOrchid => 0xBA55D3,
            Self::RosyBrown => 0xBC8F8F,
            Self::DarkKhaki => 0xBDB76B,
            Self::Silver => 0xC0C0C0,
            Self::MediumVioletRed => 0xC71585,
            Self::IndianRed => 0xCD5C5C,
            Self::Peru => 0xCD853F,
            Self::Chocolate => 0xD2691E,
            Self::Tan => 0xD2B48C,
            Self::LightGray => 0xD3D3D3,
            Self::LightGrey => 0xD3D3D3,
            Self::Thistle => 0xD8BFD8,
            Self::Orchid => 0xDA70D6,
            Self::GoldenRod => 0xDAA520,
            Self::PaleVioletRed => 0xDB7093,
            Self::Crimson => 0xDC143C,
            Self::Gainsboro => 0xDCDCDC,
            Self::Plum => 0xDDA0DD,
            Self::BurlyWood => 0xDEB887,
            Self::LightCyan => 0xE0FFFF,
            Self::Lavender => 0xE6E6FA,
            Self::DarkSalmon => 0xE9967A,
            Self::Violet => 0xEE82EE,
            Self::PaleGoldenRod => 0xEEE8AA,
            Self::LightCoral => 0xF08080,
            Self::Khaki => 0xF0E68C,
            Self::AliceBlue => 0xF0F8FF,
            Self::HoneyDew => 0xF0FFF0,
            Self::Azure => 0xF0FFFF,
            Self::SandyBrown => 0xF4A460,
            Self::Wheat => 0xF5DEB3,
            Self::Beige => 0xF5F5DC,
            Self::WhiteSmoke => 0xF5F5F5,
            Self::MintCream => 0xF5FFFA,
            Self::GhostWhite => 0xF8F8FF,
            Self::Salmon => 0xFA8072,
            Self::AntiqueWhite => 0xFAEBD7,
            Self::Linen => 0xFAF0E6,
            Self::LightGoldenRodYellow => 0xFAFAD2,
            Self::OldLace => 0xFDF5E6,
            Self::Red => 0xFF0000,
            Self::Fuchsia => 0xFF00FF,
            Self::Magenta => 0xFF00FF,
            Self::DeepPink => 0xFF1493,
            Self::OrangeRed => 0xFF4500,
            Self::Tomato => 0xFF6347,
            Self::HotPink => 0xFF69B4,
            Self::Coral => 0xFF7F50,
            Self::DarkOrange => 0xFF8C00,
            Self::LightSalmon => 0xFFA07A,
            Self::Orange => 0xFFA500,
            Self::LightPink => 0xFFB6C1,
            Self::Pink => 0xFFC0CB,
            Self::Gold => 0xFFD700,
            Self::PeachPuff => 0xFFDAB9,
            Self::NavajoWhite => 0xFFDEAD,
            Self::Moccasin => 0xFFE4B5,
            Self::Bisque => 0xFFE4C4,
            Self::MistyRose => 0xFFE4E1,
            Self::BlanchedAlmond => 0xFFEBCD,
            Self::PapayaWhip => 0xFFEFD5,
            Self::LavenderBlush => 0xFFF0F5,
            Self::SeaShell => 0xFFF5EE,
            Self::Cornsilk => 0xFFF8DC,
            Self::LemonChiffon => 0xFFFACD,
            Self::FloralWhite => 0xFFFAF0,
            Self::Snow => 0xFFFAFA,
            Self::Yellow => 0xFFFF00,
            Self::LightYellow => 0xFFFFE0,
        }
    }
}
#[allow(unused)]
pub trait ColorProps<T> {
    fn r(&self) -> T;
    fn g(&self) -> T;
    fn b(&self) -> T;
    fn rgb(&self) -> [T; 3];
    fn bgr(&self) -> [T; 3];
    fn rgba(&self, alpha: T) -> [T; 4];
    fn bgra(&self, alpha: T) -> [T; 4];
}
impl ColorProps<u8> for Color {
    fn r(&self) -> u8 {
        self.rgb()[0]
    }
    fn g(&self) -> u8 {
        self.rgb()[1]
    }
    fn b(&self) -> u8 {
        self.rgb()[2]
    }
    fn rgb(&self) -> [u8; 3] {
        let bytes = u32::to_be_bytes(self.hex() as u32);
        [bytes[1], bytes[2], bytes[3]]
    }
    fn bgr(&self) -> [u8; 3] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0]]
    }
    fn rgba(&self, alpha: u8) -> [u8; 4] {
        let rgb = self.rgb();
        [rgb[0], rgb[1], rgb[2], alpha]
    }
    fn bgra(&self, alpha: u8) -> [u8; 4] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0], alpha]
    }
}
impl ColorProps<f64> for Color {
    fn r(&self) -> f64 {
        self.rgb()[0]
    }
    fn g(&self) -> f64 {
        self.rgb()[1]
    }
    fn b(&self) -> f64 {
        self.rgb()[2]
    }
    fn rgb(&self) -> [f64; 3] {
        let bytes = u32::to_be_bytes(self.hex() as u32);
        [bytes[1] as f64, bytes[2] as f64, bytes[3] as f64]
    }
    fn bgr(&self) -> [f64; 3] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0]]
    }
    fn rgba(&self, alpha: f64) -> [f64; 4] {
        let rgb = self.rgb();
        [rgb[0], rgb[1], rgb[2], alpha]
    }
    fn bgra(&self, alpha: f64) -> [f64; 4] {
        let rgb = self.rgb();
        [rgb[2], rgb[1], rgb[0], alpha]
    }
}
}
mod dot {
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot<T> {
    pub x: T,
    pub y: T,
}
impl From<&[usize; 2]> for Dot<usize> {
    fn from(dot: &[usize; 2]) -> Self {
        Dot { x: dot[0], y: dot[1] }
    }
}
impl From<[usize; 2]> for Dot<usize> {
    fn from(dot: [usize; 2]) -> Self {
        Dot { x: dot[0], y: dot[1] }
    }
}
}
mod image {
use opencv::{boxed_ref::BoxedRef, core::{Mat, MatTraitConst, MatTraitConstManual}};
use sal_core::error::Error;
#[derive(Debug, Clone)]
pub struct Image {
    pub mat: Mat,
    pub meta: usize,
}
impl Image {
    #[allow(unused)]
    pub fn new(
        mat: Mat,
        meta: usize,
    ) -> Self {
        Self {
            meta,
            mat,
        }
    }
    pub fn from(mat: Mat, meta: usize) -> Self {
        Self {
            mat,
            meta,
        }
    }
    pub fn width(&self) -> i32 {
        self.mat.cols()
    }
    pub fn height(&self) -> i32 {
        self.mat.rows()
    }
    pub fn size(&self) -> usize {
        self.mat.total() * self.mat.elem_size1()
    }
    #[allow(unused)]
    pub fn save(&self, path: impl Into<String>) -> Result<(), Error> {
        let error = Error::new("Image", "save");
        let path = path.into();
        let params = opencv::core::Vector::new();
        opencv::imgcodecs::imwrite(&path, &self.mat, &params)
            .map(|_| ())
            .map_err(|err| error.pass_with(format!("Errorsaving image into '{path}'"), err.to_string()))
    }
    #[allow(unused)]
    pub fn load(path: impl Into<String>, meta: usize) -> Result<Self, Error> {
        let error = Error::new("Image", "load");
        let path = path.into();
        opencv::imgcodecs::imread(&path, opencv::imgcodecs::IMREAD_UNCHANGED)
            .map(|mat| Image::from(mat, meta))
            .map_err(|err| error.pass_with(format!("Error saving image into '{path}'"), err.to_string()))
    }
    #[allow(unused)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let error = Error::new("Image", "to_bytes");
        bincode::encode_to_vec(self, bincode::config::standard())
            .map_err(|err| error.pass_with(format!("Error encoding image"), err.to_string()))
    }
    #[allow(unused)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let error = Error::new("Image", "from_bytes");
        let (img, _) = bincode::decode_from_slice(bytes, bincode::config::standard())
            .map_err(|err| error.pass_with(format!("Error decoding image"), err.to_string()))?;
        Ok(img)
    }
}
impl Default for Image {
    fn default() -> Self {
        let mat = Mat::default();
        Self {
            meta: 0,
            mat,
        }
    }
}
impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        let mut dst = self.mat.clone();
        opencv::core::compare(&self.mat, &other.mat, &mut dst, opencv::core::CmpTypes::CMP_EQ as i32).is_ok()
    }
}
impl bincode::Encode for Image {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.width(), encoder)?;
        bincode::Encode::encode(&self.height(), encoder)?;
        bincode::Encode::encode(&self.mat.channels(), encoder)?;
        bincode::Encode::encode(&self.mat.typ(), encoder)?;
        bincode::Encode::encode(&self.meta, encoder)?;
        let mat = self.mat.data_bytes()
            .map_err(|err| bincode::error::EncodeError::OtherString(format!("Image.encode | Get bytes of Mat error: {:?}", err)))?;
        bincode::Encode::encode(mat, encoder)?;
        Ok(())
    }
}
impl<Context> bincode::Decode<Context> for Image {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let width: i32 = bincode::Decode::decode(decoder).unwrap();
        log::trace!("Image.decode | width: {}", width);
        let height: i32 = bincode::Decode::decode(decoder).unwrap();
        log::trace!("Image.decode | height: {}", height);
        let channels: i32 = bincode::Decode::decode(decoder).unwrap();
        log::trace!("Image.decode | channels: {}", channels);
        let typ: i32 = bincode::Decode::decode(decoder).unwrap();
        log::trace!("Image.decode | typ: {}", typ);
        let meta = bincode::Decode::decode(decoder).unwrap();
        let data: Vec<u8> = bincode::Decode::decode(decoder).unwrap();
        let mat = mat_from_bytes(typ, channels, height as i32, width as i32, &data)
            .map_err(|err| bincode::error::DecodeError::OtherString(format!("Image.decode | Mat from bytes error: {:?}", err)))?;
        Ok(Self {
            meta,
            mat: mat.clone_pointee(),
        })
    }
}
impl<'de, Context> bincode::BorrowDecode<'de, Context> for Image {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let width: i32 = bincode::BorrowDecode::borrow_decode(decoder).unwrap();
        log::trace!("Image.borrow_decode | width: {}", width);
        let height: i32 = bincode::BorrowDecode::borrow_decode(decoder).unwrap();
        log::trace!("Image.borrow_decode | height: {}", height);
        let channels: i32 = bincode::BorrowDecode::borrow_decode(decoder).unwrap();
        log::trace!("Image.borrow_decode | channels: {}", channels);
        let typ: i32 = bincode::BorrowDecode::borrow_decode(decoder).unwrap();
        log::trace!("Image.borrow_decode | typ: {}", typ);
        let meta = bincode::BorrowDecode::borrow_decode(decoder).unwrap();
        let data: Vec<u8> = bincode::BorrowDecode::borrow_decode(decoder).unwrap();
        let mat = mat_from_bytes(typ, channels, height, width, &data)
            .map_err(|err| bincode::error::DecodeError::OtherString(format!("Image.borrow_decode | Mat from bytes error: {:?}", err)))?;
        Ok(Self {
            meta,
            mat: mat.clone_pointee(),
        })
    }
}
fn mat_from_bytes(typ: i32, channels: i32, height: i32, width: i32, data: &'_ [u8]) -> Result<BoxedRef<'_, Mat>, opencv::Error> {
    match typ {
        opencv::core::CV_8UC1 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u8, 1>>(height as i32, width as i32, data),
        opencv::core::CV_8UC2 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u8, 2>>(height as i32, width as i32, data),
        opencv::core::CV_8UC3 => {
            log::trace!("Image.mat_from_bytes | typ: {} CV_8UC3", typ);
            Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u8, 3>>(height as i32, width as i32, data)
        }
        opencv::core::CV_8UC4 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u8, 4>>(height as i32, width as i32, data),
        opencv::core::CV_8SC1 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i8, 1>>(height as i32, width as i32, data),
        opencv::core::CV_8SC2 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i8, 2>>(height as i32, width as i32, data),
        opencv::core::CV_8SC3 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i8, 3>>(height as i32, width as i32, data),
        opencv::core::CV_8SC4 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i8, 4>>(height as i32, width as i32, data),
        opencv::core::CV_16UC1 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u16, 1>>(height as i32, width as i32, data),
        opencv::core::CV_16UC2 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u16, 2>>(height as i32, width as i32, data),
        opencv::core::CV_16UC3 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u16, 3>>(height as i32, width as i32, data),
        opencv::core::CV_16UC4 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<u16, 4>>(height as i32, width as i32, data),
        opencv::core::CV_16SC1 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i16, 1>>(height as i32, width as i32, data),
        opencv::core::CV_16SC2 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i16, 2>>(height as i32, width as i32, data),
        opencv::core::CV_16SC3 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i16, 3>>(height as i32, width as i32, data),
        opencv::core::CV_16SC4 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i16, 4>>(height as i32, width as i32, data),
        opencv::core::CV_32SC1 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i32, 1>>(height as i32, width as i32, data),
        opencv::core::CV_32SC2 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i32, 2>>(height as i32, width as i32, data),
        opencv::core::CV_32SC3 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i32, 3>>(height as i32, width as i32, data),
        opencv::core::CV_32SC4 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<i32, 4>>(height as i32, width as i32, data),
        opencv::core::CV_32FC1 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f32, 1>>(height as i32, width as i32, data),
        opencv::core::CV_32FC2 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f32, 2>>(height as i32, width as i32, data),
        opencv::core::CV_32FC3 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f32, 3>>(height as i32, width as i32, data),
        opencv::core::CV_32FC4 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f32, 4>>(height as i32, width as i32, data),
        opencv::core::CV_64FC1 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f64, 1>>(height as i32, width as i32, data),
        opencv::core::CV_64FC2 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f64, 2>>(height as i32, width as i32, data),
        opencv::core::CV_64FC3 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f64, 3>>(height as i32, width as i32, data),
        opencv::core::CV_64FC4 => Mat::new_rows_cols_with_bytes::<opencv::core::VecN<f64, 4>>(height as i32, width as i32, data),
        _ => Err(opencv::Error::new(opencv::core::BadOrigin, format!("Image.decode | Unsupported image format: pixel depth {typ}, channels {channels}"))),
    }
}}
pub use eval::*;
pub(crate) use filter::*;
pub(crate) use types::*;
pub use color::*;
pub use dot::*;
pub use image::*;
}
mod infrostructure {
pub mod camera {
mod camera_resolution {
use sal_sync::services::conf::ConfTree;
use serde::Deserialize;
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct CameraResolution {
    pub width: usize,
    pub height: usize,
}
impl CameraResolution{
    pub fn new(parent: impl Into<String>, conf: &ConfTree) -> Self {
        log::trace!("{}/CameraConf.new | conf_tree: {:?}", parent.into(), conf);
        serde_yaml::from_value(conf.conf.clone()).unwrap()
    }
}
impl Default for CameraResolution {
    fn default() -> Self {
        Self { width: Default::default(), height: Default::default() }
    }
}}
mod camera_conf {
use std::{fs, net::SocketAddr, str::FromStr};
use sal_core::dbg::Dbg;
use sal_sync::services::{conf::{ConfCustomKeywd, ConfTree, ConfTreeGet}, entity::Name};
use crate::infrostructure::arena::{ChannelPacketSize, Exposure, FrameRate, PixelFormat};
use super::camera_resolution::CameraResolution;
#[derive(Debug, Clone, PartialEq)]
pub struct CameraConf {
    pub name: Name,
    pub fps: FrameRate,
    pub resolution: CameraResolution,
    pub index: Option<usize>,
    pub address: Option<SocketAddr>,
    pub pixel_format: PixelFormat,
    pub exposure: Exposure,
    pub auto_packet_size: bool,
    pub channel_packet_size: ChannelPacketSize,
    pub resend_packet: bool,
    pub from_path: Option<String>,
}
impl CameraConf {
    pub fn new(parent: impl Into<String>, conf: &ConfTree) -> Self {
        let parent = parent.into();
        let conf_keywd = ConfCustomKeywd::from_str(&conf.key).unwrap();
        log::trace!("{}.new | conf.name: {:?}", "CameraConf", conf_keywd.name());
        let me = match conf_keywd.title().is_empty() {
            true => conf_keywd.name(),
            false => conf_keywd.title(),
        };
        let dbg = Dbg::new(&parent, format!("CameraConf({})", me));
        log::trace!("{}.new | conf: {:?}", dbg, conf);
        let name = Name::new(parent, me);
        log::trace!("{}.new | name: {:?}", dbg, name);
        let from_path: Option<String> = conf.get("from-path");
        log::trace!("{}.new | from-path: {:?}", dbg, from_path);
        match from_path {
            Some(_) => {
                Self {
                    name,
                    fps: FrameRate::Min,
                    resolution: CameraResolution::default(),
                    index: None,
                    address: None,
                    pixel_format: PixelFormat::Mono8,
                    exposure: Exposure::default(),
                    auto_packet_size: false,
                    channel_packet_size: ChannelPacketSize::Min,
                    resend_packet: false,
                    from_path,
                }
            }
            None => {
                let fps = conf.get("fps").unwrap();
                let fps: FrameRate = serde_yaml::from_value(fps).unwrap();
                log::trace!("{}.new | fps: {:?}", dbg, fps);
                let resolution = conf.get("resolution").unwrap();
                let resolution = CameraResolution::new(name.join(), &resolution);
                log::trace!("{}.new | resolution: {:?}", dbg, resolution);
                let index = conf.get("index").map(|ix: u64| ix as usize);
                log::trace!("{}.new | index: {:?}", dbg, index);
                let address: Option<SocketAddr> = conf.get("address").map(|addr: String| addr.parse().unwrap());
                log::trace!("{}.new | address: {:?}", dbg, address);
                let pixel_format = conf.get("pixel-format").unwrap();
                let pixel_format: PixelFormat = serde_yaml::from_value(pixel_format).unwrap();
                log::trace!("{}.new | pixel-format: {:?}", dbg, pixel_format);
                let exposure = conf.get("exposure").unwrap();
                let exposure: Exposure = serde_yaml::from_value(exposure).unwrap();
                log::trace!("{}.new | exposure: {:?}", dbg, exposure);
                let auto_packet_size = conf.get("auto-packet-size").unwrap();
                log::trace!("{}.new | auto-packet-size: {:?}", dbg, auto_packet_size);
                let channel_packet_size = conf.get("channel-packet-size").unwrap();
                let channel_packet_size: ChannelPacketSize = serde_yaml::from_value(channel_packet_size).unwrap();
                log::trace!("{}.new | channel-packet-size: {:?}", dbg, channel_packet_size);
                let resend_packet = conf.get("resend-packet").unwrap();
                log::trace!("{}.new | resend-packet: {:?}", dbg, resend_packet);
                Self {
                    name,
                    fps,
                    resolution,
                    index,
                    address,
                    pixel_format,
                    exposure,
                    auto_packet_size,
                    channel_packet_size,
                    resend_packet,
                    from_path,
                }
            }
        }
    }
    pub fn from_yaml(parent: impl Into<String>, value: &serde_yaml::Value) -> CameraConf {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(parent, &ConfTree::new(key.as_str().unwrap(), value.clone()))
            }
            None => {
                panic!("CameraConf.from_yaml | Format error or empty conf: {:#?}", value)
            }
        }
    }
    #[allow(dead_code)]
    pub fn read(parent: impl Into<String>, path: &str) -> CameraConf {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        CameraConf::from_yaml(parent, &config)
                    }
                    Err(err) => {
                        panic!("CameraConf.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    }
                }
            }
            Err(err) => {
                panic!("CameraConf.read | File {} reading error: {:?}", path, err)
            }
        }
    }
}}
mod camera {
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::JoinHandle, time::Duration};
use opencv::videoio::VideoCaptureTrait;
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::{services::entity::{Name, Object}, sync::AtomicUsizeOption};
use crate::{domain::{channel_unbounded, Receiver, Sender, Image}, infrostructure::arena::{AcDevice, AcSystem}};
use super::camera_conf::CameraConf;
pub struct Camera {
    dbg: Dbg,
    name: Name,
    conf: CameraConf,
    send: Sender<Image>,
    recv: Option<Receiver<Image>>,
    suspend: Arc<AtomicBool>,
    meta: Arc<AtomicUsizeOption>,
    exit: Arc<AtomicBool>,
}
impl Camera {
    pub fn new(meta: Arc<AtomicUsizeOption>, conf: CameraConf) -> Self {
        let dbg = Dbg::new(conf.name.parent(), conf.name.me());
        log::trace!("{}.new | : ", dbg);
        let (send, recv) = channel_unbounded();
        Self {
            dbg,
            name: conf.name.clone(),
            meta,
            conf,
            send,
            recv: Some(recv),
            suspend: Arc::new(AtomicBool::new(false)),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn stream(&mut self) -> Receiver<Image> {
        match self.recv.take() {
            Some(recv) => recv,
            None => {
                panic!("{}.stream | Receiver can be returned only once", self.name);
            },
        }
    }
    pub fn read(&self) -> Result<JoinHandle<()>, Error> {
        let dbg = self.dbg.clone();
        let conf = self.conf.clone();
        let send = self.send.clone();
        let suspend = self.suspend.clone();
        let meta = self.meta.clone();
        let exit = self.exit.clone();
        let handle = std::thread::spawn(move || {
            log::info!("{}.read | Start", dbg);
            loop {
                let mut ac_system = AcSystem::new(&dbg);
                match ac_system.run() {
                    Ok(_) => {
                        match ac_system.devices() {
                            Some(devices) => {
                                if devices > 0 {
                                    log::debug!("{}.read | Devices found: {}", dbg, devices);
                                    for dev in 0..devices {
                                        let device_vendor = ac_system.device_vendor(dev).unwrap();
                                        let device_model = ac_system.device_model(dev).unwrap();
                                        log::trace!("{}.read | Device {} model: {}", dbg, dev, device_model);
                                        let device_serial = ac_system.device_serial(dev).unwrap();
                                        log::trace!("{}.read | Device {} serial: {}", dbg, dev, device_serial);
                                        let device_mac = ac_system.device_mac(dev).unwrap();
                                        log::trace!("{}.read | Device {} MAC: {}", dbg, dev, device_mac);
                                        let device_ip = ac_system.device_ip(dev).unwrap();
                                        log::trace!("{}.read | Device {} IP: {}", dbg, dev, device_ip);
                                        let device_firmware = ac_system.device_firmware(dev).unwrap();
                                        log::trace!("{}.read | Device {} Firmware: {}", dbg, dev, device_firmware);
                                        log::debug!(
                                            "{}.read | Device {}: {:?} | {:?} | {:?} | {:?} | {:?} | {:?}",
                                            dbg, dev, device_vendor, device_model, device_serial, device_mac, device_ip, device_firmware);
                                    }
                                    match &conf.index {
                                        Some(index) => {
                                            if devices >= index + 1 {
                                                let meta_ = meta.clone();
                                                let mut device = AcDevice::new(
                                                    &dbg,
                                                    ac_system.system,
                                                    *index,
                                                    conf.clone(),
                                                    Some(suspend.clone()),
                                                    meta_,
                                                    Some(exit.clone()),
                                                );
                                                let result = device.listen(|frame| {
                                                    if let Err(err) = send.send(frame) {
                                                        log::warn!("{}.read | Send Error: {}", dbg, err);
                                                    }
                                                });
                                                if let Err(err) = result {
                                                    log::warn!("{}.read | Error: {}", dbg, err);
                                                }
                                            } else {
                                                log::warn!("{}.read | Specified device index '{}' out of found devices count '{}'", dbg, index, devices);
                                            }
                                        }
                                        None => {
                                            log::error!("{}.read | Device index - is not specified in the camera conf", dbg);
                                        }
                                    }
                                } else {
                                    log::warn!("{}.read | No devices detected on current network interface", dbg);
                                }
                            }
                            None => {
                                log::warn!("{}.read | No devices detected, Possible AcSystem is not executed first", dbg);
                            }
                        }
                    }
                    Err(err) => {
                        log::warn!("{}.read | Error: {}", dbg, err);
                    }
                }
                std::thread::sleep(Duration::from_secs(1));
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("{}.read | Exit", dbg);
        });
        Ok(handle)
    }
    #[allow(unused)]
    pub fn from_video(&self, path: impl Into<String>) -> Result<CameraIntoIterator, Error> {
        match opencv::videoio::VideoCapture::from_file(&path.into(), opencv::videoio::CAP_ANY) {
            Ok(mut video) => {
                let mut frames = vec![];
                let mut frame = opencv::core::Mat::default();
                let mut meta = 0;
                while let Ok(result) = video.read(&mut frame) {
                    if result {
                        frames.push(Image::from(frame.clone(), meta));
                        meta += 1;
                    } else {
                        break;
                    }
                }
                Ok(CameraIntoIterator { frames })
            }
            Err(err) => Err(Error::new(&self.dbg, "from_video").err(err.to_string())),
        }
    }
    #[allow(unused)]
    pub fn from_images(&self, path: impl Into<String>) -> Result<CameraIntoIterator, Error> {
        let mut frames = vec![];
        let mut meta = 0;
        match std::fs::read_dir(path.into()) {
            Ok(paths) => {
                for path in paths {
                    match path {
                        Ok(path) => {
                            if path.path().is_file() {
                                let path = path.path();
                                let path = path.to_str().ok_or(Error::new(&self.dbg, "from_images").err(format!("Error in path {}", path.display())))?;
                                match Image::load(path, meta) {
                                    Ok(img) => {
                                        frames.push(img);
                                    }
                                    Err(err) => return Err(Error::new(&self.dbg, "from_images").pass(err.to_string())),
                                }
                                meta += 1;
                            }
                        }
                        Err(err) => return Err(Error::new(&self.dbg, "from_images").pass(err.to_string())),
                    }
                }
            }
            Err(err) => return Err(Error::new(&self.dbg, "from_images").pass(err.to_string())),
        }
        Ok(CameraIntoIterator { frames })
    }
    pub fn suspend(&self) {
        self.suspend.store(true, Ordering::Release);
        log::debug!("{}.suspend | Suspension mode: ON", self.dbg);
    }
    pub fn resume(&self) {
        log::debug!("{}.resume | Suspension mode: OFF", self.dbg);
        self.suspend.store(false, Ordering::Release);
    }
    #[allow(unused)]
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
impl Object for Camera {
    fn name(&self) -> Name {
        self.name.clone()
    }
}
pub struct CameraIntoIterator {
    frames: Vec<Image>,
}
impl CameraIntoIterator {
    #[allow(unused)]
    pub fn push_frame(&mut self, frame: Image) {
        self.frames.push(frame);
    }
    fn pop_first(&mut self) -> Option<Image> {
        if self.frames.is_empty() {
            None
        } else {
            Some(self.frames.remove(0))
        }
    }
}
impl IntoIterator for Camera {
    type Item = Image;
    type IntoIter = CameraIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        CameraIntoIterator {
            frames: vec![]
        }
    }
}
impl Iterator for CameraIntoIterator {
    type Item = Image;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_first()
    }
}}
pub use camera_resolution::*;
pub use camera_conf::*;
pub use camera::*;
}
}
use debugging::session::{DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use crate::{
    algorithm::{
        AutoGamma, Cropping, FastContours, FastEdges, FastScanConf, FastScanCtx, FineScanConf, RopeDefect, Gray, Initial, InitialCtx, Mad, TemporalFilter, RopeDistortions
    }, conf::{Conf, NormalizeConf}, domain::Eval, infrostructure::camera::{Camera, CameraConf}
};

fn main() {
    DebugSession::new().filter(LogLevel::Debug).init().unwrap();
    let dbg = Dbg::own("main");
    let path = "./config.yaml";
    let conf = CameraConf::read(&dbg, path);
    let mut camera = Camera::new(Default::default(), conf);
    let recv = camera.stream();
    let handle = camera.read().unwrap();
    let window = "Retrived";
    if let Err(err) = opencv::highgui::named_window(window, opencv::highgui::WINDOW_NORMAL) {
        log::warn!("{}.stream | Create Window Error: {}", dbg, err);
    }
    opencv::highgui::wait_key(1).unwrap();
    let conf = Conf {
        normalize: NormalizeConf::default(),
        fast_scan: FastScanConf::default(),
        fine_scan: FineScanConf::default(),
    };
    let debug = false;
    let scan_rope = RopeDefect::<FastScanCtx>::new(
        conf.fast_scan.distortion_threshold,
        *Box::new(Mad::new()),
        RopeDistortions::<FastScanCtx>::new(
            conf.fast_scan.distortion_threshold,
            *Box::new(Mad::new()),
            FastEdges::new(
                conf.fast_scan.fast_edges.otsu_tune,
                conf.fast_scan.fast_edges.threshold,
                conf.fast_scan.fast_edges.smooth,
                FastContours::new(
                    conf.fast_scan.fast_contours.clone(),
                    TemporalFilter::<FastScanCtx>::new(
                        conf.fast_scan.temporal_filter.gaussian,
                        conf.fast_scan.temporal_filter.open_kernel,
                        conf.fast_scan.temporal_filter.erode_kernel,
                        conf.fast_scan.temporal_filter.threshold,
                        Gray::new(
                            AutoGamma::new(
                                conf.normalize.gamma.factor,
                                Cropping::new(
                                    conf.normalize.cropping.x,
                                    conf.normalize.cropping.width,
                                    conf.normalize.cropping.y,
                                    conf.normalize.cropping.height,
                                    Initial::new(
                                        InitialCtx::new(),
                                    ),
                                    debug,
                                ),
                                debug,
                            ),
                        ),
                        debug,
                    ),
                    debug,
                ),
            ),
        ),
    );
    for frame in recv {
        log::trace!("{dbg} | Frame width: {},  height: {}, timestamp: {}", frame.width(), frame.height(), frame.meta);
        if let Err(err) = opencv::highgui::imshow(window, &frame.mat) {
            log::warn!("{}.stream | Display img error: {:?}", dbg, err);
        };
        opencv::highgui::wait_key(1).unwrap();
        let result = scan_rope.eval(frame);
        _ = result;
    }
    handle.join().unwrap()
}
