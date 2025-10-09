use std::time::Instant;
use opencv::core::{Mat, MatTraitConst, MatTraitConstManual, Point2i, Size2i};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, FilterIsChanged, ResultCtx, TemporalFilterCtx}, domain::{Eval, Filter, Image, RwLock},
};
///
/// Temporal Filter | Highlighting / Hiding pixels depending on those changing speed
pub struct TemporalFilter {
    amplify_factor: f64,
    grow_speed: f64,
    reduce_factor: f64,
    down_speed: f64,
    threshold: f64,
    filters: RwLock<Vec<FilterIsChanged::<f32>>>,
    // background: RefCell<Mat>,
    ctx: Box<dyn Eval<Image, EvalResult> + Send + Sync>,
    debug: bool,
}
//
//
impl TemporalFilter {
    ///
    /// Returns [TemporalFilter] new instance
    /// - `cache` - path to the cache folder
    pub fn new(amplify_factor: f64, grow_speed: f64, reduce_factor: f64, down_speed: f64, threshold: f64, ctx: impl Eval<Image, EvalResult> + Send + Sync + 'static, debug: bool) -> Self {
        Self {
            amplify_factor,
            grow_speed,
            reduce_factor,
            down_speed,
            threshold,
            filters: RwLock::new(vec![]),
            ctx: Box::new(ctx),
            debug,
        }
    }
}
//
//
impl Eval<Image, EvalResult> for TemporalFilter {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("TemporalFilter", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx = ctx.read();
                let frame = &result.frame;
                match frame.mat.data_bytes() {
                    Ok(input) => {
                        let height = frame.mat.rows() as usize;
                        let width = frame.mat.cols() as usize;
                        let pixels = width * height * frame.mat.channels() as usize;
                        let mut out = vec![0u8; pixels];
                        log::debug!("TemporalFilter.eval | pixels: {:?}", pixels);
                        if self.filters.read().is_empty() {
                            *self.filters.write() = (0..pixels).map(|_| {
                                FilterIsChanged::<f32>::new(None, self.threshold)
                            }).collect();
                        }
                        log::debug!("TemporalFilter.eval | mat.typ: {:?}", frame.mat.typ());
                        log::debug!("TemporalFilter.eval | mat.channels: {:?}", frame.mat.channels());
                        {
                            let mut filters = self.filters.write();
                            for i in 0..pixels {
                                match input.get(i) {
                                    Some(value) => {
                                        if let Some(filter) = filters.get_mut(i) {
                                            match out.get_mut(i) {
                                                Some(pixel) => *pixel = match filter.add(*value as f32) {
                                                    Some(_) => 255,
                                                    None => 0,
                                                },
                                                None => return Err(error.err(format!("Out image format error, index [{i}] out of image range {width}x{height}={pixels}"))),
                                            }
                                        }
                                    }
                                    None => return Err(error.err(format!("Input image format error, index [{i}] out of image range {width}x{height}={pixels}"))),
                                }
                            }
                        }
                        let out = unsafe { Mat::new_rows_cols_with_data_unsafe(
                            height as i32,
                            width as i32,
                            opencv::core::CV_8UC1,
                            out.as_ptr() as *mut std::ffi::c_void,
                            opencv::core::Mat_AUTO_STEP,
                        ) }.map_err(|err| error.pass(err.to_string()))?;
                        let kernel = opencv::imgproc::get_structuring_element(opencv::imgproc::MORPH_ELLIPSE, Size2i::new(5, 5), Point2i::new(-1, -1)).unwrap();
                        let mut dst = Mat::default();
                        opencv::imgproc::morphology_ex(
                            &out,
                            &mut dst,
                            opencv::imgproc::MORPH_OPEN,
                            &kernel,
                            Point2i::new(-1, -1),
                            1,
                            opencv::core::BORDER_CONSTANT,
                            opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass(err.to_string()))?,
                        ).map_err(|err| error.pass(err.to_string()))?;
                        opencv::imgproc::morphology_ex(
                            &dst.clone(),
                            &mut dst,
                            opencv::imgproc::MORPH_ERODE,
                            &kernel,
                            Point2i::new(-1, -1),
                            1,
                            opencv::core::BORDER_CONSTANT,
                            opencv::imgproc::morphology_default_border_value().map_err(|err| error.pass(err.to_string()))?,
                        ).map_err(|err| error.pass(err.to_string()))?;
                        let frame = Image::with(dst);
                        let ctx = if self.debug {
                            let result = TemporalFilterCtx { frame: frame.clone() };
                            ctx.write(result).map_err(|err| error.pass(err))?
                        } else {
                            ctx
                        };
                        let result = ResultCtx { frame };
                        log::debug!("TemporalFilter.eval | Elapsed: {:?}", t.elapsed());
                        ctx.write(result)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
