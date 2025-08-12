use std::time::Instant;
use opencv::{core::{Mat, MatTraitConst, MatTraitConstManual}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, DetectingContoursCvCtx, EvalResult, InitialPoints},
    domain::{Dot, Eval, Filter, Image, FilterLowPass},
};
use super::edge_detection_ctx::EdgeDetectionCtx;
///
/// Take [Image]
/// Return vectors of [Dot] for upper and lower edges of rope
pub struct EdgeDetection {
    threshold: u8,
    ctx: Box<dyn Eval<Image, EvalResult>>,
}
//
//
impl EdgeDetection {
    ///
    /// Returns [EdgeDetection] new instance
    pub fn new(threshold: u8, ctx: impl Eval<Image, EvalResult> + 'static) -> Self {
        Self {
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
                let image = ContextRead::<DetectingContoursCvCtx>::read(&ctx).result.clone();
                let mut otsu = Mat::default();
                let threshold = (imgproc::threshold(&image.mat, &mut otsu, 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * 0.99).round()as u8;
                let rows = image.mat.rows();
                let cols = image.mat.cols();
                let mut upper_edge = Vec::with_capacity(cols as usize);
                let mut lower_edge = Vec::with_capacity(cols as usize);
                let mut filter_smooth_upper = FilterLowPass::<6, _>::new(None, 1.0);
                let mut filter_smooth_lower = FilterLowPass::<6, _>::new(None, 1.0);
                let mut upper;
                let mut lower;
                let mat = image.mat.data_bytes().unwrap();
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
