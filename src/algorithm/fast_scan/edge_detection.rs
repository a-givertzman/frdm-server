use std::time::Instant;
use opencv::{core::{Mat, MatTraitConst, MatTraitConstManual}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, EvalResult, InitialPoints, ResultCtx},
    domain::{Dot, Eval, Filter, FilterSmooth, Image},
};
use super::edge_detection_ctx::EdgeDetectionCtx;
///
/// Take [Image]
/// Return vectors of [Dot] for upper and lower edges of rope
pub struct EdgeDetection {
    otsu_tune: Option<f64>,
    threshold: Option<u8>,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl EdgeDetection {
    ///
    /// Returns [EdgeDetection] new instance
    pub fn new(otsu_tune: Option<f64>, threshold: Option<u8>, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
            otsu_tune,
            threshold,
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<Image, EvalResult> for EdgeDetection {
    fn eval(&self, frame: Image) -> EvalResult {
        let error = Error::new("EdgeDetection", "eval");
        match self.ctx.eval(frame) {
            Ok(ctx) => {
                let t = Instant::now();
                let result: &ResultCtx = ctx.read();
                let frame = &result.frame;
                let threshold = match (self.otsu_tune, self.threshold) {
                    (None, None) => imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap().round() as u8,
                    (None, Some(threshold)) => threshold,
                    (Some(otsu_tune), None) => (imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * otsu_tune).round() as u8,
                    (Some(otsu_tune), Some(_)) => (imgproc::threshold(&frame.mat, &mut Mat::default(), 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * otsu_tune).round() as u8,
                };
                log::debug!("EdgeDetection.eval | threshold: {threshold}");
                let rows = frame.mat.rows();
                let cols = frame.mat.cols();
                let mut upper_edge = Vec::with_capacity(cols as usize);
                let mut lower_edge = Vec::with_capacity(cols as usize);
                let mut filter_smooth_upper = FilterSmooth::new(None, 24.0);
                let mut filter_smooth_lower = FilterSmooth::new(None, 24.0);
                let mut upper;
                let mut lower;
                let mat = frame.mat.data_bytes().unwrap();
                for x in 0..cols {
                    upper = false;
                    lower = false;
                    for y in 0..rows {
                        match mat.get((y * cols + x) as usize) {
                            Some(pixel_value) => {
                                if !upper && pixel_value >= &threshold {
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
                        let y = rows - y -1;
                        match mat.get((y * cols + x) as usize) {
                            Some(pixel_value) => {
                                if !lower && pixel_value >= &threshold {
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
                        if upper && lower {
                            break;
                        }
                    }
                }
                let result = EdgeDetectionCtx {
                    result: InitialPoints::new(upper_edge, lower_edge),
                };
                log::debug!("EdgeDetection.eval | Elapsed: {:?}", t.elapsed());
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
