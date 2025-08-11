use opencv::{core::{Mat, MatTraitConst}, imgproc};
use sal_core::error::Error;
use crate::{
    algorithm::{ContextRead, ContextWrite, DetectingContoursCvCtx, EvalResult, InitialPoints},
    domain::{Dot, Eval, Filter, FilterSmooth, Image, FilterLowPass},
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
                let image = ContextRead::<DetectingContoursCvCtx>::read(&ctx).result.clone();
                let mut otsu = Mat::default();
                let threshold = (imgproc::threshold(&image.mat, &mut otsu, 0.0, 255.0, imgproc::THRESH_OTSU).unwrap() * 0.8).round()as u8;
                let rows = image.mat.rows();
                let cols = image.mat.cols();
                let mut upper_edge = Vec::new();
                let mut lower_edge = Vec::new();
                for x in 0..cols {
                    let mut filter_smooth = FilterLowPass::<2, _>::new(None, 1.0);
                    for y in 0..rows {
                        match image.mat.at_2d::<u8>(y, x) {
                            Ok(&pixel_value) => {
                                if pixel_value >= threshold {
                                    if let Some(y) = filter_smooth.add(y) {
                                        upper_edge.push(Dot {x: x as usize, y: y as usize});
                                        break;
                                    }
                                }
                            }   
                            Err(err) => {
                                return Err(error.pass_with("Input image format error", err.to_string()));
                            }
                        }
                    }
                    let mut filter_smooth = FilterLowPass::<2, _>::new(None, 1.0);
                    for y in (0..rows).rev() {
                        match image.mat.at_2d::<u8>(y, x) {
                            Ok(&pixel_value) => {
                                if pixel_value >= threshold {
                                    if let Some(y) = filter_smooth.add(y) {
                                        lower_edge.push(Dot {x: x as usize, y: y as usize});
                                        break;
                                    }
                                }
                            }
                            Err(err) => {
                                return Err(error.pass_with("Input image format error", err.to_string()));
                            }
                        }
                    }
                }
                let result = EdgeDetectionCtx {
                    result: InitialPoints::new(upper_edge, lower_edge),
                };
                ctx.write(result)
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
